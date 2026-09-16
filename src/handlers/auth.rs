use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use crate::{
    db::{self, DbPool},
    models::{ApiResponse, LoginRequest, User},
};

/// Handler login untuk Admin, Petugas, dan Siswa
pub async fn login_handler(
    State(pool): State<DbPool>,
    Json(payload): Json<LoginRequest>,
) -> (StatusCode, Json<ApiResponse<User>>) {
    if payload.username.trim().is_empty() || payload.password.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                message: "Username dan password wajib diisi".to_string(),
                data: None,
            }),
        );
    }

    let conn = match pool.get() {
        Ok(c) => c,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    message: format!("Koneksi database gagal: {}", e),
                    data: None,
                }),
            );
        }
    };

    match db::authenticate_user(&conn, &payload) {
        Ok(Some(user)) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                message: format!("Login berhasil. Selamat datang, {}", user.nama_lengkap),
                data: Some(user),
            }),
        ),
        Ok(None) => (
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse {
                success: false,
                message: "Username atau password salah!".to_string(),
                data: None,
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                message: format!("Terjadi kesalahan saat otentikasi: {}", e),
                data: None,
            }),
        ),
    }
}
