// Les avertissements des fonctions pas encore écrites (paramètres inutilisés…) sont tus.
#![allow(
    unused_imports,
    unused_variables,
    unused_mut,
    dead_code,
    unreachable_code
)]

//! Chapitre 5, combinators : `zip`, `enumerate`, `flat_map`, `map_while`, `windows`, `scan`, `chunk_by`, `fold`
//! avec `entry`, et `collect` vers d'autres collections que `Vec`.
//!
//! Cours : `README.md` du chapitre. Lancer les tests : `cargo test -p ch05-combinators`
//! (un seul exercice : `cargo test -p ch05-combinators e03`).

pub mod e01_produit_scalaire;
pub mod e02_indices_pairs;
pub mod e03_tags_uniques;
pub mod e04_cumul_avant_panne;
pub mod e05_frequences;
pub mod e06_series;
