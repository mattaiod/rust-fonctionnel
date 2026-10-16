// Les avertissements des fonctions pas encore écrites (paramètres inutilisés…) sont tus.
#![allow(
    unused_imports,
    unused_variables,
    unused_mut,
    dead_code,
    unreachable_code
)]

//! Chapitre 13 : Traits au service du domaine : `From`/`Into`, `Display`, `Default`, `Add`/`Sum`, un trait
//! métier à type associé, `Iterator` pour son propre type, et des fonctions génériques avec bounds.
//!
//! Cours : `README.md` du chapitre. Lancer les tests : `cargo test -p ch13-traits`
//! (un seul exercice : `cargo test -p ch13-traits e03`).

pub mod e01_temperatures;
pub mod e02_affichage_defaut;
pub mod e03_argent;
pub mod e04_metrique;
pub mod e05_fibonacci;
pub mod e06_dates;
