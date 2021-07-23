# pagesegtest1 written by Rust

## What's this
A porting of [leptonica pagesegtest1.c](https://github.com/DanBloomberg/leptonica/blob/master/prog/pagesegtest1.c).

### Build
Install before building
```bash
sudo apt install libclang-10-dev libleptonica-dev
```

Run
```bash
cargo build
```

### Test
Run
```bash
cargo run pageseg1.tif 
```

Then, `/tmp/lept/pageseg/debug.pdf` will be created.

### TIPS
Create bilevel gray image
```bash
convert -verbose -type bilevel -depth 1 input.jpg output.tif
```
