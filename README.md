# adjust-epub-page-nums
CLI tool for adjusting page numbering in EPUBs

## Preparation
- Un-zip the EPUB
- Page numbers must be in *exactly* this format: `<span epub:type="pagebreak" id="page{num}" title="{num}"></span>` where `{num}` is a whole number greater than 0. For example:
    - `<span epub:type="pagebreak" id="page1" title="1"></span>`
    - `<span epub:type="pagebreak" id="page735" title="735"></span>`

## Usage
- Within the EPUB's .xhtml files, after each page number you'd like to adjust, add an HTML comment `<!-- {add|subtract} {amount} -->` where `{amount}` is a whole number greater than 0. For example:
    - `<span epub:type="pagebreak" id="page1" title="1"></span><!-- add 3 -->`
    - `<span epub:type="pagebreak" id="page735" title="735"></span><!-- subtract 101 -->`
- The tool takes, as an argument on the command line, the path to the folder containing the un-zipped EPUB's .xhtml files you'd like to process. For example:
    - `adjust-epub-page-nums ~/Documents/EPUBS/book1/text`

## Behavior
- When run, the tool will log to STDOUT what files it processed
- For each page number/comment combination matching the accepted format, the page number will be adjusted accordingly, and the comment removed. For example:
    - `<span epub:type="pagebreak" id="page1" title="1"></span><!-- add 3 -->` becomes `<span epub:type="pagebreak" id="page4" title="4"></span>`
    - `<span epub:type="pagebreak" id="page735" title="735"></span><!-- subtract 101 -->` becomes `<span epub:type="pagebreak" id="page634" title="634"></span>`
- If the result of performing the given operation on a page number results in 0 or less, the number will be substituted with 'i'. For example:
    - `<span epub:type="pagebreak" id="page1" title="1"></span><!-- subtract 3 -->` becomes `<span epub:type="pagebreak" id="pagei" title="i"></span>`

## Tips
I find it most useful to either copy the tool's binary or create a symlink to it in a folder included in the environment's PATH.

That way, I can `cd` into the folder containing .xhtml files and run `adjust-epub-page-nums .`

Feel free to shorten the command, either by re-naming the binary or symlink. I have mine as `adj-nums`

## Building from Source
- Clone this repo
- Clone [flintsteel7/read-write-files](https://github.com/flintsteel7/read-write-files)
- Ensure the `path` to the read-write-files dependency in **Cargo.toml** is correct
- With appropriate versions of Rust and Cargo installed, run `cargo build --release` from within this repo
- The binary should be written to **/target/release/**