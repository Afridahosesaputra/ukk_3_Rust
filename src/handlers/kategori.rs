use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use crate::{
    db::{self, DbPool},
    models::{ApiResponse, CreateKategoriRequest, Kategori},
};

/// Handler untuk mengambil seluruh kategori fasilitas
pub async fn get_kategori_list_handler(
    State(pool): State<DbPool>,
) -> (StatusCode, Json<ApiResponse<Vec<Kategori>>>) {
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

    match db::get_all_kategori(&conn) {
        Ok(list) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                message: "Data kategori berhasil dimuat".to_string(),
                data: Some(list),
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                message: format!("Gagal mengambil kategori: {}", e),
                data: None,
            }),
        ),
    }
}

/// Handler untuk menambah kategori fasilitas baru
pub async fn create_kategori_handler(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateKategoriRequest>,
) -> (StatusCode, Json<ApiResponse<i64>>) {
    if payload.nama_kategori.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                message: "Nama kategori tidak boleh kosong".to_string(),
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

    match db::create_kategori(&conn, &payload) {
        Ok(id) => (
            StatusCode::CREATED,
            Json(ApiResponse {
                success: true,
                message: "Kategori baru berhasil ditambahkan".to_string(),
                data: Some(id),
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                message: format!("Gagal menambahkan kategori (mungkin nama sudah ada): {}", e),
                data: None,
            }),
        ),
    }
}

/// Handler untuk mengubah kategori fasilitas
pub async fn update_kategori_handler(
    State(pool): State<DbPool>,
    Path(id): Path<i64>,
    Json(payload): Json<CreateKategoriRequest>,
) -> (StatusCode, Json<ApiResponse<bool>>) {
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

    match db::update_kategori(&conn, id, &payload) {
        Ok(true) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                message: "Kategori berhasil diperbarui".to_string(),
                data: Some(true),
            }),
        ),
        Ok(false) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                message: "Kategori tidak ditemukan".to_string(),
                data: None,
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                message: format!("Gagal memperbarui kategori: {}", e),
                data: None,
            }),
        ),
    }
}

/// Handler untuk menghapus kategori fasilitas
pub async fn delete_kategori_handler(
    State(pool): State<DbPool>,
    Path(id): Path<i64>,
) -> (StatusCode, Json<ApiResponse<bool>>) {
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

    match db::delete_kategori(&conn, id) {
        Ok(true) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                message: "Kategori berhasil dihapus".to_string(),
                data: Some(true),
            }),
        ),
        Ok(false) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                message: "Kategori tidak ditemukan".to_string(),
                data: None,
            }),
        ),
        Err(e) => (
            StatusCode::CONFLICT,
            Json(ApiResponse {
                success: false,
                message: format!("Kategori tidak dapat dihapus karena masih digunakan pada pengaduan: {}", e),
                data: None,
            }),
        ),
    }
}
