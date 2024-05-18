#[derive(utoipa::IntoResponses)]
#[response(status = NOT_FOUND)]
pub struct NotFound;
