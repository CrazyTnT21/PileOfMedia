use super::*;

#[derive(utoipa::OpenApi)]
#[openapi(tags((name = "Accounts", description = "Endpoints related to managing accounts")),
  paths(login, register, refresh_token)
)]
pub(crate) struct AccountDoc;
