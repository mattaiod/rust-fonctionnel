// Les avertissements des fonctions pas encore écrites (paramètres inutilisés…) sont tus.
#![allow(
    unused_imports,
    unused_variables,
    unused_mut,
    dead_code,
    unreachable_code
)]

//! Chapitre 0 : kit de survie. Les pièges de Rust qu'on rencontre dans tous les autres chapitres (références dans
//! les itérateurs, `String` et `&str`, `clone`, `mut` dans un `fold`, closures `move`, `Option<&T>`).
//!
//! Cours : `README.md` du chapitre, avec pour chaque piège le message du compilateur et sa correction.
//! Lancer les tests : `cargo test -p ch00-survie` (un seul exercice : `cargo test -p ch00-survie e03`).

pub mod e01_depassements;
pub mod e02_mots_empruntes;
pub mod e03_noms_clones;
pub mod e04_accumulateur;
pub mod e05_closure_move;
pub mod e06_option_reference;
