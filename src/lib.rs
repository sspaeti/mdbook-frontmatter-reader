use clap::{Arg, Command};
use gray_matter::engine::YAML;
use gray_matter::Matter;
use mdbook::book::{Book, BookItem};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
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
                // Optionally deserialize into a struct, if required
                if let Some(data) = parsed.data {
                    if let Ok(front_matter) = data.deserialize::<FrontMatter>() {
                        // Example: Print or use the front matter as needed
                        println!("Title: {}", front_matter.title);
                        println!("Tags: {:?}", front_matter.tags);
                    }
                }
                chapter.content = parsed.content; // Update the content minus the front matter
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        // Adjust this as needed based on which renderers you want to support
        renderer == "html"
    }
}

#[derive(Deserialize, Debug)]
struct FrontMatter {
    title: String,
    tags: Vec<String>,
}
