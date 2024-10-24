use serde::{Deserialize, Serialize};

mod assert;
mod delete_file;
mod execute_bash;
mod execute_binary;
mod execute_document;
mod s3_download;

pub use assert::Assert;
pub use delete_file::DeleteFile;
pub use execute_bash::ExecuteBash;
pub use execute_binary::ExecuteBinary;
pub use execute_document::ExecuteDocument;
pub use s3_download::S3Download;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "action", content = "inputs")]
pub enum Action {
    Assert(Assert),
    S3Download(Vec<S3Download>),
    ExecuteBash(ExecuteBash),
    ExecuteBinary(ExecuteBinary),
    DeleteFile(Vec<DeleteFile>),
    ExecuteDocument(ExecuteDocument),
}
