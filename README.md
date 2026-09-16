# Aplikasi Pengaduan Sarana Sekolah (SARPRAS CARE)
### Uji Kompetensi Keahlian (UKK) Rekayasa Perangkat Lunak 2025/2026
**Kode Soal: KM25.4.1.1 | Bahasa Pemrograman: Rust**

---

## 📌 Deskripsi Singkat
Aplikasi **Pengaduan Sarana Sekolah** berbasis web berkinerja tinggi yang dibangun menggunakan bahasa pemrograman **Rust** dengan framework web **Axum** dan database **SQLite**.

Aplikasi ini mencakup dua portal utama:
1. **Portal Siswa**: Formulir pengaduan kerusakan sarana sekolah, penomoran kode tiket otomatis, pelacakan status real-time (*Menunggu*, *Diproses*, *Selesai*, *Ditolak*), dan histori pengaduan per siswa.
2. **Portal Admin / Petugas**: Dashboard statistik, daftar aspirasi dengan multi-filter (**Per Tanggal**, **Per Bulan**, **Per Siswa/NISN**, **Per Kategori**, **Per Status**), form umpan balik/tanggapan, slider persentase progres perbaikan fisik (0-100%), histori audit log, manajemen kategori sarana, dan cetak laporan resmi lengkap dengan kop surat sekolah.

---

## 🛠️ Teknologi yang Digunakan
- **Bahasa Pemrograman**: Rust (Edition 2021)
- **Web Framework**: Axum (v0.7) + Tokio Runtime
- **Database**: SQLite (Rusqlite + R2D2 Connection Pooling)
- **Frontend**: Vanilla HTML5, Modern CSS3 (Glassmorphism, Dark/Light Mode), Vanilla JavaScript
- **Serialization**: Serde & Serde JSON

---

## 🚀 Cara Menjalankan Aplikasi
1. Pastikan Rust toolchain telah terpasang:
   ```bash
   rustc --version
   cargo --version
   ```
2. Jalankan server aplikasi:
   ```bash
   cargo run
   ```
3. Buka browser dan akses:
   ```text
   http://localhost:3000
   ```

### 🔑 Kredensial Demo Login:
- **Administrator**: `admin` / `admin123`
- **Petugas Sarana**: `petugas1` / `petugas123`

---

## 📚 Dokumentasi Lengkap
- [Panduan Pembuatan Lengkap (Step-by-Step Tutorial)](PANDUAN_PEMBUATAN_LENGKAP.md)
- [Dokumentasi Resmi Proyek UKK (ERD, Deskripsi, Kasus Uji, Evaluasi)](DOKUMENTASI_PROYEK_UKK.md)
- [Skema Database SQL](database/schema.sql)
