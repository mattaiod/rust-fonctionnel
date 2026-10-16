//! Exercice 3 : appliquer une fonction `n` fois.
//!
//! `appliquer_n_fois(f, 3, x)` vaut `f(f(f(x)))`. On REÇOIT une closure : le paramètre `f: impl Fn(i64) -> i64`
//! accepte n'importe quelle closure (ou fonction nommée) de ce type, et le compilateur produit une version de
//! `appliquer_n_fois` par closure, sans coût à l'appel. Deux façons sans boucle :
//! 1. un `fold` sur la *range* `0..n` (les entiers de 0 à n − 1, un itérateur comme un autre), dont on ignore les
//!    éléments (`|acc, _|`, le `_` veut dire « je n'utilise pas ce paramètre ») : seul le nombre de tours compte ;
//! 2. la récursion (la fonction s'appelle elle-même, chapitre 6) : 0 fois, c'est `x` ; `n` fois, c'est `n − 1` fois
//!    sur `f(x)`.

/// Version impérative. Ne pas modifier.
pub fn appliquer_n_fois_imperatif(f: impl Fn(i64) -> i64, n: u32, x: i64) -> i64 {
    let mut valeur = x;
    for _ in 0..n {
        valeur = f(valeur);
    }
    valeur
}

/// Version fonctionnelle : `f` appliquée `n` fois à `x`.
///
/// **Aide Rust** (ch00, sections 7, 9)
/// - La closure du `fold` RENVOIE la nouvelle valeur : `|acc, _| f(acc)`. Si le compilateur dit
///   ``E0384: cannot assign to immutable argument `acc` `` : vous avez écrit `acc = …`.
/// - En récursif, `appliquer_n_fois(f, n - 1, f(x))` donne ``E0382: borrow of moved value: `f` `` : `f` part dans
///   l'appel avant que `f(x)` soit calculé. Calculez d'abord `let suivant = f(x);`.
/// - Ne passez pas `&f` à l'appel récursif : la compilation échoue sur
///   ``reached the recursion limit while instantiating``.
pub fn appliquer_n_fois(f: impl Fn(i64) -> i64, n: u32, x: i64) -> i64 {
    // Outils : `(0..n).fold(x, |acc, _| …)`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    fn doubler(x: i64) -> i64 {
        x * 2
    }

    #[test]
    fn cas_connus() {
        assert_eq!(appliquer_n_fois(|x| x + 3, 4, 0), 12);
        assert_eq!(appliquer_n_fois(doubler, 10, 1), 1024);
        assert_eq!(appliquer_n_fois(|x| x * 100, 0, 7), 7);
    }

    #[test]
    fn la_closure_peut_capturer() {
        let pas = 5;
        assert_eq!(appliquer_n_fois(|x| x - pas, 3, 100), 85);
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        let collatz = |x: i64| if x % 2 == 0 { x / 2 } else { 3 * x + 1 };
        jeux::entiers(42, 30, 1, 1000).iter().for_each(|x| {
            (0..12).for_each(|n| {
                assert_eq!(
                    appliquer_n_fois(collatz, n, *x),
                    appliquer_n_fois_imperatif(collatz, n, *x),
                    "x = {x}, n = {n}"
                );
                assert_eq!(
                    appliquer_n_fois(|v| (v * 7 + 1) % 1000, n, *x),
                    appliquer_n_fois_imperatif(|v| (v * 7 + 1) % 1000, n, *x),
                    "x = {x}, n = {n}"
                );
            })
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e03_appliquer_n_fois.rs"),
            &["appliquer_n_fois"],
        );
    }
}
