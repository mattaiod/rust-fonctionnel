// Les avertissements des fonctions pas encore écrites (paramètres inutilisés…) sont tus.
#![allow(
    unused_imports,
    unused_variables,
    unused_mut,
    dead_code,
    unreachable_code
)]

//! Chapitre 15 : traits et génériques. Tests : `cargo test -p ch15-traits` (un exercice : `… e02`).

pub mod e01_generique;
pub mod e02_trait;
pub mod e03_methode_par_defaut;
pub mod e04_trait_comme_bound;
pub mod e05_dyn;
pub mod e06_display_from;
