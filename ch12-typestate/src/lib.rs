// Les avertissements des fonctions pas encore écrites (paramètres inutilisés…) sont tus.
#![allow(
    unused_imports,
    unused_variables,
    unused_mut,
    dead_code,
    unreachable_code
)]

//! Chapitre 12 : Typestate : un type par état, des transitions qui consomment `self`, `PhantomData`, des
//! méthodes qui n'existent que dans le bon état.
//!
//! Cours : `README.md` du chapitre. Lancer les tests : `cargo test -p ch12-typestate`
//! (un seul exercice : `cargo test -p ch12-typestate e03`). Les doc-tests `compile_fail` prouvent qu'une
//! transition interdite ne compile pas : `cargo test -p ch12-typestate --doc`.

pub mod e01_porte;
pub mod e02_article;
pub mod e03_requete;
pub mod e04_soumission;
pub mod e05_commande;
