#![deny(missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
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
    /// let uri = Uri::new("https:/doc.rust-lang.org:1234");
    /// assert_eq!(uri.port.unwrap(), 1234);
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

static URI_PATTERN: &'static str = "(?P<scheme>[a-zA-Z][a-zA-Z0-9+.-]*):\
                                    (?P<authority_and_path>[^?#]*)(?:\\?([^#]*))?(?:#(.*))?";

static AUTHORITY_AND_PATH_PATTERN: &'static str = "//(?P<authority>[^/]*)(?P<path>/.*)?";
static AUTHORITY_PATTERN: &'static str = "(?:(?P<username>[^@:]*)(?::\
                                          (?P<password>[^@]*))?@)(?P<host>\\[[^\\]]*\\]|[^\\[:\
                                          ]*)(?::(?P<port>\\d*))?";

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
    let uri_re = Regex::new(URI_PATTERN).unwrap();
    uri_re.is_match(uristr)
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
        };

        let uri_re = Regex::new(URI_PATTERN).unwrap();
        let uricaps = match uri_re.captures(uristr) {
            Some(value) => value,
            None => return None,
        };

        let scheme = map_to_string!(uricaps.name("scheme"));
        match scheme {
            Some(value) => uri.scheme = value,
            None => return None,
        };

        let auth_and_path = uricaps.name("authority_and_path");
        if auth_and_path == None {
            return None;
        }

        let auth_and_path_re = Regex::new(AUTHORITY_AND_PATH_PATTERN).unwrap();
        let auth_and_path_caps = match auth_and_path_re.captures(auth_and_path.unwrap()) {
            Some(value) => value,
            None => return None,
        };


        let authority = auth_and_path_caps.name("authority");
        if authority == None {
            return None;
        }

        let authority_re = Regex::new(AUTHORITY_PATTERN).unwrap();
        match authority_re.captures(authority.unwrap()) {
            Some(caps) => {
                uri.username = map_to_string!(caps.name("username"));
                uri.password = map_to_string!(caps.name("password"));
                uri.host = map_to_string!(caps.name("host"));
                uri.port = map_to_u16!(caps.name("port"));
            }
            None => uri.host = Some(String::from(authority.unwrap())),
        }

        uri.path = map_to_string!(auth_and_path_caps.name("path"));

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
