-- ==========================================================
-- DATA AWAL / SEED DATA: APLIKASI PENGADUAN SARANA SEKOLAH
-- ==========================================================

-- Data Kategori Fasilitas/Sarana
INSERT OR IGNORE INTO kategori (id, nama_kategori, deskripsi) VALUES
(1, 'Ruang Kelas & Perabot', 'Meja, kursi siswa/guru, papan tulis, lemari, pintu, jendela kelas'),
(2, 'Kelistrikan & Elektronik', 'Lampu penerangan, stop kontak, saklar, kipas angin, AC pendingin ruang'),
(3, 'Laboratorium & Komputer', 'PC/Laptop lab, LCD proyektor, kabel VGA/HDMI, headset, jaringan internet/LAN'),
(4, 'Sanitasi & Toilet', 'Kloset, keran air, wastafel, saluran pembuangan, pintu toilet, bak air'),
(5, 'Olahraga & Lapangan', 'Tiang basket, gawang futsal, net voli/badminton, lapangan retak/licin'),
(6, 'Perpustakaan & Sarana Lain', 'Rak buku, karpet ruang baca, AC perpus, tempat sampah, koridor/taman');

-- Data Akun Petugas/Admin
INSERT OR IGNORE INTO users (id, username, password, nama_lengkap, role, nisn, kelas, no_telp) VALUES
(1, 'admin', 'admin123', 'Administrator Sarpras', 'admin', NULL, NULL, '081234567890'),
(2, 'petugas1', 'petugas123', 'Budi Santoso, S.Pd (Petugas Sarana)', 'petugas', NULL, NULL, '082345678901'),
(3, '0061234567', 'siswa123', 'Ahmad Rizky Pratama', 'siswa', '0061234567', 'XII RPL 1', '083456789012');

-- Data Contoh Pengaduan Aspirasi Siswa
INSERT OR IGNORE INTO aspirasi (id, kode_tiket, nisn, nama_siswa, kelas, id_kategori, lokasi_sarana, deskripsi, foto_url, status, created_at, updated_at) VALUES
(1, 'ASP-2026-001', '0061234567', 'Ahmad Rizky Pratama', 'XII RPL 1', 3, 'Lab Komputer RPL 2', 'LCD Proyektor di Lab RPL 2 warna tampilannya menguning dan sering berkedip saat presentasi.', '', 'diproses', '2026-09-10 08:30:00', '2026-09-11 10:15:00'),
(2, 'ASP-2026-002', '0067654321', 'Siti Nurhaliza', 'XII RPL 2', 4, 'Toilet Siswi Lantai 2 Gedung B', 'Keran air di wastafel toilet siswi bocor sehingga air terus menetes dan membasahi lantai.', '', 'selesai', '2026-09-08 09:15:00', '2026-09-09 14:00:00'),
(3, 'ASP-2026-003', '0069988776', 'Dimas Arya Saputra', 'XI TKJ 1', 1, 'Ruang Kelas XI TKJ 1', 'Terdapat 3 meja siswa yang kayunya sudah retak dan goyang, membahayakan saat dipakai belajar.', '', 'menunggu', '2026-09-15 13:45:00', '2026-09-15 13:45:00'),
(4, 'ASP-2026-004', '0061122334', 'Putri Ayu Anggraini', 'X DKV 2', 2, 'Ruang Kelas X DKV 2', 'AC pendingin ruangan tidak dingin hanya mengeluarkan angin biasa.', '', 'menunggu', '2026-09-16 08:10:00', '2026-09-16 08:10:00');

-- Data Umpan Balik / Tanggapan
INSERT OR IGNORE INTO umpan_balik (id, id_aspirasi, id_petugas, nama_petugas, tanggapan, progres_persen, foto_perbaikan, created_at) VALUES
(1, 1, 2, 'Budi Santoso, S.Pd', 'Kabel HDMI dan filter lampu proyektor telah dicek. Sedang dipesan kabel pengganti dan pembersihan lensa.', 50, '', '2026-09-11 10:15:00'),
(2, 2, 2, 'Budi Santoso, S.Pd', 'Klep keran air wastafel sudah diganti dengan unit baru. Aliran air sudah normal dan tidak bocor lagi.', 100, '', '2026-09-09 14:00:00');

-- Data Histori Perubahan Status
INSERT OR IGNORE INTO histori_status (id, id_aspirasi, status_lama, status_baru, catatan, diubah_oleh, created_at) VALUES
(1, 1, 'menunggu', 'diproses', 'Pengaduan diverifikasi teknisi sarpras', 'Budi Santoso, S.Pd', '2026-09-11 10:15:00'),
(2, 2, 'menunggu', 'diproses', 'Petugas pemeliharaan menuju lokasi', 'Budi Santoso, S.Pd', '2026-09-08 11:00:00'),
(3, 2, 'diproses', 'selesai', 'Pekerjaan perbaikan keran selesai dilakukan', 'Budi Santoso, S.Pd', '2026-09-09 14:00:00');
