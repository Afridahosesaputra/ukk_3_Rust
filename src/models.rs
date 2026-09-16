use serde::{Deserialize, Serialize};

// ==========================================
// MODEL ENTITAS DATA (DATABASE MODELS)
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kategori {
    pub id: i64,
    pub nama_kategori: String,
    pub deskripsi: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub role: String, // 'admin', 'petugas', 'siswa'
    pub nama_lengkap: String,
    pub nisn: Option<String>,
    pub kelas: Option<String>,
    pub no_telp: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aspirasi {
    pub id: i64,
    pub kode_tiket: String,
    pub nisn: String,
    pub nama_siswa: String,
    pub kelas: String,
    pub id_kategori: i64,
    pub nama_kategori: Option<String>,
    pub lokasi_sarana: String,
    pub deskripsi: String,
    pub foto_url: Option<String>,
    pub status: String, // 'menunggu', 'diproses', 'selesai', 'ditolak'
    pub created_at: String,
    pub updated_at: String,
    pub progres_terakhir: Option<i64>,
    pub umpan_balik_list: Option<Vec<UmpanBalik>>,
    pub histori_list: Option<Vec<HistoriStatus>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UmpanBalik {
    pub id: i64,
    pub id_aspirasi: i64,
    pub id_petugas: Option<i64>,
    pub nama_petugas: String,
    pub tanggapan: String,
    pub progres_persen: i64,
    pub foto_perbaikan: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoriStatus {
    pub id: i64,
    pub id_aspirasi: i64,
    pub status_lama: String,
    pub status_baru: String,
    pub catatan: Option<String>,
    pub diubah_oleh: String,
    pub created_at: String,
}

// ==========================================
// REQUEST DTO (DATA TRANSFER OBJECTS)
// ==========================================

#[derive(Debug, Deserialize)]
pub struct CreateAspirasiRequest {
    pub nisn: String,
    pub nama_siswa: String,
    pub kelas: String,
    pub id_kategori: i64,
    pub lokasi_sarana: String,
    pub deskripsi: String,
    pub foto_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FilterAspirasiQuery {
    pub status: Option<String>,
    pub id_kategori: Option<i64>,
    pub nisn: Option<String>,
    pub search: Option<String>,
    pub tanggal_mulai: Option<String>,
    pub tanggal_selesai: Option<String>,
    pub bulan: Option<String>, // format YYYY-MM
}

#[derive(Debug, Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String,
    pub catatan: Option<String>,
    pub diubah_oleh: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUmpanBalikRequest {
    pub id_aspirasi: i64,
    pub id_petugas: Option<i64>,
    pub nama_petugas: String,
    pub tanggapan: String,
    pub progres_persen: i64,
    pub foto_perbaikan: Option<String>,
    pub ubah_status_ke: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateKategoriRequest {
    pub nama_kategori: String,
    pub deskripsi: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

// ==========================================
// RESPONSE DTO & STATISTIK
// ==========================================

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Debug, Serialize)]
pub struct KategoriStats {
    pub nama_kategori: String,
    pub jumlah: i64,
}

#[derive(Debug, Serialize)]
pub struct DashboardStats {
    pub total_aspirasi: i64,
    pub menunggu: i64,
    pub diproses: i64,
    pub selesai: i64,
    pub ditolak: i64,
    pub total_kategori: i64,
    pub kategori_stats: Vec<KategoriStats>,
}
