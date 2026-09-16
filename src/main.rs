mod db;
mod handlers;
mod models;
mod routes;

use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Inisialisasi logging console
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "pengaduan_sarana_sekolah=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    println!("============================================================");
    println!("  APLIKASI PENGADUAN SARANA SEKOLAH (RUST + AXUM + SQLITE)  ");
    println!("  UJI KOMPETENSI KEAHLIAN (UKK) REKAYASA PERANGKAT LUNAK    ");
    println!("============================================================");

    // 1. Inisialisasi Database SQLite
    let db_path = "pengaduan_sekolah.db";
    println!("[INFO] Menginisialisasi database SQLite: {}", db_path);
    let pool = db::init_db(db_path);
    println!("[SUCCESS] Database berhasil dimuat dan dimigrasi.");

    // 2. Bangun Router Axum
    let app = routes::create_router(pool);

    // 3. Bind TCP listener pada port 3000
    let host = [127, 0, 0, 1];
    let port = 3000;
    let addr = SocketAddr::from((host, port));

    println!("[INFO] Server web berjalan di http://{}", addr);
    println!("[INFO] Buka browser Anda dan akses: http://localhost:{}", port);
    println!("------------------------------------------------------------");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Gagal mengikat port 3000. Pastikan port tidak sedang digunakan.");

    axum::serve(listener, app)
        .await
        .expect("Terjadi error saat menjalankan server web");
}
