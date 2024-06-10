#[derive(utoipa::IntoResponses)]
#[response(status = 401)]
pub struct NotAuthorized(String);
