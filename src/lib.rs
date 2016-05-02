/*
 *   Copyright 2016 Jean Piere Dudey
 *
 *   Licensed under the Apache License, Version 2.0 (the "License");
 *   you may not use this file except in compliance with the License.
 *   You may obtain a copy of the License at
 *
 *       http://www.apache.org/licenses/LICENSE-2.0
 *
 *   Unless required by applicable law or agreed to in writing, software
 *   distributed under the License is distributed on an "AS IS" BASIS,
 *   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *   See the License for the specific language governing permissions and
 *   limitations under the License.
*/

#![deny(missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unused_import_braces, unused_qualifications)]

#![cfg_attr(feature="dev", feature(plugin))]
#![cfg_attr(feature="dev", plugin(clippy))]

/// This crate provides the type Uri used to parse a given uri string.
///
/// # Usage
///
/// This crate is on crates.io and can be used by adding uri to your dependencies in your
/// project's Cargo.toml.
///
/// ```toml
/// [dependencies]
/// uri = "0.1"
/// ```
///
/// and this to your crate root:
/// ```
/// extern crate uri;
/// ```

#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::*;

/// Uri represents the data contained in an uri string.
#[derive(Default, Debug)]
pub struct Uri {
    /// Represents the scheme of an uri.
    ///
    /// # Examples
    ///
    /// ```
    /// use uri::Uri;
    ///
    /// let uri = Uri::new("https://doc.rust-lang.org/book/README.html").unwrap();
    /// assert_eq!(uri.scheme, "https");
    /// ```
    pub scheme: String,

    /// Represents the username (authority) of an uri.
    ///
    /// # Examples
    ///
    /// ```
    /// use uri::Uri;
    ///
    /// let uri = Uri::new("https://user:pass@doc.rust-lang.org/book/README.html").unwrap();
    /// assert_eq!(uri.username.unwrap(), "user");
    /// ```
    pub username: Option<String>,

    /// Represents the password (authority) of an uri.
    ///
    /// # Examples
    ///
    /// ```
    /// use uri::Uri;
    ///
    /// let uri = Uri::new("https://user:pass@doc.rust-lang.org/book/README.html").unwrap();
    /// assert_eq!(uri.password.unwrap(), "pass");
    /// ```
    pub password: Option<String>,

    /// Represents the host of an uri.
    ///
    /// # Examples
    ///
    /// ```
    /// use uri::Uri;
    ///
    /// let uri = Uri::new("https://doc.rust-lang.org/book/README.html").unwrap();
    /// assert_eq!(uri.host.unwrap(), "doc.rust-lang.org");
    /// ```
    pub host: Option<String>,

    /// Represents the port of an uri.
    ///
    /// # Examples
    ///
    /// ```
    /// use uri::Uri;
    ///
    /// let uri = Uri::new("https://doc.rust-lang.org:1234").unwrap();
    /// assert_eq!(uri.host.unwrap(), "doc.rust-lang.org");
    /// assert_eq!(uri.port, Some(1234));
    /// ```
    pub port: Option<u16>,

    /// Represents the path of an uri.
    ///
    /// # Examples
    ///
    /// ```
    /// use uri::Uri;
    ///
    /// let uri = Uri::new("https:/doc.rust-lang.org/book/README.html").unwrap();
    /// assert_eq!(uri.path.unwrap(), "/book/README.html");
    /// ```
    pub path: Option<String>,

    /// Represents the query of an uri.
    ///
    /// # Examples
    ///
    /// ```
    /// use uri::Uri;
    ///
    /// let uri = Uri::new("https:/doc.rust-lang.org/?query=value&q=v").unwrap();
    /// assert_eq!(uri.query.unwrap(), "query=value&q=v");
    /// ```
    pub query: Option<String>,

    /// Represents the fragment of an uri.
    ///
    /// # Examples
    ///
    /// ```
    /// use uri::Uri;
    ///
    /// let uri = Uri::new("https:/doc.rust-lang.org/#fragment").unwrap();
    /// assert_eq!(uri.fragment.unwrap(), "fragment");
    /// ```
    pub fragment: Option<String>,
}

