// Les avertissements des fonctions pas encore écrites (paramètres inutilisés…) sont tus.
#![allow(
    unused_imports,
    unused_variables,
    unused_mut,
    dead_code,
    unreachable_code
)]

//! Chapitre 7, composition : la chaîne de méthodes comme `pipe`, les fonctions nommées (point-free), composer des
//! closures, l'application partielle à la place du currying.
//!
//! Cours : `README.md` du chapitre. Lancer les tests : `cargo test -p ch07-composition`
//! (un seul exercice : `cargo test -p ch07-composition e03`).

pub mod e01_nettoyer;
pub mod e02_application_partielle;
pub mod e03_composer;
pub mod e04_operations;
pub mod e05_pipe;
