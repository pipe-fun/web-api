pub trait StatusTrait {
    type StatusCode;
    type Status;
    type DbAPIStatus;
    type _DbAPIStatus;
    fn set_status(self, status: Self::Status) -> Self;
    fn set_db_api_status(self, status: Self::DbAPIStatus) -> Self;
    fn set_db_api_err(status: Self::_DbAPIStatus, e: String) -> Self;
    fn set_db_api_err_simple(status: Self::DbAPIStatus) -> Self;
    fn status_code(&self) -> Self::StatusCode;
    fn status(&self) -> Self::Status;
    fn db_api_status(&self) -> Self::DbAPIStatus;
}