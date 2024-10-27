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
mod msi_action;
mod read_file;
mod reboot;
mod s3_download;
mod s3_upload;
mod set_file_encoding;
mod set_file_owner;
mod set_file_permissions;
mod set_folder_owner;
mod set_folder_permissions;
mod set_registry;
mod update_os;
mod web_download;

pub use assert::Assert;
pub use create_file::CreateFile;
pub use create_folder::CreateFolder;
pub use create_symlink::CreateSymlink;
pub use delete_file::DeleteFile;
pub use delete_folder::DeleteFolder;
pub use execute_bash::ExecuteBash;
pub use execute_binary::ExecuteBinary;
pub use execute_document::ExecuteDocument;
pub use execute_powershell::ExecutePowerShell;
pub use file_system::File;
pub use file_system::MoveOperation;
pub use list_files::ListFiles;
pub use msi_action::MsiAction;
pub use read_file::ReadFile;
pub use reboot::Reboot;
pub use s3_download::S3Download;
pub use s3_upload::S3Upload;
pub use set_file_encoding::SetFileEncoding;
pub use set_file_owner::SetFileOwner;
pub use set_file_permissions::SetFilePermissions;
pub use set_folder_owner::SetFolderOwner;
pub use set_folder_permissions::SetFolderPermissions;
pub use set_registry::SetRegistry;
pub use update_os::UpdateOS;
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
    SetFileEncoding(Vec<SetFileEncoding>),
    SetFileOwner(Vec<SetFileOwner>),
    SetFolderOwner(Vec<SetFolderOwner>),
    SetFolderPermissions(Vec<SetFolderPermissions>),
    SetFilePermissions(Vec<SetFilePermissions>),
    InstallMSI(MsiAction),
    UninstallMSI(MsiAction),
    Reboot(Reboot),
    SetRegistry(Vec<SetRegistry>),
    UpdateOS(UpdateOS),
}