macro_rules! map_to_string {
    ( $x:expr ) => {
        $x.map(String::from);
    };
}

macro_rules! map_to_u16 {
    ( $x:expr ) => {
        $x.map(|value| String::from(value).parse::<u16>().unwrap());
    };
}

lazy_static! {
    static ref URI_REGEX: Regex = Regex::new(URI_PATTERN).unwrap();
}

static URI_PATTERN: &'static str = "^(?P<scheme>[a-zA-Z][a-zA-Z0-9+.-]*):\
                                    /{0,3}\
                                    (?P<username>.*?)?\
                                    (:(?P<password>.*?))?@?\
                                    (?P<host>[0-9\\.A-Za-z-?]+)\
                                    (?::(?P<port>\\d+))?\
                                    (:?(?P<path>/[^?#]*))?\
                                    (?:\\?(?P<query>[^#]*))?\
                                    (?:#(?P<fragment>.*))?$";

/// Checks if a given string is an URI.
///
/// # Examples
///
/// ```
/// use uri::is_uri;
///
/// let result = is_uri("https://doc.rust-lang.org/book/README.html");
/// assert_eq!(result, true);
/// ```
pub fn is_uri(uristr: &str) -> bool {
    URI_REGEX.is_match(uristr)
}

impl Uri {
    /// Creates Uri object from a given uri string.
    ///
    /// # Failures
    ///
    /// If the string isn't a valid uri the function will return `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use uri::Uri;
    ///
    /// let uri = match Uri::new("https://doc.rust-lang.org/book/README.html") {
    ///     Some(value) => value,
    ///     None => panic!("Oh no!")
    /// };
    /// ```
    pub fn new(uristr: &str) -> Option<Uri> {
        if !is_uri(uristr) {
            return None;
        }

        let mut uri = Uri {
            scheme: String::new(),
            username: None,
            password: None,
            host: None,
            port: None,
            path: None,
            query: None,
            fragment: None,
        };

        match URI_REGEX.captures(uristr) {
            Some(caps) => {
                match caps.name("scheme") {
                    Some(scheme) => uri.scheme = String::from(scheme),
                    None => {println!("here"); return None;}
                }

                uri.username = map_to_string!(caps.name("username"));
                uri.password = map_to_string!(caps.name("password"));
                uri.host = map_to_string!(caps.name("host"));
                uri.port = map_to_u16!(caps.name("port"));
                uri.path = map_to_string!(caps.name("path"));
                uri.query = map_to_string!(caps.name("query"));
                uri.fragment = map_to_string!(caps.name("fragment"));
            },
            None => return None
        };

        Some(uri)
    }
}

#[cfg(test)]
mod test {

    static URI_GOOD_STRING: &'static str = "https://www.unknown.host/false/path.html?query=value";
    static URI_BAD_STRING: &'static str = "lazy string";

    #[test]
    fn is_uri() {
        let res1 = ::is_uri(URI_GOOD_STRING);
        assert_eq!(res1, true);

        let res2 = ::is_uri(URI_BAD_STRING);
        assert_eq!(res2, false);
    }

    #[test]
    fn uri_new() {
        match ::Uri::new(URI_GOOD_STRING) {
            Some(uri) => {
                assert_eq!(uri.scheme, "https");
                assert_eq!(uri.host.unwrap(), "www.unknown.host");
                assert_eq!(uri.path.unwrap(), "/false/path.html");
            }
            None => panic!("Cannot create uri from a valid one"),
        }

        match ::Uri::new("https://rust:1234567eight9@www.unknown.host:1345") {
            Some(uri) => {
                assert_eq!(uri.scheme, "https");
                assert_eq!(uri.host.unwrap(), "www.unknown.host");
                assert_eq!(uri.port.unwrap(), 1345);
                assert_eq!(uri.username.unwrap(), "rust");
                assert_eq!(uri.password.unwrap(), "1234567eight9");
            }
            None => panic!("Cannot create uri from a valid one"),
        }
    }

}
