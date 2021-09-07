use bytes::Bytes;
use derive_more::Display;
use flowy_derive::{ProtoBuf, ProtoBuf_Enum};
use flowy_dispatch::prelude::{EventResponse, ResponseBuilder};
use flowy_net::errors::ErrorCode as NetworkErrorCode;
use std::{convert::TryInto, fmt};

#[derive(Debug, Default, Clone, ProtoBuf)]
pub struct WorkspaceError {
    #[pb(index = 1)]
    pub code: ErrorCode,

    #[pb(index = 2)]
    pub msg: String,
}

impl WorkspaceError {
    pub fn new(code: ErrorCode, msg: &str) -> Self { Self { code, msg: msg.to_owned() } }
}

#[derive(Debug, Clone, ProtoBuf_Enum, Display, PartialEq, Eq)]
pub enum ErrorCode {
    #[display(fmt = "Unknown")]
    Unknown              = 0,

    #[display(fmt = "Workspace name is invalid")]
    WorkspaceNameInvalid = 1,

    #[display(fmt = "Workspace id is invalid")]
    WorkspaceIdInvalid   = 2,

    #[display(fmt = "Color style of the App is invalid")]
    AppColorStyleInvalid = 3,

    #[display(fmt = "Workspace desc is invalid")]
    WorkspaceDescInvalid = 4,

    #[display(fmt = "Current workspace not found")]
    CurrentWorkspaceNotFound = 5,

    #[display(fmt = "Id of the App  is invalid")]
    AppIdInvalid         = 10,

    #[display(fmt = "Name of the App  is invalid")]
    AppNameInvalid       = 11,

    #[display(fmt = "Name of the View  is invalid")]
    ViewNameInvalid      = 20,

    #[display(fmt = "Thumbnail of the view is invalid")]
    ViewThumbnailInvalid = 21,

    #[display(fmt = "Id of the View is invalid")]
    ViewIdInvalid        = 22,

    #[display(fmt = "Description of the View is invalid")]
    ViewDescInvalid      = 23,

    #[display(fmt = "Get database connection failed")]
    DatabaseConnectionFail = 100,

    #[display(fmt = "Database internal error")]
    WorkspaceDatabaseError = 101,

    #[display(fmt = "User internal error")]
    UserInternalError    = 102,

    #[display(fmt = "User not login yet")]
    UserNotLoginYet      = 103,

    #[display(fmt = "UserIn is empty")]
    UserIdIsEmpty        = 104,

    #[display(fmt = "Server error")]
    ServerError          = 1000,
    #[display(fmt = "Record not found")]
    RecordNotFound       = 1001,
}

impl std::default::Default for ErrorCode {
    fn default() -> Self { ErrorCode::Unknown }
}

impl std::convert::From<flowy_net::errors::ServerError> for WorkspaceError {
    fn from(error: flowy_net::errors::ServerError) -> Self {
        match error.code {
            NetworkErrorCode::RecordNotFound => ErrorBuilder::new(ErrorCode::RecordNotFound).error(error.msg).build(),

            _ => ErrorBuilder::new(ErrorCode::ServerError).error(error.msg).build(),
        }
    }
}

impl std::convert::From<flowy_database::Error> for WorkspaceError {
    fn from(error: flowy_database::Error) -> Self { ErrorBuilder::new(ErrorCode::WorkspaceDatabaseError).error(error).build() }
}

impl flowy_dispatch::Error for WorkspaceError {
    fn as_response(&self) -> EventResponse {
        let bytes: Bytes = self.clone().try_into().unwrap();
        ResponseBuilder::Err().data(bytes).build()
    }
}

impl fmt::Display for WorkspaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:?}: {}", &self.code, &self.msg) }
}

pub type ErrorBuilder = flowy_infra::errors::Builder<ErrorCode, WorkspaceError>;

impl flowy_infra::errors::Build<ErrorCode> for WorkspaceError {
    fn build(code: ErrorCode, msg: String) -> Self { WorkspaceError::new(code, &msg) }
}
