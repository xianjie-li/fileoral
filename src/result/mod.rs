//! 统一应用的响应和错误格式

use anyhow::anyhow;
use axum::{
    http::{status, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

/// 作为handler的通用返回类型
pub type AppResult = Result<Response, AppError>;

/// 统一应用返回
#[derive(Serialize, Debug)]
pub struct AppResponse<T: Serialize> {
    /// 0 : success, 1 >= : error
    pub code: u32,
    /// 成功或失败的消息
    pub msg: Option<String>,
    /// 数据
    pub data: Option<T>,
}

/// 用于构建AppResult的一些工具方法
pub trait AppResultUtil {
    /// 构建成功返回
    fn build_ok(data: impl Serialize) -> AppResult;

    /// 构建失败返回
    fn build_err(msg: impl Into<String>) -> AppResult;

    /// 构建失败返回(完整参数)
    fn build_err_full(
        msg: impl Into<String>,
        code: Option<u32>,
        status_code: Option<StatusCode>,
    ) -> AppResult;
}

impl AppResultUtil for AppResult {
    fn build_ok(data: impl Serialize) -> AppResult {
        Ok(Json(AppResponse {
            code: 0,
            msg: None,
            data: Some(data),
        })
        .into_response())
    }

    fn build_err(msg: impl Into<String>) -> AppResult {
        let msg: String = msg.into();

        Err(AppError {
            err: anyhow!("{}", msg),
            code: 1,
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })
    }

    fn build_err_full(
        msg: impl Into<String>,
        code: Option<u32>,
        status_code: Option<StatusCode>,
    ) -> AppResult {
        let msg: String = msg.into();

        Err(AppError {
            err: anyhow!("{}", msg),
            code: code.unwrap_or(1),
            status_code: status_code.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
        })
    }
}

/// 应用自定义错误, 主要目的是让 anyhow::Error 支持 IntoResponse
pub struct AppError {
    /// anyhow::Error
    pub err: anyhow::Error,
    /// 自定义错误码(默认1)
    pub code: u32,
    /// http状态码(默认500)
    pub status_code: StatusCode,
}

/// 为错误实现 IntoResponse
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            self.status_code,
            Json(AppResponse::<()> {
                code: self.code,
                msg: Some(self.err.to_string()),
                data: None,
            }),
        )
            .into_response()
    }
}

/// 转换 `Result<_, anyhow::Error>` 为 `Result<_, AppError>`, 使其支持 ? 操作符
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self {
            err: err.into(),
            code: 1,
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
