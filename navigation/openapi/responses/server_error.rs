#[derive(utoipa::IntoResponses)]
#[response(status = 500)]
pub struct ServerError(String);
