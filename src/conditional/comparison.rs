use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Comparison {
    BinaryExists(String),
    FileExists(String),
    StringIsEmpty(String),
    FolderExists(String),
    StringIsWhitespace(String),
    #[serde(untagged)]
    #[serde(rename_all = "camelCase")]
    StringEquals {
        string_equals: String,
        value: String,
    },
    #[serde(untagged)]
    #[serde(rename_all = "camelCase")]
    StringLessThan {
        string_less_than: String,
        value: String,
    },

    #[serde(untagged)]
    #[serde(rename_all = "camelCase")]
    StringGreaterThan {
        string_greater_than: String,
        value: String,
    },
    #[serde(untagged)]
    #[serde(rename_all = "camelCase")]
    StringGreaterThanEquals {
        string_greater_than_equals: String,
        value: String,
    },
    #[serde(untagged)]
    #[serde(rename_all = "camelCase")]
    PatternMatches {
        pattern_matches: String,
        value: String,
    },
    #[serde(untagged)]
    #[serde(rename_all = "camelCase")]
    NumberEquals {
        number_equals: f64,
        value: f64,
    },
    #[serde(untagged)]
    #[serde(rename_all = "camelCase")]
    NumberLessThan {
        number_less_than: f64,
        value: f64,
    },
    #[serde(untagged)]
    #[serde(rename_all = "camelCase")]
    NumberLessThanEquals {
        number_less_than_equals: f64,
        value: f64,
    },
    #[serde(untagged)]
    #[serde(rename_all = "camelCase")]
    NumberGreaterThan {
        number_greater_than: f64,
        value: f64,
    },
    #[serde(untagged)]
    #[serde(rename_all = "camelCase")]
    NumberGreaterThanEquals {
        number_greater_than_equals: f64,
        value: f64,
    },
    #[serde(untagged)]
    FileMD5Equals {
        #[serde(rename = "fileMD5Equals")]
        file_md5_equals: f64,
        value: f64,
    },
    #[serde(untagged)]
    FileSHA1Equals {
        #[serde(rename = "fileSHA1Equals")]
        file_sha1_equals: f64,
        value: f64,
    },
    #[serde(untagged)]
    FileSHA256Equals {
        #[serde(rename = "fileSHA256Equals")]
        file_sha256_equals: f64,
        value: f64,
    },
    #[serde(untagged)]
    FileSHA512Equals {
        #[serde(rename = "fileSHA512Equals")]
        file_sha512_equals: f64,
        value: f64,
    },
}
