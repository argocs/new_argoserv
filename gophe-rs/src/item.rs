#![allow(unused)]

use std::str::FromStr;

/// Represents all supported Gopher protocol item types
/// Some types are omitted due to being irrelevant today.
/// 
/// See RFC-1436 for reference
#[derive(Copy, Clone, Debug)]
pub enum GopherItem<'a> {
    /// Read as plaintext by the client
    TextFile(Selector<'a>),
    /// Used to navigate the server
    Directory(Selector<'a>),
    /// Signals an error condition
    Error(Message<'a>),
    /// A uuencoded file
    UuFile(Selector<'a>),
    /// Link to a telnet server
    Telnet(TelnetData<'a>),
    /// A binary file
    Binary(Selector<'a>),
    /// An image in the GIF format
    Gif(Selector<'a>),
    /// Some kind of image file
    Image(Selector<'a>),
    /// Info line, not a link
    Info(Message<'a>),
}

impl<'a> GopherItem<'a> {
    /// Returns the item prefix as assigned by RFC-1436 for this ItemType
    pub fn to_char(&self) -> char {
        match self {
            Self::TextFile(_) => '0',
            Self::Directory(_) => '1',
            Self::Error(_) => '3',
            Self::UuFile(_) => '6',
            Self::Telnet(_) => '8',
            Self::Binary(_) => '9',
            Self::Gif(_) => 'g',
            Self::Image(_) => 'I',
            Self::Info(_) => 'i',
        }
    }
}

impl<'a> FromStr for GopherItem<'a> {
    type Err = &'static str;

    fn from_str(from: &str) -> Result<Self, Self::Err> {
        let marker = from.chars().nth(0);
        if marker.is_none() {
            return Err("TODO MESSAGE: Missing content in string.")
        }

        match marker.unwrap() {
            '0' => {
                unimplemented!()
            },
            '1' => {
                unimplemented!()
            },
            '3' => {
                unimplemented!()
            },
            '6' => {
                unimplemented!()
            },
            '8' => {
                unimplemented!()
            },
            '9' => {
                unimplemented!()
            },
            'g' => {
                unimplemented!()
            },
            'I' => {
                unimplemented!()
            },
            'i' => {
                unimplemented!()
            },
            _ => Err("Invalid gopher item type. One of: 0, 1, 3, 6, 8, 9, g, I, i. See RFC-1436"),
        }
    }
}

/// An item that represents a selector
/// 
/// Element is a selector string
#[derive(Copy, Clone, Debug)]
pub struct Selector<'a>(pub &'a str);

impl<'a> Selector<'a> {
    /// Gets the selector of this item
    pub fn selector(&self) -> &'a str {
        self.0
    }
}

/// An item that represents a message
/// 
/// Element is the message shown to the client
#[derive(Copy, Clone, Debug)]
pub struct Message<'a>(pub &'a str);

impl<'a> Message<'a> {
    /// Gets the message of this item
    pub fn message(&self) -> &'a str {
        self.0
    }
}

/// Data for a telnet item
/// 
/// (host, port, login name)
#[derive(Copy, Clone, Debug)]
pub struct TelnetData<'a>(pub &'a str, pub u32, pub &'a str);

impl<'a> TelnetData<'a> {
    /// Gets the hostname/ip of the telnet connection
    pub fn host(&self) -> &'a str {
        self.0
    }

    /// Gets the port of the telnet connection
    pub fn port(&self) -> u32 {
        self.1
    }

    /// Gets the login name to use for the connection
    pub fn login_name(&self) -> &'a str {
        self.2
    }
}