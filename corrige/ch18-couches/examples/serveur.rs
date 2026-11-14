//! Le serveur, pour tester à la main. Lancer : `cargo run -p ch18-couches --example serveur`, puis :
//!
//! ```text
//! curl -s -X POST localhost:3000/capteurs/1/mesures -H 'Content-Type: application/json' -d '{"valeurs": [21.5, 22.0, 19.8]}'
//! curl -s localhost:3000/capteurs/1/stats
//! curl -si localhost:3000/capteurs/9/stats
//! ```

use ch18_couches::e01_repository::MesuresEnMemoire;
use ch18_couches::e04_routes::application;

#[tokio::main]
async fn main() {
    let repo = MesuresEnMemoire::avec_capteurs(&[1, 2, 3]);
    let ecoute = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Serveur sur http://localhost:3000 (capteurs 1, 2 et 3)");
    axum::serve(ecoute, application(repo)).await.unwrap();
}
