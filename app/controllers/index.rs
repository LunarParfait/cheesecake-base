use axum::response::IntoResponse;
use config::cheesecake::app_error::AppResult;

pub async fn render() -> AppResult<impl IntoResponse> {
    Ok(views::index::render()?)
}

pub async fn post_smth() -> AppResult<impl IntoResponse> {
    Ok(views::index::render()?)
}
