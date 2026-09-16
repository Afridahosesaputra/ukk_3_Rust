use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use crate::{
    db::{self, DbPool},
    models::{ApiResponse, DashboardStats},
};

/// Handler untuk mengambil data ringkasan metrik dan statistik dashboard
pub async fn get_dashboard_stats_handler(
    State(pool): State<DbPool>,
) -> (StatusCode, Json<ApiResponse<DashboardStats>>) {
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

    match db::get_dashboard_stats(&conn) {
        Ok(stats) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                message: "Data statistik berhasil dimuat".to_string(),
                data: Some(stats),
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                message: format!("Gagal memuat data statistik: {}", e),
                data: None,
            }),
        ),
    }
}
