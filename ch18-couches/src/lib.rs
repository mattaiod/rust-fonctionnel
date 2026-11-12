// Les avertissements des fonctions pas encore écrites (paramètres inutilisés…) sont tus.
#![allow(
    unused_imports,
    unused_variables,
    unused_mut,
    dead_code,
    unreachable_code
)]

//! Chapitre 18 : une API en couches avec Axum. Un fichier par couche, à faire dans l'ordre :
//!
//! ```text
//! e04_routes       quelle URL appelle quel handler
//! e03_controleur   HTTP : lire la requête, appeler le service, rendre un code et du JSON
//! e02_service      les règles : calculer les stats, ni HTTP ni stockage
//! e01_repository   le stockage, derrière un trait
//! ```
//!
//! Chaque couche ne connaît que celle du dessous. Tests : `cargo test -p ch18-couches` (un exercice : `… e02`).
//! Lancer le serveur : `cargo run -p ch18-couches --example serveur`.

pub mod e01_repository;
pub mod e02_service;
pub mod e03_controleur;
pub mod e04_routes;
