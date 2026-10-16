// Les avertissements des fonctions pas encore écrites (paramètres inutilisés…) sont tus.
#![allow(
    unused_imports,
    unused_variables,
    unused_mut,
    dead_code,
    unreachable_code
)]

//! Chapitre 10 : Newtypes : un type par concept, parse don't validate.
//!
//! Cours : `README.md` du chapitre. Lancer les tests : `cargo test -p ch10-newtypes`
//! (un seul exercice : `cargo test -p ch10-newtypes e03` ; les doc-tests : `cargo test -p ch10-newtypes --doc`).

pub mod e01_email;
pub mod e02_pourcentage;
pub mod e03_score;
pub mod e04_identifiants;
pub mod e05_non_vide;
pub mod e06_code_postal;
