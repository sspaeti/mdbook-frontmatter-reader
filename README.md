# mdbook-frontmatter-reader

This is a preprocessor for [mdBook](https://github.com/rust-lang/mdBook/) to read the frontmatter of each page and convert it to ogp meta tags which are helpful for SEO and social media sharing.

E.g. you can add a `description, title, image` to the frontmatter of a page and it will be converted to a meta tag like this:


In your chapter at the beginning of each markdown file, you can add a frontmatter like this:
```yaml
---
title: "Introduction to the Field of Data Engineering"
description: "Explore the transition from early data warehousing to today's advanced cloud computing, AI hype, and modern data stacks in data engineering. This chapter provides a concise overview of the field's evolution, key advancements, and current challenges, along with practical strategies for navigating the complex landscape, crafted by an industry pioneer for data engineers at all levels."
featured-image-url: "https://www.dedp.online/static/og-image.png"
canonicalUrl: "https://www.dedp.online"
canonicalUrl: "Simon"
---

# Title 
my text of chapter here.
```

The rendered HTML tag that gets added to `<head>`-tag will look like this:
```html
        <meta property="og:title" content="Introduction to the Field of Data Engineering">
        <meta property="og:image" content="https://www.dedp.online/static/og-image.png">
        <meta property="og:url" content="https://www.dedp.online/part-1/1-introduction/_intro-data-engineering.html">
        <meta property="og:description" content="Explore the transition from early data warehousing to today's advanced cloud computing, AI hype, and modern data stacks in data engineering. This chapter provides a concise overview of the field's evolution, key advancements, and current challenges, along with practical strategies for navigating the complex landscape, crafted by an industry pioneer for data engineers at all levels.">
        <meta name="author" content="Simon">
```

## Quick How-To

1. I installed it through `cargo install mdbook-frontmatter-reader` (I guess, maybe this part I did manually)
2. I added this config to my `book.toml`:

```toml
[preprocessor.frontmatter-reader]
command = "mdbook-frontmatter-reader"
```

The preprocessor will automatically check on each chapter, and add the meta tags to the html file.
