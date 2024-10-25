use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct AppendFile {
    path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding: Option<Encoding>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub enum Encoding {
    #[default]
    #[serde(rename = "utf8", alias = "utf-8")]
    Utf8,
    #[serde(rename = "utf16", alias = "utf-16")]
    Utf16,
    #[serde(rename = "utf16-LE", alias = "utf-16-LE")]
    Utf16LE,
    #[serde(rename = "utf16-BE", alias = "utf-16-BE")]
    Utf16BE,
    #[serde(rename = "utf32", alias = "utf-32")]
    Utf32,
    #[serde(rename = "utf32-LE", alias = "utf-32-LE")]
    Utf32LE,
    #[serde(rename = "utf32-BE", alias = "utf-32-BE")]
    Utf32BE,
    #[serde(rename = "UTF8", alias = "UTF-8")]
    UTF8,
    #[serde(rename = "UTF16", alias = "UTF-16")]
    UTF16,
    #[serde(rename = "UTF16-LE", alias = "UTF-16-LE")]
    UTF16LE,
    #[serde(rename = "UTF16-BE", alias = "UTF-16-BE")]
    UTF16BE,
    #[serde(rename = "UTF32", alias = "UTF-32")]
    UTF32,
    #[serde(rename = "UTF32-LE", alias = "UTF-32-LE")]
    UTF32LE,
    #[serde(rename = "UTF32-BE", alias = "UTF-32-BE")]
    UTF32BE,
}
