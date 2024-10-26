use serde::{Deserialize, Serialize};

mod assert;
mod delete_file;
mod execute_bash;
mod execute_binary;
mod execute_document;
mod execute_powershell;
mod file_operation;
mod move_operation;
mod s3_download;
mod s3_upload;
mod web_download;

pub use assert::Assert;
pub use delete_file::DeleteFile;
pub use execute_bash::ExecuteBash;
pub use execute_binary::ExecuteBinary;
pub use execute_document::ExecuteDocument;
pub use execute_powershell::ExecutePowerShell;
pub use file_operation::CreateFile;
pub use file_operation::CreateFolder;
pub use file_operation::FileOperation;
pub use move_operation::MoveOperation;
pub use s3_download::S3Download;
pub use s3_upload::S3Upload;
pub use web_download::WebDownload;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "action", content = "inputs")]
pub enum Action {
    Assert(Assert),
    S3Download(Vec<S3Download>),
    ExecuteBash(ExecuteBash),
    ExecuteBinary(ExecuteBinary),
    DeleteFile(Vec<DeleteFile>),
    ExecuteDocument(ExecuteDocument),
    ExecutePowerShell(ExecutePowerShell),
    S3Upload(Vec<S3Upload>),
    WebDownload(Vec<WebDownload>),
    AppendFile(Vec<FileOperation>),
    CopyFile(Vec<MoveOperation>),
    CopyFolder(Vec<MoveOperation>),
    CreateFile(Vec<CreateFile>),
    CreateFolder(Vec<CreateFolder>),
}
