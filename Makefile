.DEFAULT_GOAL := build

build: 
	cargo build --release
	mv target/release/mdbook-frontmatter-reader /usr/local/bin/mdbook-frontmatter-reader

# build: prepare run

