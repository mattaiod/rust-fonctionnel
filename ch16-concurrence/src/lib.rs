// Les avertissements des fonctions pas encore écrites (paramètres inutilisés…) sont tus.
#![allow(
    unused_imports,
    unused_variables,
    unused_mut,
    dead_code,
    unreachable_code
)]

//! Chapitre 16 : threads, `Arc<Mutex>`, channels, rayon. Tests : `cargo test -p ch16-concurrence` (un exercice : `… e02`).

pub mod e01_threads;
pub mod e02_arc_mutex;
pub mod e03_channel;
pub mod e04_producteurs;
pub mod e05_rayon;
