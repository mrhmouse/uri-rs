<p align="center">
  <a href="https://travis-ci.org/jeandudey/uri-rs">
      <img src="https://travis-ci.org/jeandudey/uri-rs.svg?branch=master" alt="Build Status">
  </a>
</p>

## uri-rs
`uri` is a library to help in parsing and creating an uri.

### Examples
- Parsing an uri.
```rust
extern crate uri;

use uri::Uri;

fn main() {
    match Uri::new("https://github.com/jeandudey/uri-rs/") {
        Some(uri) => {
            println!("-- URI INFO --");
            println!("Scheme: {}", uri.scheme);
            println!("Host: {}", uri.host.unwrap());
            println!("Path: {}", uri.path.unwrap());
        },
        None => panic!("Oh no!")
    }
}
```
