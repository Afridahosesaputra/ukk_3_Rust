use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Connection, Result};
use std::sync::Arc;
use chrono::Local;

use crate::models::*;

pub type DbPool = Arc<Pool<SqliteConnectionManager>>;

/// Inisialisasi Database Pool SQLite dan Migrasi Skema
pub fn init_db(database_url: &str) -> DbPool {
    let manager = SqliteConnectionManager::file(database_url);
    let pool = Pool::new(manager).expect("Gagal membuat koneksi database pool SQLite");

    // Jalankan skema awal jika tabel belum ada
    let conn = pool.get().expect("Gagal mengambil koneksi dari pool");
    setup_tables(&conn).expect("Gagal menginisialisasi tabel database");

    Arc::new(pool)
}

/// Menyiapkan struktur tabel dan seed data awal jika database masih kosong
fn setup_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS kategori (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            nama_kategori TEXT NOT NULL UNIQUE,
            deskripsi TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

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

        CREATE INDEX IF NOT EXISTS idx_aspirasi_nisn ON aspirasi(nisn);
        CREATE INDEX IF NOT EXISTS idx_aspirasi_kategori ON aspirasi(id_kategori);
        CREATE INDEX IF NOT EXISTS idx_aspirasi_status ON aspirasi(status);
        CREATE INDEX IF NOT EXISTS idx_aspirasi_created_at ON aspirasi(created_at);
        CREATE INDEX IF NOT EXISTS idx_umpan_balik_aspirasi ON umpan_balik(id_aspirasi);
        CREATE INDEX IF NOT EXISTS idx_histori_aspirasi ON histori_status(id_aspirasi);
        "
    )?;

    // Periksa apakah tabel kategori kosong, jika kosong isi seed data
    let mut count_stmt = conn.prepare("SELECT COUNT(*) FROM kategori")?;
    let count: i64 = count_stmt.query_row([], |row| row.get(0))?;

    if count == 0 {
        conn.execute_batch(
            "
            INSERT OR IGNORE INTO kategori (id, nama_kategori, deskripsi) VALUES
            (1, 'Ruang Kelas & Perabot', 'Meja, kursi siswa/guru, papan tulis, lemari, pintu, jendela kelas'),
            (2, 'Kelistrikan & Elektronik', 'Lampu penerangan, stop kontak, saklar, kipas angin, AC pendingin ruang'),
            (3, 'Laboratorium & Komputer', 'PC/Laptop lab, LCD proyektor, kabel VGA/HDMI, headset, jaringan internet/LAN'),
            (4, 'Sanitasi & Toilet', 'Kloset, keran air, wastafel, saluran pembuangan, pintu toilet, bak air'),
            (5, 'Olahraga & Lapangan', 'Tiang basket, gawang futsal, net voli/badminton, lapangan retak/licin'),
            (6, 'Perpustakaan & Sarana Lain', 'Rak buku, karpet ruang baca, AC perpus, tempat sampah, koridor/taman');

            INSERT OR IGNORE INTO users (id, username, password, nama_lengkap, role, nisn, kelas, no_telp) VALUES
            (1, 'admin', 'admin123', 'Administrator Sarpras', 'admin', NULL, NULL, '081234567890'),
            (2, 'petugas1', 'petugas123', 'Budi Santoso, S.Pd (Petugas Sarana)', 'petugas', NULL, NULL, '082345678901'),
            (3, '0061234567', 'siswa123', 'Ahmad Rizky Pratama', 'siswa', '0061234567', 'XII RPL 1', '083456789012');

            INSERT OR IGNORE INTO aspirasi (id, kode_tiket, nisn, nama_siswa, kelas, id_kategori, lokasi_sarana, deskripsi, foto_url, status, created_at, updated_at) VALUES
            (1, 'ASP-2026-001', '0061234567', 'Ahmad Rizky Pratama', 'XII RPL 1', 3, 'Lab Komputer RPL 2', 'LCD Proyektor di Lab RPL 2 warna tampilannya menguning dan sering berkedip saat presentasi.', '', 'diproses', '2026-09-10 08:30:00', '2026-09-11 10:15:00'),
            (2, 'ASP-2026-002', '0067654321', 'Siti Nurhaliza', 'XII RPL 2', 4, 'Toilet Siswi Lantai 2 Gedung B', 'Keran air di wastafel toilet siswi bocor sehingga air terus menetes dan membasahi lantai.', '', 'selesai', '2026-09-08 09:15:00', '2026-09-09 14:00:00'),
            (3, 'ASP-2026-003', '0069988776', 'Dimas Arya Saputra', 'XI TKJ 1', 1, 'Ruang Kelas XI TKJ 1', 'Terdapat 3 meja siswa yang kayunya sudah retak dan goyang, membahayakan saat dipakai belajar.', '', 'menunggu', '2026-09-15 13:45:00', '2026-09-15 13:45:00'),
            (4, 'ASP-2026-004', '0061122334', 'Putri Ayu Anggraini', 'X DKV 2', 2, 'Ruang Kelas X DKV 2', 'AC pendingin ruangan tidak dingin hanya mengeluarkan angin biasa.', '', 'menunggu', '2026-09-16 08:10:00', '2026-09-16 08:10:00');

            INSERT OR IGNORE INTO umpan_balik (id, id_aspirasi, id_petugas, nama_petugas, tanggapan, progres_persen, foto_perbaikan, created_at) VALUES
            (1, 1, 2, 'Budi Santoso, S.Pd', 'Kabel HDMI dan filter lampu proyektor telah dicek. Sedang dipesan kabel pengganti dan pembersihan lensa.', 50, '', '2026-09-11 10:15:00'),
            (2, 2, 2, 'Budi Santoso, S.Pd', 'Klep keran air wastafel sudah diganti dengan unit baru. Aliran air sudah normal dan tidak bocor lagi.', 100, '', '2026-09-09 14:00:00');

            INSERT OR IGNORE INTO histori_status (id, id_aspirasi, status_lama, status_baru, catatan, diubah_oleh, created_at) VALUES
            (1, 1, 'menunggu', 'diproses', 'Pengaduan diverifikasi teknisi sarpras', 'Budi Santoso, S.Pd', '2026-09-11 10:15:00'),
            (2, 2, 'menunggu', 'diproses', 'Petugas pemeliharaan menuju lokasi', 'Budi Santoso, S.Pd', '2026-09-08 11:00:00'),
            (3, 2, 'diproses', 'selesai', 'Pekerjaan perbaikan keran selesai dilakukan', 'Budi Santoso, S.Pd', '2026-09-09 14:00:00');
            "
        )?;
    }

    Ok(())
}

