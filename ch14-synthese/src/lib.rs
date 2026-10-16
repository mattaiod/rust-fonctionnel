// Les avertissements des fonctions pas encore écrites (paramètres inutilisés…) sont tus.
#![allow(
    unused_imports,
    unused_variables,
    unused_mut,
    dead_code,
    unreachable_code
)]

//! Chapitre 14 : Synthèse. Cinq petits programmes écrits directement en fonctionnel.
//!
//! Cours et méthode : `README.md` du chapitre. Lancer les tests : `cargo test -p ch14-synthese`
//! (un seul exercice : `cargo test -p ch14-synthese e03`). L'exercice 5 contient des tests à écrire, marqués
//! `#[ignore]` : `cargo test -p ch14-synthese -- --include-ignored` les lance aussi.

pub mod e01_bulletin;
pub mod e02_rle;
pub mod e03_journal;
pub mod e04_panier;
pub mod e05_reservations;
