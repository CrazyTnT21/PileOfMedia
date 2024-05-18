#[derive(utoipa::IntoResponses)]
#[response(status = 400)]
pub struct BadRequest(String);