// ==========================================
// OPERASI KATEGORI SARANA
// ==========================================

pub fn get_all_kategori(conn: &Connection) -> Result<Vec<Kategori>> {
    let mut stmt = conn.prepare("SELECT id, nama_kategori, deskripsi, created_at FROM kategori ORDER BY id ASC")?;
    let rows = stmt.query_map([], |row| {
        Ok(Kategori {
            id: row.get(0)?,
            nama_kategori: row.get(1)?,
            deskripsi: row.get(2)?,
            created_at: row.get(3)?,
        })
    })?;

    let mut result = Vec::new();
    for item in rows {
        result.push(item?);
    }
    Ok(result)
}

pub fn create_kategori(conn: &Connection, req: &CreateKategoriRequest) -> Result<i64> {
    conn.execute(
        "INSERT INTO kategori (nama_kategori, deskripsi) VALUES (?1, ?2)",
        params![req.nama_kategori, req.deskripsi],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn update_kategori(conn: &Connection, id: i64, req: &CreateKategoriRequest) -> Result<bool> {
    let count = conn.execute(
        "UPDATE kategori SET nama_kategori = ?1, deskripsi = ?2 WHERE id = ?3",
        params![req.nama_kategori, req.deskripsi, id],
    )?;
    Ok(count > 0)
}

pub fn delete_kategori(conn: &Connection, id: i64) -> Result<bool> {
    let count = conn.execute("DELETE FROM kategori WHERE id = ?1", params![id])?;
    Ok(count > 0)
}

// ==========================================
// OPERASI ASPIRASI / PENGADUAN
// ==========================================

pub fn generate_kode_tiket(conn: &Connection) -> Result<String> {
    let year = Local::now().format("%Y").to_string();
    let prefix = format!("ASP-{}-%", year);
    
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM aspirasi WHERE kode_tiket LIKE ?1")?;
    let count: i64 = stmt.query_row(params![prefix], |row| row.get(0))?;
    
    let sequence = count + 1;
    Ok(format!("ASP-{}-{:03}", year, sequence))
}

pub fn create_aspirasi(conn: &Connection, req: &CreateAspirasiRequest) -> Result<Aspirasi> {
    let kode_tiket = generate_kode_tiket(conn)?;
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    conn.execute(
        "INSERT INTO aspirasi (kode_tiket, nisn, nama_siswa, kelas, id_kategori, lokasi_sarana, deskripsi, foto_url, status, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 'menunggu', ?9, ?9)",
        params![
            kode_tiket,
            req.nisn,
            req.nama_siswa,
            req.kelas,
            req.id_kategori,
            req.lokasi_sarana,
            req.deskripsi,
            req.foto_url.clone().unwrap_or_default(),
            now
        ],
    )?;

    let id = conn.last_insert_rowid();

    // Catat histori awal
    conn.execute(
        "INSERT INTO histori_status (id_aspirasi, status_lama, status_baru, catatan, diubah_oleh, created_at)
         VALUES (?1, 'baru', 'menunggu', 'Pengaduan sarana diajukan oleh siswa', ?2, ?3)",
        params![id, req.nama_siswa, now],
    )?;

    get_aspirasi_by_id(conn, id)
}

pub fn get_aspirasi_by_id(conn: &Connection, id: i64) -> Result<Aspirasi> {
    let mut stmt = conn.prepare(
        "SELECT a.id, a.kode_tiket, a.nisn, a.nama_siswa, a.kelas, a.id_kategori, k.nama_kategori,
                a.lokasi_sarana, a.deskripsi, a.foto_url, a.status, a.created_at, a.updated_at
         FROM aspirasi a
         LEFT JOIN kategori k ON a.id_kategori = k.id
         WHERE a.id = ?1",
    )?;

    let mut aspirasi = stmt.query_row(params![id], |row| {
        Ok(Aspirasi {
            id: row.get(0)?,
            kode_tiket: row.get(1)?,
            nisn: row.get(2)?,
            nama_siswa: row.get(3)?,
            kelas: row.get(4)?,
            id_kategori: row.get(5)?,
            nama_kategori: row.get(6)?,
            lokasi_sarana: row.get(7)?,
            deskripsi: row.get(8)?,
            foto_url: row.get(9)?,
            status: row.get(10)?,
            created_at: row.get(11)?,
            updated_at: row.get(12)?,
            progres_terakhir: None,
            umpan_balik_list: None,
            histori_list: None,
        })
    })?;

    // Ambil Umpan Balik & Histori
    aspirasi.umpan_balik_list = Some(get_umpan_balik_by_aspirasi_id(conn, aspirasi.id)?);
    aspirasi.histori_list = Some(get_histori_by_aspirasi_id(conn, aspirasi.id)?);

    // Ambil progres persen terakhir
    if let Some(ref ub_list) = aspirasi.umpan_balik_list {
        if let Some(last_ub) = ub_list.last() {
            aspirasi.progres_terakhir = Some(last_ub.progres_persen);
        } else if aspirasi.status == "selesai" {
            aspirasi.progres_terakhir = Some(100);
        } else if aspirasi.status == "diproses" {
            aspirasi.progres_terakhir = Some(50);
        } else {
            aspirasi.progres_terakhir = Some(0);
        }
    }

    Ok(aspirasi)
}

pub fn get_aspirasi_list(conn: &Connection, filter: &FilterAspirasiQuery) -> Result<Vec<Aspirasi>> {
    let mut sql = String::from(
        "SELECT a.id, a.kode_tiket, a.nisn, a.nama_siswa, a.kelas, a.id_kategori, k.nama_kategori,
                a.lokasi_sarana, a.deskripsi, a.foto_url, a.status, a.created_at, a.updated_at,
                COALESCE((SELECT progres_persen FROM umpan_balik WHERE id_aspirasi = a.id ORDER BY id DESC LIMIT 1), 
                         CASE WHEN a.status = 'selesai' THEN 100 WHEN a.status = 'diproses' THEN 50 ELSE 0 END) AS progres
         FROM aspirasi a
         LEFT JOIN kategori k ON a.id_kategori = k.id
         WHERE 1=1"
    );

    let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(ref status) = filter.status {
        if !status.is_empty() && status != "semua" {
            sql.push_str(" AND a.status = ?");
            params_vec.push(Box::new(status.clone()));
        }
    }

    if let Some(id_kategori) = filter.id_kategori {
        if id_kategori > 0 {
            sql.push_str(" AND a.id_kategori = ?");
            params_vec.push(Box::new(id_kategori));
        }
    }

    if let Some(ref nisn) = filter.nisn {
        if !nisn.is_empty() {
            sql.push_str(" AND a.nisn = ?");
            params_vec.push(Box::new(nisn.clone()));
        }
    }

    if let Some(ref bulan) = filter.bulan {
        if !bulan.is_empty() {
            sql.push_str(" AND strftime('%Y-%m', a.created_at) = ?");
            params_vec.push(Box::new(bulan.clone()));
        }
    }

    if let Some(ref tgl_mulai) = filter.tanggal_mulai {
        if !tgl_mulai.is_empty() {
            sql.push_str(" AND date(a.created_at) >= date(?)");
            params_vec.push(Box::new(tgl_mulai.clone()));
        }
    }

    if let Some(ref tgl_selesai) = filter.tanggal_selesai {
        if !tgl_selesai.is_empty() {
            sql.push_str(" AND date(a.created_at) <= date(?)");
            params_vec.push(Box::new(tgl_selesai.clone()));
        }
    }

    if let Some(ref search) = filter.search {
        if !search.is_empty() {
            sql.push_str(" AND (a.nama_siswa LIKE ? OR a.kode_tiket LIKE ? OR a.nisn LIKE ? OR a.lokasi_sarana LIKE ? OR a.deskripsi LIKE ?)");
            let wildcard = format!("%{}%", search);
            params_vec.push(Box::new(wildcard.clone()));
            params_vec.push(Box::new(wildcard.clone()));
            params_vec.push(Box::new(wildcard.clone()));
            params_vec.push(Box::new(wildcard.clone()));
            params_vec.push(Box::new(wildcard.clone()));
        }
    }

    sql.push_str(" ORDER BY a.created_at DESC");

    let mut stmt = conn.prepare(&sql)?;
    let param_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|b| b.as_ref()).collect();

    let rows = stmt.query_map(param_refs.as_slice(), |row| {
        Ok(Aspirasi {
            id: row.get(0)?,
            kode_tiket: row.get(1)?,
            nisn: row.get(2)?,
            nama_siswa: row.get(3)?,
            kelas: row.get(4)?,
            id_kategori: row.get(5)?,
            nama_kategori: row.get(6)?,
            lokasi_sarana: row.get(7)?,
            deskripsi: row.get(8)?,
            foto_url: row.get(9)?,
            status: row.get(10)?,
            created_at: row.get(11)?,
            updated_at: row.get(12)?,
            progres_terakhir: row.get(13)?,
            umpan_balik_list: None,
            histori_list: None,
        })
    })?;

    let mut result = Vec::new();
    for item in rows {
        result.push(item?);
    }
    Ok(result)
}

pub fn update_status_aspirasi(
    conn: &Connection,
    id: i64,
    status_baru: &str,
    catatan: Option<&str>,
    diubah_oleh: &str,
) -> Result<bool> {
    // Ambil status lama
    let mut status_stmt = conn.prepare("SELECT status FROM aspirasi WHERE id = ?1")?;
    let status_lama: String = match status_stmt.query_row(params![id], |row| row.get(0)) {
        Ok(val) => val,
        Err(_) => return Ok(false),
    };

    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    conn.execute(
        "UPDATE aspirasi SET status = ?1, updated_at = ?2 WHERE id = ?3",
        params![status_baru, now, id],
    )?;

    // Catat histori perubahan status
    conn.execute(
        "INSERT INTO histori_status (id_aspirasi, status_lama, status_baru, catatan, diubah_oleh, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![id, status_lama, status_baru, catatan.unwrap_or(""), diubah_oleh, now],
    )?;

    Ok(true)
}

// ==========================================
// OPERASI UMPAN BALIK & HISTORI
// ==========================================

pub fn create_umpan_balik(conn: &Connection, req: &CreateUmpanBalikRequest) -> Result<UmpanBalik> {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    conn.execute(
        "INSERT INTO umpan_balik (id_aspirasi, id_petugas, nama_petugas, tanggapan, progres_persen, foto_perbaikan, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            req.id_aspirasi,
            req.id_petugas,
            req.nama_petugas,
            req.tanggapan,
            req.progres_persen,
            req.foto_perbaikan.clone().unwrap_or_default(),
            now
        ],
    )?;

    let id = conn.last_insert_rowid();

    // Jika ada request untuk mengubah status aspirasi secara otomatis
    if let Some(ref status_baru) = req.ubah_status_ke {
        if !status_baru.is_empty() {
            let catatan = format!("Umpan balik ditambahkan: {} (Progres {}%)", req.tanggapan, req.progres_persen);
            let _ = update_status_aspirasi(conn, req.id_aspirasi, status_baru, Some(&catatan), &req.nama_petugas);
        }
    } else {
        // Otomatis ubah status berdasarkan progres persen jika tidak ditentukan
        let auto_status = if req.progres_persen >= 100 {
            Some("selesai")
        } else if req.progres_persen > 0 {
            Some("diproses")
        } else {
            None
        };

        if let Some(status_baru) = auto_status {
            let catatan = format!("Progres pengerjaan mencapai {}%", req.progres_persen);
            let _ = update_status_aspirasi(conn, req.id_aspirasi, status_baru, Some(&catatan), &req.nama_petugas);
        }
    }

    Ok(UmpanBalik {
        id,
        id_aspirasi: req.id_aspirasi,
        id_petugas: req.id_petugas,
        nama_petugas: req.nama_petugas.clone(),
        tanggapan: req.tanggapan.clone(),
        progres_persen: req.progres_persen,
        foto_perbaikan: req.foto_perbaikan.clone(),
        created_at: now,
    })
}

