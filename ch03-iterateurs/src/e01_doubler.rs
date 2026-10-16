//! Exercice 1 : doubler chaque nombre.
//!
//! Transformer chaque élément d'une collection, c'est un `map` : on décrit ce qu'on fait à UN élément,
//! l'itérateur s'occupe du parcours.

/// Version impérative : un `Vec` mutable, un index, un `push` par tour. Ne pas modifier.
#[allow(clippy::needless_range_loop)] // l'index est voulu : c'est la boucle « à la C » qu'on va remplacer
pub fn doubler_imperatif(nombres: &[i64]) -> Vec<i64> {
    let mut resultat = Vec::new();
    for i in 0..nombres.len() {
        resultat.push(nombres[i] * 2);
    }
    resultat
}

/// Version fonctionnelle.
///
/// **Aide Rust** (ch00, sections 3, 12)
/// - Si le compilateur dit ``E0308: expected `Vec<i64>`, found `Map<Iter<'_, i64>, {closure@…}>` `` : `map` ne fait
///   rien seul, finissez par `.collect()`.
/// - `n` est un `&i64`, et `n * 2` marche quand même : Rust sait multiplier une référence d'entier.
/// - Dans un `let` intermédiaire sans type, `collect` ne sait souvent pas quoi fabriquer :
///   ``E0282: type annotations needed`` → `let doubles: Vec<i64> = …`. Ici, le type de retour de la fonction
///   suffit.
pub fn doubler(nombres: &[i64]) -> Vec<i64> {
    // Outils : `iter()`, `map(|n| …)`, `collect()`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    #[test]
    fn cas_connus() {
        assert_eq!(doubler(&[1, 2, 3]), vec![2, 4, 6]);
        assert_eq!(doubler(&[-5, 0]), vec![-10, 0]);
        assert_eq!(doubler(&[]), Vec::<i64>::new());
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        jeux::plusieurs(-100, 100)
            .iter()
            .for_each(|jeu| assert_eq!(doubler(jeu), doubler_imperatif(jeu), "jeu : {jeu:?}"));
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(include_str!("e01_doubler.rs"), &["doubler"]);
    }
}
