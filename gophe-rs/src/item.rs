#![allow(unused)]
/// Represents all supported Gopher protocol item types
/// Some types are omitted due to being irrelevant today
#[derive(Copy, Clone, Debug)]
pub enum ItemType<'a> {
    /// Read as plaintext by the client
    TextFile(SelectorItem<'a>),
    /// Used to navigate the server
    Directory(SelectorItem<'a>),
    /// Signals an error condition
    Error(MessageItem<'a>),
    /// A uuencoded file
    UuFile(SelectorItem<'a>),
    /// Link to a telnet server
    Telnet(TelnetItem<'a>),
    /// A binary file
    Binary(SelectorItem<'a>),
    /// An image in the GIF format
    Gif(SelectorItem<'a>),
    /// Some kind of image file
    Image(SelectorItem<'a>),
    /// Info line, not a link
    Info(MessageItem<'a>),
}

impl<'a> ItemType<'a> {
    /// Returns the item prefix as assigned by RFC-1436 for this ItemType
    pub fn to_char(&self) -> char {
        match self {
            Self::TextFile(_) => '0',
            Self::Directory(_) => '1',
            Self::Error(_) => '3',
            Self::UuFile(_) => '6',
            Self::Telnet(..) => '8',
            Self::Binary(_) => '9',
            Self::Gif(_) => 'g',
            Self::Image(_) => 'I',
            Self::Info(_) => 'i',
        }
    }
}

/// An item that represents a selector
/// 
/// Element is a selector string
#[derive(Copy, Clone, Debug)]
pub struct SelectorItem<'a>(&'a str);

impl<'a> SelectorItem<'a> {
    /// Gets the selector of this item
    pub fn selector(&self) -> &'a str {
        self.0
    }
}

/// An item that represents a message
/// 
/// Element is the message shown to the client
#[derive(Copy, Clone, Debug)]
pub struct MessageItem<'a>(&'a str);

impl<'a> MessageItem<'a> {
    /// Gets the message of this item
    pub fn message(&self) -> &'a str {
        self.0
    }
}

/// Data for a telnet item
/// 
/// (host, port, login name)
#[derive(Copy, Clone, Debug)]
pub struct TelnetItem<'a>(&'a str, u32, &'a str);

impl<'a> TelnetItem<'a> {
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