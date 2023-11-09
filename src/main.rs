//!

use crate::config::load_config_from_file;
use crate::replacement::Replacements;
use clap::{Arg, ArgMatches, Command};
use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use mdbook::BookItem;
use semver::{Version, VersionReq};
use std::{io, process};

mod config;
mod replacement;
mod tests;

/// A `mdbook` preprocessor for simple replacement patterns.
struct Yapp {
  replacements: Replacements,
}

impl Yapp {
  ///
  pub fn new() -> Self {
    if let Some(replacements) = load_config_from_file() {
      Self { replacements }
    } else {
      process::exit(1);
    }
  }
}

impl Preprocessor for Yapp {
  ///
  fn name(&self) -> &str {
    "yapp-preprocessor"
  }

  ///
  fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
    book.for_each_mut(|book_item| {
      if let BookItem::Chapter(chapter) = book_item {
        chapter.content = self.replacements.replace(&chapter.content)
      }
    });
    Ok(book)
  }

  ///
  fn supports_renderer(&self, renderer: &str) -> bool {
    renderer != "not-supported"
  }
}

///
fn make_app() -> Command {
  Command::new("yapp-preprocessor").about("A mdbook preprocessor for simple replacement patterns").subcommand(
    Command::new("supports")
      .arg(Arg::new("renderer").required(true))
      .about("Check whether a renderer is supported by this preprocessor"),
  )
}

///
fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
  let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;
  let book_version = Version::parse(&ctx.mdbook_version)?;
  let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION)?;
  if !version_req.matches(&book_version) {
    eprintln!(
      "Warning: The {} plugin was built against version {} of mdbook, but is being called from version {}",
      pre.name(),
      mdbook::MDBOOK_VERSION,
      ctx.mdbook_version
    );
  }
  let processed_book = pre.run(&ctx, book)?;
  serde_json::to_writer(io::stdout(), &processed_book)?;
  Ok(())
}

///
fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> ! {
  let renderer = sub_args.get_one::<String>("renderer").expect("Required argument");
  let supported = pre.supports_renderer(renderer);
  if supported {
    process::exit(0);
  } else {
    process::exit(1);
  }
}

///
fn main() {
  let matches = make_app().get_matches();
  let preprocessor = Yapp::new();
  if let Some(sub_args) = matches.subcommand_matches("supports") {
    handle_supports(&preprocessor, sub_args);
  } else if let Err(e) = handle_preprocessing(&preprocessor) {
    eprintln!("{}", e);
    process::exit(1);
  }
}
