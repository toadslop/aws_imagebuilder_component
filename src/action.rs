use create_symlink::CreateSymlink;
use file_system::MoveOperation;
use serde::{Deserialize, Serialize};

mod assert;
mod create_file;
mod create_folder;
mod create_symlink;
mod delete_file;
mod delete_folder;
mod execute_bash;
mod execute_binary;
mod execute_document;
mod execute_powershell;
mod file_system;
mod list_files;
mod read_file;
mod s3_download;
mod s3_upload;
mod web_download;

pub use assert::Assert;
pub use create_file::CreateFile;
pub use create_folder::CreateFolder;
pub use delete_file::DeleteFile;
pub use delete_folder::DeleteFolder;
pub use execute_bash::ExecuteBash;
pub use execute_binary::ExecuteBinary;
pub use execute_document::ExecuteDocument;
pub use execute_powershell::ExecutePowerShell;
pub use file_system::File;
pub use list_files::ListFiles;
pub use read_file::ReadFile;
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
    AppendFile(Vec<File>),
    CopyFile(Vec<MoveOperation>),
    CopyFolder(Vec<MoveOperation>),
    CreateFile(Vec<CreateFile>),
    CreateFolder(Vec<CreateFolder>),
    CreateSymlink(Vec<CreateSymlink>),
    DeleteFolder(Vec<DeleteFolder>),
    ListFiles(Vec<ListFiles>),
    MoveFile(Vec<MoveOperation>),
    MoveFolder(Vec<MoveOperation>),
    ReadFile(Vec<ReadFile>),
}
