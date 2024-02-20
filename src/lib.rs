// use gray_matter::Matter;
// use mdbook::book::{Book, BookItem};
// use mdbook::errors::Error;
// use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
// use semver::{Version, VersionReq};
// use std::io;
// use std::process;
// use clap::{Arg, ArgMatches, Command};

use gray_matter::Matter;
use gray_matter::engine::YAML;
use mdbook::book::{Book, BookItem};
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use semver::{Version, VersionReq};
use std::io;
use std::process;
use clap::{Arg, ArgMatches, Command};
use serde::Deserialize;


pub fn make_app() -> Command<'static> {
    Command::new("frontmatter-preprocessor")
        .about("An mdbook preprocessor that handles front matter in markdown files")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}

fn main() {
    let matches = make_app().get_matches();

    let preprocessor = FrontMatterPreprocessor;

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&preprocessor, sub_args);
    } else if let Err(e) = handle_preprocessing(&preprocessor) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, mut book) = CmdPreprocessor::parse_input(io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> ! {
    let renderer = sub_args
        .get_one::<String>("renderer")
        .expect("Required argument");
    let supported = pre.supports_renderer(renderer);

    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

pub struct FrontMatterPreprocessor;

impl Preprocessor for FrontMatterPreprocessor {
    fn name(&self) -> &str {
        "frontmatter-preprocessor"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let matter = Matter::<YAML>::new();
        
        book.for_each_mut(|item: &mut BookItem| {
            if let BookItem::Chapter(ref mut chapter) = item {
                let parsed = matter.parse(&chapter.content);

                // Example usage: Print the title and tags if available
                if let Some(data) = parsed.data {
                    if let Ok(front_matter) = data.deserialize::<FrontMatter>() {
                        println!("Title: {}", front_matter.title);
                        println!("Tags: {:?}", front_matter.tags);

                        // Here you could adjust the chapter content or metadata based on the front matter
                        chapter.content = parsed.content; // Update the chapter content minus the front matter
                    }
                }
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html" // Adjust according to your needs
    }
}

#[derive(Deserialize, Debug)]
struct FrontMatter {
    title: String,
    tags: Vec<String>,
}