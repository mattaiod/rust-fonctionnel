//! # Exercice 3 : la couche contrôleur, HTTP et rien d'autre
//!
//! Un handler Axum lit la requête avec des **extracteurs** (`State`, `Path`, `Json`), appelle le service, puis
//! traduit le résultat en réponse HTTP. Aucun calcul ici : il est dans le service.
//!
//! **À faire** : le handler `stats`, sur le modèle de `ajouter` (fourni, lisez-le d'abord) :
//! - capteur inconnu : `introuvable()` (code 404) ;
//! - sinon : les statistiques en JSON (code 200).
//!
//! **Ce qu'il faut savoir**
//! - `State(repo)` donne le repository, `Path(id)` l'id lu dans l'URL `/capteurs/{id}/stats`.
//! - `service::stats_capteur(&repo, id)` rend une `Option<StatsCapteur>`.
//! - `Json(stats).into_response()` fabrique une réponse 200 avec le JSON.
//!
//! Test : `cargo test -p ch18-couches e03`

use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use serde_json::json;

use crate::e01_repository::MesuresEnMemoire;
use crate::e02_service as service;

/// Le corps JSON de `POST /capteurs/{id}/mesures` : `{"valeurs": [21.5, 22.0]}`.
#[derive(Deserialize)]
pub struct NouvellesMesures {
    pub valeurs: Vec<f64>,
}

/// Fourni : la réponse 404.
pub fn introuvable() -> Response {
    (
        StatusCode::NOT_FOUND,
        Json(json!({"erreur": "capteur introuvable"})),
    )
        .into_response()
}

/// Fourni (le modèle) : `POST /capteurs/{id}/mesures` → 201 `{"ajoutees": 2}`, ou 404.
pub async fn ajouter(
    State(repo): State<MesuresEnMemoire>,
    Path(id): Path<u32>,
    Json(corps): Json<NouvellesMesures>,
) -> Response {
    match service::ajouter(&repo, id, corps.valeurs) {
        None => introuvable(),
        Some(nombre) => (StatusCode::CREATED, Json(json!({"ajoutees": nombre}))).into_response(),
    }
}

/// `GET /capteurs/{id}/stats` → 200 et les statistiques en JSON, ou 404.
pub async fn stats(State(repo): State<MesuresEnMemoire>, Path(id): Path<u32>) -> Response {
    // Le handler ne calcule rien : `None` devient 404, `Some` devient 200 avec le JSON.
    // `Json(stats)` marche parce que `StatsCapteur` dérive `Serialize`.
    match service::stats_capteur(&repo, id) {
        None => introuvable(),
        Some(stats) => Json(stats).into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn codes_http() {
        // On appelle le handler directement, comme le ferait Axum.
        let repo = MesuresEnMemoire::avec_capteurs(&[1]);
        assert_eq!(
            stats(State(repo.clone()), Path(1)).await.status(),
            StatusCode::OK
        );
        assert_eq!(
            stats(State(repo), Path(9)).await.status(),
            StatusCode::NOT_FOUND
        );
    }
}
