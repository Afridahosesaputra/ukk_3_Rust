use axum::{
    routing::{get, patch, post, put},
    Router,
};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

use crate::{
    db::DbPool,
    handlers::{
        aspirasi::{
            create_aspirasi_handler, get_aspirasi_detail_handler, get_aspirasi_list_handler,
            update_aspirasi_status_handler,
        },
        auth::login_handler,
        kategori::{
            create_kategori_handler, delete_kategori_handler, get_kategori_list_handler,
            update_kategori_handler,
        },
        stats::get_dashboard_stats_handler,
        umpan_balik::{create_umpan_balik_handler, get_umpan_balik_by_aspirasi_handler},
    },
};

pub fn create_router(pool: DbPool) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let api_router = Router::new()
        // Auth
        .route("/auth/login", post(login_handler))
        // Stats
        .route("/stats", get(get_dashboard_stats_handler))
        // Kategori
        .route("/kategori", get(get_kategori_list_handler).post(create_kategori_handler))
        .route("/kategori/:id", put(update_kategori_handler).delete(delete_kategori_handler))
        // Aspirasi
        .route("/aspirasi", get(get_aspirasi_list_handler).post(create_aspirasi_handler))
        .route("/aspirasi/:id", get(get_aspirasi_detail_handler))
        .route("/aspirasi/:id/status", patch(update_aspirasi_status_handler))
        // Umpan Balik
        .route("/umpan-balik", post(create_umpan_balik_handler))
        .route("/umpan-balik/:id_aspirasi", get(get_umpan_balik_by_aspirasi_handler));

    Router::new()
        .nest("/api", api_router)
        .fallback_service(ServeDir::new("static"))
        .layer(cors)
        .with_state(pool)
}
