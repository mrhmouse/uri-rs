
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

//! This crate provides the type Uri used to parse a given uri string.
//!
//! # Usage
//!
//! This crate is on crates.io and can be used by adding uri to your dependencies in your
//! project's Cargo.toml.
//!
//! ```toml
//! [dependencies]
//! uri = "0.2"
//! ```
//!
//! and this to your crate root:
//! ```
//! extern crate uri;
//! ```

#[macro_use]
extern crate lazy_static;
extern crate regex;
use std::fmt;
use regex::*;

/// Uri represents the data contained in an uri string.
#[derive(Default, Debug)]
pub struct Uri {
    /// Represents the scheme of an uri.
    pub scheme: String,

    /// Represents the username (authority) of an uri.
    pub username: Option<String>,

    /// Represents the password (authority) of an uri.
    pub password: Option<String>,

    /// Represents the host of an uri.
    pub host: Option<String>,

    /// Represents the port of an uri.
    pub port: Option<u16>,

    /// Represents the path of an uri.
    pub path: Option<String>,

    /// Represents the query of an uri.
    pub query: Option<String>,

    /// Represents the fragment of an uri.
    pub fragment: Option<String>,
}

macro_rules! map_to_string {
    ( $x:expr ) => ({
        if let Some(contents) = $x.map(String::from) {
            if contents == "" {
                None
            } else {
                Some(contents)
            }
        } else {
            None
        }
    });
}

/// Parses a string into a u16.

fn map_to_u16(value: &str) -> Option<u16> {
    match String::from(value).parse::<u16>() {
        Ok(parsed_value) => Some(parsed_value),
        _ => None
    }
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
                uri.port = caps.name("port").and_then(map_to_u16);
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
    fn blank_username_should_parse_as_none() {
        let uri_with_blank_username = "http://some.host/";
        if let Some(parsed_uri) = ::Uri::new(uri_with_blank_username) {
            assert_eq!(parsed_uri.username, None);
        } else {
            panic!("Cannot create a URI from a valid string");
        }
    }

    #[test]
    fn bad_port_shouldnt_panic() {
        let bad_port_uri = "http://some.host:99999";
        if let Some(parsed_uri) = ::Uri::new(bad_port_uri) {
            if let Some(weird_port) = parsed_uri.port {
                panic!("Incorrectly parsed port as {}", weird_port);
            }
        } else {
            panic!("Cannot create URI");
        }
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

    #[test]
    fn uri_should_parse_roundtrip() {
        let uris = ["http://user:pass@host.tld/path?query#fragment",
                    "https://host.tld/?query=1234&asdf=bar",
                    "ftp://user@subdomain.host.tld/",
                    "some-custom-scheme://foo.bar.baz/path#some-fragment&a=1",
                    "gopher://foo.bar:1234/asdf"];
        for uri in uris.iter() {
            test_roundtrip_parsing(uri);
        }
    }

    fn test_roundtrip_parsing(uri: &str) {
        let original = ::Uri::new(uri)
            .expect("Cannot parse URI from a valid string");
        let roundtrip = ::Uri::new(original.to_string().as_str())
            .expect("Cannot parse URI from string version of a Uri struct");

        assert_eq!(original.scheme, roundtrip.scheme);
        assert_eq!(original.username, roundtrip.username);
        assert_eq!(original.password, roundtrip.password);
        assert_eq!(original.host, roundtrip.host);
        assert_eq!(original.port, roundtrip.port);
        assert_eq!(original.path, roundtrip.path);
        assert_eq!(original.query, roundtrip.query);
        assert_eq!(original.fragment, roundtrip.fragment);
    }
}

struct UriWriter<'a, 'b, 'c: 'b> {
    uri: &'a Uri,
    formatter: &'b mut fmt::Formatter<'c>,
}

impl<'a, 'b, 'c> UriWriter<'a, 'b, 'c> {
    fn new(uri: &'a Uri, formatter: &'b mut fmt::Formatter<'c>)
           -> UriWriter<'a, 'b, 'c> {
        return UriWriter { uri: uri, formatter: formatter};
    }

    fn write_uri(&mut self) -> fmt::Result {
        if !self.is_valid_uri() {
            return Err(fmt::Error);
        }
        
        try!(self.write_scheme());
        try!(self.write_authority());
        try!(self.write_host());
        try!(self.write_port());
        try!(self.write_path());
        try!(self.write_query());
        return self.write_fragment();
    }

    fn is_valid_uri(&self) -> bool {
        return self.uri.host.is_some();
    }

    fn write_scheme(&mut self) -> fmt::Result {
        return write!(self.formatter, "{}://", self.uri.scheme);
    }

    fn write_authority(&mut self) -> fmt::Result {
        if let Some(ref user) = self.uri.username {
            try!(write!(self.formatter, "{}", user));
            if let Some(ref pass) = self.uri.password {
                try!(write!(self.formatter, ":{}", pass));
            }

            try!(write!(self.formatter, "@"));
        }

        return Ok(());
    }

    fn write_host(&mut self) -> fmt::Result {
        if let Some(ref host) = self.uri.host {
            return write!(self.formatter, "{}", host);
        } else {
            return Err(fmt::Error);
        }
    }

    fn write_port(&mut self) -> fmt::Result {
        if let Some(ref port) = self.uri.port {
            try!(write!(self.formatter, ":{}", port));
        }

        return Ok(());
    }

    fn write_path(&mut self) -> fmt::Result {
        if let Some(ref path) = self.uri.path {
            return write!(self.formatter, "{}", path);
        } else {
            return write!(self.formatter, "/");
        }
    }

    fn write_query(&mut self) -> fmt::Result {
        if let Some(ref query) = self.uri.query {
            try!(write!(self.formatter, "?{}", query));
        }

        return Ok(());
    }

    fn write_fragment(&mut self) -> fmt::Result {
        if let Some(ref fragment) = self.uri.fragment {
            try!(write!(self.formatter, "#{}", fragment));
        }

        return Ok(());
    }
}

impl fmt::Display for Uri {
    fn fmt(&self, f: &mut fmt::Formatter)
           -> fmt::Result {
        let mut writer = UriWriter::new(self, f);
        writer.write_uri()
    }
}
