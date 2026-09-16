-- ==========================================================
-- SKEMA DATABASE: APLIKASI PENGADUAN SARANA SEKOLAH
-- UJI KOMPETENSI KEAHLIAN (UKK) RPL 2025/2026
-- ==========================================================

-- 1. Tabel Kategori Sarana
CREATE TABLE IF NOT EXISTS kategori (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nama_kategori TEXT NOT NULL UNIQUE,
    deskripsi TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 2. Tabel Pengguna (Admin, Petugas, Siswa)
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    nama_lengkap TEXT NOT NULL,
    role TEXT NOT NULL CHECK(role IN ('admin', 'petugas', 'siswa')),
    nisn TEXT,
    kelas TEXT,
    no_telp TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 3. Tabel Aspirasi / Pengaduan Sarana
CREATE TABLE IF NOT EXISTS aspirasi (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    kode_tiket TEXT NOT NULL UNIQUE,
    nisn TEXT NOT NULL,
    nama_siswa TEXT NOT NULL,
    kelas TEXT NOT NULL,
    id_kategori INTEGER NOT NULL,
    lokasi_sarana TEXT NOT NULL,
    deskripsi TEXT NOT NULL,
    foto_url TEXT,
    status TEXT NOT NULL DEFAULT 'menunggu' CHECK(status IN ('menunggu', 'diproses', 'selesai', 'ditolak')),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (id_kategori) REFERENCES kategori(id) ON DELETE RESTRICT
);

-- 4. Tabel Umpan Balik / Tanggapan Aspirasi
CREATE TABLE IF NOT EXISTS umpan_balik (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    id_aspirasi INTEGER NOT NULL,
    id_petugas INTEGER,
    nama_petugas TEXT NOT NULL,
    tanggapan TEXT NOT NULL,
    progres_persen INTEGER NOT NULL DEFAULT 0 CHECK(progres_persen >= 0 AND progres_persen <= 100),
    foto_perbaikan TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (id_aspirasi) REFERENCES aspirasi(id) ON DELETE CASCADE,
    FOREIGN KEY (id_petugas) REFERENCES users(id) ON DELETE SET NULL
);

-- 5. Tabel Histori / Log Status Aspirasi
CREATE TABLE IF NOT EXISTS histori_status (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    id_aspirasi INTEGER NOT NULL,
    status_lama TEXT NOT NULL,
    status_baru TEXT NOT NULL,
    catatan TEXT,
    diubah_oleh TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (id_aspirasi) REFERENCES aspirasi(id) ON DELETE CASCADE
);

-- Indeks untuk pencarian dan filter cepat (Best Practice Query Efisien)
CREATE INDEX IF NOT EXISTS idx_aspirasi_nisn ON aspirasi(nisn);
CREATE INDEX IF NOT EXISTS idx_aspirasi_kategori ON aspirasi(id_kategori);
CREATE INDEX IF NOT EXISTS idx_aspirasi_status ON aspirasi(status);
CREATE INDEX IF NOT EXISTS idx_aspirasi_created_at ON aspirasi(created_at);
CREATE INDEX IF NOT EXISTS idx_umpan_balik_aspirasi ON umpan_balik(id_aspirasi);
CREATE INDEX IF NOT EXISTS idx_histori_aspirasi ON histori_status(id_aspirasi);