pub fn get_umpan_balik_by_aspirasi_id(conn: &Connection, id_aspirasi: i64) -> Result<Vec<UmpanBalik>> {
    let mut stmt = conn.prepare(
        "SELECT id, id_aspirasi, id_petugas, nama_petugas, tanggapan, progres_persen, foto_perbaikan, created_at
         FROM umpan_balik
         WHERE id_aspirasi = ?1
         ORDER BY id ASC",
    )?;

    let rows = stmt.query_map(params![id_aspirasi], |row| {
        Ok(UmpanBalik {
            id: row.get(0)?,
            id_aspirasi: row.get(1)?,
            id_petugas: row.get(2)?,
            nama_petugas: row.get(3)?,
            tanggapan: row.get(4)?,
            progres_persen: row.get(5)?,
            foto_perbaikan: row.get(6)?,
            created_at: row.get(7)?,
        })
    })?;

    let mut result = Vec::new();
    for item in rows {
        result.push(item?);
    }
    Ok(result)
}

pub fn get_histori_by_aspirasi_id(conn: &Connection, id_aspirasi: i64) -> Result<Vec<HistoriStatus>> {
    let mut stmt = conn.prepare(
        "SELECT id, id_aspirasi, status_lama, status_baru, catatan, diubah_oleh, created_at
         FROM histori_status
         WHERE id_aspirasi = ?1
         ORDER BY id ASC",
    )?;

    let rows = stmt.query_map(params![id_aspirasi], |row| {
        Ok(HistoriStatus {
            id: row.get(0)?,
            id_aspirasi: row.get(1)?,
            status_lama: row.get(2)?,
            status_baru: row.get(3)?,
            catatan: row.get(4)?,
            diubah_oleh: row.get(5)?,
            created_at: row.get(6)?,
        })
    })?;

    let mut result = Vec::new();
    for item in rows {
        result.push(item?);
    }
    Ok(result)
}

