//! Exercice 5 : composer des fonctions, sans version impérative. À vous.
//!
//! `composer2(f, g)` rend la fonction « `f` puis `g` » : `composer2(f, g)(x) == g(f(x))`. Elle reçoit deux
//! closures et en rend une troisième qui les capture toutes les deux (par valeur : `move`). Les types sont
//! **génériques** : `<A, B, C>` après le nom de la fonction déclare trois noms de types que l'appelant choisit à
//! chaque appel (des `i64`, des `&str`, ce qu'il veut). Ici, `f` va de `A` vers `B`, `g` de `B` vers `C`, et le
//! résultat de `A` vers `C`. Vous n'avez rien à écrire sur ces types : le compilateur vérifie que `g(f(x))` les
//! respecte.
//!
//! `composer_tout` généralise à une liste de fonctions de `i64` vers `i64`. Chaque closure de la liste a son propre
//! type, elles sont donc rangées dans des `Box<dyn Fn>` (ch00, section 10). La liste vide donne l'identité
//! (`x ↦ x`). Pensez au `fold` de l'exercice 4.

/// « `f` puis `g` ».
///
/// **Aide Rust** (ch00, sections 8, 9)
/// - L'ordre compte : d'abord `f`, puis `g` sur le résultat de `f`.
/// - Le `move` est écrit pour vous : sans lui,
///   ``E0373: closure may outlive the current function, but it borrows `f` `` (et `g`).
pub fn composer2<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C {
    move |x| todo!()
}

/// Toutes les fonctions de la liste, de la première à la dernière.
///
/// **Aide Rust** (ch00, sections 8, 9, 10)
/// - La forme du résultat : `Box::new(move |x| …)`, une closure qui GARDE la liste. Sans `move` :
///   ``E0373: closure may outlive the current function, but it borrows `fonctions` ``.
/// - Dans la closure, parcourez `fonctions.iter()`, pas `into_iter()` : une `Fn` peut être appelée plusieurs fois,
///   elle ne doit pas consommer sa liste. Sinon :
///   ``E0507: cannot move out of `fonctions`, a captured variable in an `Fn` closure``.
/// - Le reste est le `fold` de l'exercice 4 : départ `x`, chaque `f` appliquée à l'accumulateur.
pub fn composer_tout(fonctions: Vec<Box<dyn Fn(i64) -> i64>>) -> Box<dyn Fn(i64) -> i64> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::style;

    #[test]
    fn cas_connus() {
        let plus_un_puis_double = composer2(|x: i64| x + 1, |x| x * 2);
        assert_eq!(plus_un_puis_double(3), 8);
        let double_puis_plus_un = composer2(|x: i64| x * 2, |x| x + 1);
        assert_eq!(double_puis_plus_un(3), 7);
    }

    #[test]
    fn change_de_type() {
        let longueur_paire = composer2(|s: &str| s.chars().count(), |n| n % 2 == 0);
        assert!(longueur_paire("élève!"));
        assert!(!longueur_paire("abc"));
    }

    #[test]
    fn composer_une_liste() {
        let f = composer_tout(vec![
            Box::new(|x| x + 1),
            Box::new(|x| x * 10),
            Box::new(|x| x - 3),
        ]);
        assert_eq!(f(1), 17);
        assert_eq!(f(0), 7);
        assert_eq!(composer_tout(vec![])(42), 42);
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e05_composer.rs"),
            &["composer2", "composer_tout"],
        );
    }
}
