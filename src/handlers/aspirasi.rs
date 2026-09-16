use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use crate::{
    db::{self, DbPool},
    models::{ApiResponse, Aspirasi, CreateAspirasiRequest, FilterAspirasiQuery, UpdateStatusRequest},
};

/// Handler untuk siswa membuat pengaduan sarana baru
pub async fn create_aspirasi_handler(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateAspirasiRequest>,
) -> (StatusCode, Json<ApiResponse<Aspirasi>>) {
    // Validasi input
    if payload.nisn.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                message: "NISN wajib diisi".to_string(),
                data: None,
            }),
        );
    }
    if payload.nama_siswa.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                message: "Nama siswa wajib diisi".to_string(),
                data: None,
            }),
        );
    }
    if payload.deskripsi.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                message: "Deskripsi pengaduan wajib diisi".to_string(),
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
                    message: format!("Gagal menghubungi database: {}", e),
                    data: None,
                }),
            );
        }
    };

    match db::create_aspirasi(&conn, &payload) {
        Ok(aspirasi) => (
            StatusCode::CREATED,
            Json(ApiResponse {
                success: true,
                message: format!("Aspirasi berhasil dikirim! Kode Tiket: {}", aspirasi.kode_tiket),
                data: Some(aspirasi),
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                message: format!("Gagal menyimpan aspirasi: {}", e),
                data: None,
            }),
        ),
    }
}

/// Handler untuk mengambil list aspirasi dengan multi-filter (admin & siswa)
pub async fn get_aspirasi_list_handler(
    State(pool): State<DbPool>,
    Query(filter): Query<FilterAspirasiQuery>,
) -> (StatusCode, Json<ApiResponse<Vec<Aspirasi>>>) {
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

    match db::get_aspirasi_list(&conn, &filter) {
        Ok(list) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                message: "Data aspirasi berhasil dimuat".to_string(),
                data: Some(list),
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                message: format!("Gagal mengambil data aspirasi: {}", e),
                data: None,
            }),
        ),
    }
}

/// Handler untuk mengambil detail aspirasi beserta riwayat umpan balik & timeline status
pub async fn get_aspirasi_detail_handler(
    State(pool): State<DbPool>,
    Path(id): Path<i64>,
) -> (StatusCode, Json<ApiResponse<Aspirasi>>) {
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

    match db::get_aspirasi_by_id(&conn, id) {
        Ok(aspirasi) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                message: "Detail aspirasi berhasil dimuat".to_string(),
                data: Some(aspirasi),
            }),
        ),
        Err(rusqlite::Error::QueryReturnedNoRows) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                message: "Aspirasi tidak ditemukan".to_string(),
                data: None,
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                message: format!("Gagal mengambil data: {}", e),
                data: None,
            }),
        ),
    }
}

/// Handler untuk mengubah status aspirasi secara manual (Admin/Petugas)
pub async fn update_aspirasi_status_handler(
    State(pool): State<DbPool>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateStatusRequest>,
) -> (StatusCode, Json<ApiResponse<bool>>) {
    let valid_statuses = ["menunggu", "diproses", "selesai", "ditolak"];
    if !valid_statuses.contains(&payload.status.as_str()) {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                message: "Status tidak valid. Pilihan: menunggu, diproses, selesai, ditolak".to_string(),
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

    match db::update_status_aspirasi(&conn, id, &payload.status, payload.catatan.as_deref(), &payload.diubah_oleh) {
        Ok(true) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                message: format!("Status berhasil diubah menjadi '{}'", payload.status),
                data: Some(true),
            }),
        ),
        Ok(false) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                message: "Aspirasi tidak ditemukan".to_string(),
                data: None,
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                message: format!("Gagal mengubah status: {}", e),
                data: None,
            }),
        ),
    }
}
