use axum::http::{StatusCode, Uri};

use crate::result::{AppResult, AppResultUtil as _};

/// 自定义反馈路由
pub async fn fallback(uri: Uri) -> AppResult {
    return AppResult::build_err_full(
        format!("Not found: {}", uri.to_string()),
        Some(1),
        Some(StatusCode::NOT_FOUND),
    );
}
