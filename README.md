<--
   Copyright 2016 Jean Piere Dudey

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
-->


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

### License
```
   Copyright 2016 Jean Piere Dudey

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
```