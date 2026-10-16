// Les avertissements des fonctions pas encore écrites (paramètres inutilisés…) sont tus.
#![allow(
    unused_imports,
    unused_variables,
    unused_mut,
    dead_code,
    unreachable_code
)]

//! Chapitre 11 : Erreurs de domaine : un `enum` par couche, `Display`, `Error`, `From` pour `?`, et l'accumulation
//! de toutes les erreurs au lieu de s'arrêter à la première.
//!
//! Cours : `README.md` du chapitre. Lancer les tests : `cargo test -p ch11-erreurs`
//! (un seul exercice : `cargo test -p ch11-erreurs e03`).

pub mod e01_note;
pub mod e02_reservation;
pub mod e03_commande;
pub mod e04_inscription;
pub mod e05_csv;
