//! # Exercice 4 : la couche routes, assembler l'application
//!
//! Le `Router` dit quelle URL et quelle méthode HTTP appellent quel handler, puis lui donne le repository avec
//! `with_state`. C'est le seul endroit où les couches sont branchées entre elles.
//!
//! **À faire** : `application` rend un `Router` avec deux routes :
//! - `POST /capteurs/{id}/mesures` → `controleur::ajouter` ;
//! - `GET  /capteurs/{id}/stats`   → `controleur::stats`.
//!
//! **Ce qu'il faut savoir**
//! - `Router::new().route("/chemin/{id}", get(handler)).route(…).with_state(repo)`
//! - `get` et `post` viennent de `axum::routing` (déjà importés). `{id}` est lu par `Path(id)` dans le handler.
//!
//! Test : `cargo test -p ch18-couches e04` (il envoie de vraies requêtes HTTP à l'application, sans ouvrir de port).
//! Puis à la main : `cargo run -p ch18-couches --example serveur` et les `curl` de `examples/serveur.rs`.

use axum::Router;
use axum::routing::{get, post};

use crate::e01_repository::MesuresEnMemoire;
use crate::e03_controleur as controleur;

/// L'application complète, prête à servir.
pub fn application(repo: MesuresEnMemoire) -> Router {
    // `{id}` est la syntaxe des paramètres de chemin en Axum 0.8 (`:id` dans les versions d'avant).
    // `with_state` donne le repository à tous les extracteurs `State` et rend un `Router<()>` prêt à servir.
    Router::new()
        .route("/capteurs/{id}/mesures", post(controleur::ajouter))
        .route("/capteurs/{id}/stats", get(controleur::stats))
        .with_state(repo)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use http_body_util::BodyExt;
    use serde_json::{Value, json};
    use tower::ServiceExt;

    async fn envoyer(app: Router, requete: Request<Body>) -> (StatusCode, Value) {
        let reponse = app.oneshot(requete).await.unwrap();
        let statut = reponse.status();
        let octets = reponse.into_body().collect().await.unwrap().to_bytes();
        (
            statut,
            serde_json::from_slice(&octets).unwrap_or(Value::Null),
        )
    }

    #[tokio::test]
    async fn de_la_route_au_repository() {
        let app = application(MesuresEnMemoire::avec_capteurs(&[1]));

        let post = Request::post("/capteurs/1/mesures")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"valeurs": [12.0, 20.0, 10.0]}"#))
            .unwrap();
        assert_eq!(
            envoyer(app.clone(), post).await,
            (StatusCode::CREATED, json!({"ajoutees": 3}))
        );

        let get = Request::get("/capteurs/1/stats")
            .body(Body::empty())
            .unwrap();
        let attendu =
            json!({"capteur_id": 1, "nb_mesures": 3, "moyenne": 14.0, "min": 10.0, "max": 20.0});
        assert_eq!(envoyer(app.clone(), get).await, (StatusCode::OK, attendu));

        let inconnu = Request::get("/capteurs/9/stats")
            .body(Body::empty())
            .unwrap();
        assert_eq!(envoyer(app, inconnu).await.0, StatusCode::NOT_FOUND);
    }
}
