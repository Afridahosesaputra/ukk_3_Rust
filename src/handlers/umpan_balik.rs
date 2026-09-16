use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use crate::{
    db::{self, DbPool},
    models::{ApiResponse, CreateUmpanBalikRequest, UmpanBalik},
};

/// Handler untuk memberikan umpan balik dan update progres perbaikan (Admin/Petugas)
pub async fn create_umpan_balik_handler(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateUmpanBalikRequest>,
) -> (StatusCode, Json<ApiResponse<UmpanBalik>>) {
    if payload.tanggapan.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                message: "Tanggapan umpan balik wajib diisi".to_string(),
                data: None,
            }),
        );
    }

    if payload.progres_persen < 0 || payload.progres_persen > 100 {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                message: "Persentase progres harus antara 0 sampai 100".to_string(),
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

    match db::create_umpan_balik(&conn, &payload) {
        Ok(ub) => (
            StatusCode::CREATED,
            Json(ApiResponse {
                success: true,
                message: "Umpan balik berhasil dikirim".to_string(),
                data: Some(ub),
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                message: format!("Gagal menyimpan umpan balik: {}", e),
                data: None,
            }),
        ),
    }
}

/// Handler untuk mengambil daftar umpan balik berdasarkan ID Aspirasi
pub async fn get_umpan_balik_by_aspirasi_handler(
    State(pool): State<DbPool>,
    Path(id_aspirasi): Path<i64>,
) -> (StatusCode, Json<ApiResponse<Vec<UmpanBalik>>>) {
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

    match db::get_umpan_balik_by_aspirasi_id(&conn, id_aspirasi) {
        Ok(list) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                message: "Data umpan balik berhasil dimuat".to_string(),
                data: Some(list),
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                message: format!("Gagal mengambil data umpan balik: {}", e),
                data: None,
            }),
        ),
    }
}
