#[derive(utoipa::IntoResponses)]
#[response(status = 403)]
pub struct Forbidden(String);
