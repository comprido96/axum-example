use axum::{http::StatusCode, response::{IntoResponse, Response}};


pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInReqExt,

    LoginFail,

    TicketDeleteFailIdNotFound {id: u64,},
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("--> {:<12} - {self:?}", "INTO_RES", );

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED CLIENT ERROR").into_response()
    }
}