// ==========================================
// DASHBOARD & STATISTIK
// ==========================================

pub fn get_dashboard_stats(conn: &Connection) -> Result<DashboardStats> {
    let total_aspirasi: i64 = conn.query_row("SELECT COUNT(*) FROM aspirasi", [], |r| r.get(0))?;
    let menunggu: i64 = conn.query_row("SELECT COUNT(*) FROM aspirasi WHERE status = 'menunggu'", [], |r| r.get(0))?;
    let diproses: i64 = conn.query_row("SELECT COUNT(*) FROM aspirasi WHERE status = 'diproses'", [], |r| r.get(0))?;
    let selesai: i64 = conn.query_row("SELECT COUNT(*) FROM aspirasi WHERE status = 'selesai'", [], |r| r.get(0))?;
    let ditolak: i64 = conn.query_row("SELECT COUNT(*) FROM aspirasi WHERE status = 'ditolak'", [], |r| r.get(0))?;
    let total_kategori: i64 = conn.query_row("SELECT COUNT(*) FROM kategori", [], |r| r.get(0))?;

    let mut stmt = conn.prepare(
        "SELECT k.nama_kategori, COUNT(a.id) AS jumlah
         FROM kategori k
         LEFT JOIN aspirasi a ON k.id = a.id_kategori
         GROUP BY k.id, k.nama_kategori
         ORDER BY jumlah DESC",
    )?;

    let k_rows = stmt.query_map([], |row| {
        Ok(KategoriStats {
            nama_kategori: row.get(0)?,
            jumlah: row.get(1)?,
        })
    })?;

    let mut kategori_stats = Vec::new();
    for k in k_rows {
        kategori_stats.push(k?);
    }

    Ok(DashboardStats {
        total_aspirasi,
        menunggu,
        diproses,
        selesai,
        ditolak,
        total_kategori,
        kategori_stats,
    })
}

// ==========================================
// AUTENTIKASI PENGGUNA
// ==========================================

pub fn authenticate_user(conn: &Connection, req: &LoginRequest) -> Result<Option<User>> {
    let mut stmt = conn.prepare(
        "SELECT id, username, password, nama_lengkap, role, nisn, kelas, no_telp, created_at
         FROM users
         WHERE username = ?1",
    )?;

    let user_opt = stmt.query_row(params![req.username], |row| {
        let stored_password: String = row.get(2)?;
        if stored_password == req.password {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                nama_lengkap: row.get(3)?,
                role: row.get(4)?,
                nisn: row.get(5)?,
                kelas: row.get(6)?,
                no_telp: row.get(7)?,
                created_at: row.get(8)?,
            })
        } else {
            Err(rusqlite::Error::QueryReturnedNoRows)
        }
    });

    match user_opt {
        Ok(user) => Ok(Some(user)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}
