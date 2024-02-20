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
                if chapter.name == "Introduction to the Field of Data Engineering"{

                let parsed = matter.parse(&chapter.content);
                // eprintln!("DEBUG: Parsed frontmatter: {:?}, {:?}", parsed.data, &chapter.name);
                // eprintln!("DEBUG: Parsed frontmatter: {:?}", &chapter);

                let base_url = "https://www.dedp.online";

                if let Some(data) = parsed.data {
                    if let Ok(front_matter) = data.deserialize::<FrontMatter>() {
                        // eprintln!("title: {}", front_matter.title);
                        // eprintln!("description: {:?}", front_matter.description);
                        // eprintln!("featured_image_url: {:?}", front_matter.featured_image_url);
                        // eprintln!("author: {:?}", front_matter.author);

                        let chapter_url = chapter.source_path.as_ref().map_or_else(
                            || base_url.to_string(),
                            |path| {
                                let mut path_str = path.display().to_string();
                                // Replace `.md` with `.html`
                                path_str = path_str.replace(".md", ".html");
                                format!("{}/{}", base_url, path_str)
                            }
                        );

                        // Construct meta tags string
                        let meta_tags = format!(
                            "<meta property=\"og:title\" content=\"{}\" />\n\
                             <meta property=\"og:image\" content=\"{}\" />\n\
                             <meta property=\"og:url\" content=\"{}\" />\n\
                             <meta property=\"og:description\" content=\"{}\" />\n\
                             <meta name=\"author\" content=\"{}\" />",
                            front_matter.title,
                            front_matter.featured_image_url,
                            &chapter_url,
                            front_matter.description,
                            front_matter.author
                        );
                        // Example of injecting directly into the chapter content
                        // This is a simplistic approach and might not be suitable for your actual requirements
                        chapter.content = format!("{}\n{}", meta_tags, chapter.content);
                    }
                }
                chapter.content = parsed.content; // Update the content minus the front matter
                }
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
    description: String,
    featured_image_url: String,
    author: String,
}
