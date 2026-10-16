//! Exercice 3 : composer des fonctions.
//!
//! Composer `f` puis `g`, c'est fabriquer la fonction `x ↦ g(f(x))`. En Rust, c'est une fonction générique qui
//! prend deux fonctions et rend une closure : les trois paramètres de type `A → B → C` disent que la sortie de `f`
//! doit être l'entrée de `g`, et le compilateur le vérifie.
//! Pour composer une **liste** de fonctions dont on ne connaît le nombre qu'à l'exécution, il faut un type commun :
//! chaque closure a son propre type, on les range donc dans des `Box<dyn Fn(i32) -> i32>`. Composer toute la liste,
//! c'est la replier : on part de la fonction identité et on compose avec chaque élément.
//! `Box<dyn Fn(i32) -> i32>` implémente lui-même `Fn(i32) -> i32` : on peut le passer à `composer`.
//! Au chapitre 2 (exercice 5), `composer_tout` gardait la liste et la parcourait à chaque appel. Ici, on compose
//! vraiment : le `fold` replie les fonctions elles-mêmes en une seule, et la liste disparaît.

/// Version impérative : on fait passer la valeur dans chaque fonction, une par une. Ne pas modifier.
pub fn appliquer_tout_imperatif(fonctions: &[Box<dyn Fn(i32) -> i32>], depart: i32) -> i32 {
    let mut valeur = depart;
    for f in fonctions {
        valeur = f(valeur);
    }
    valeur
}

/// `f` puis `g` : la fonction `x ↦ g(f(x))`.
pub fn composer<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C {
    move |x| {
        // Appliquer l'une puis l'autre.
        todo!()
    }
}

/// Une seule fonction qui applique toutes les `fonctions`, dans l'ordre de la liste.
///
/// **Aide Rust** (ch00, sections 8, 10, 12)
/// - Chaque tour du `fold` doit rendre le même type que l'accumulateur, une `Box<dyn Fn(i32) -> i32>` : emballez le
///   résultat de `composer` dans `Box::new(…)`.
/// - Si le compilateur dit `` error[E0308]: mismatched types … expected `Box<dyn Fn(i32) -> i32>`, found opaque type ``
///   : vous rendez `composer(avant, f)` nu (« opaque type » = le `impl Fn` de `composer`). Mettez-le dans une `Box`.
/// - Si le compilateur dit `` error[E0515]: cannot return value referencing function parameter `fonctions` `` :
///   `fonctions.iter()` ne fait que prêter les fonctions, qui meurent à la fin de `composer_tout`. `into_iter()` les
///   donne.
pub fn composer_tout(fonctions: Vec<Box<dyn Fn(i32) -> i32>>) -> Box<dyn Fn(i32) -> i32> {
    // Replier la liste en partant de l'identité, en composant l'accumulateur avec chaque fonction.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    /// Une liste de fonctions tirée d'un jeu de nombres.
    fn fabriquer(jeu: &[i64]) -> Vec<Box<dyn Fn(i32) -> i32>> {
        jeu.iter()
            .map(|&n| -> Box<dyn Fn(i32) -> i32> {
                let n = n as i32;
                match n.rem_euclid(3) {
                    0 => Box::new(move |x| x + n),
                    1 => Box::new(move |x| x - n),
                    _ => Box::new(|x| x / 2),
                }
            })
            .collect()
    }

    #[test]
    fn cas_connus() {
        let ajouter_un = |x: i32| x + 1;
        let doubler = |x: i32| x * 2;
        assert_eq!(composer(ajouter_un, doubler)(5), 12);
        assert_eq!(composer(doubler, ajouter_un)(5), 11);
        assert_eq!(composer(str::len, |n: usize| n * 10)("abc"), 30);
        assert_eq!(
            composer(|x: i32| x.to_string(), |s: String| s + "!")(7),
            "7!"
        );
    }

    #[test]
    fn composer_tout_cas_connus() {
        let fonctions: Vec<Box<dyn Fn(i32) -> i32>> =
            vec![Box::new(|x| x + 1), Box::new(|x| x * 10), Box::new(|x| -x)];
        assert_eq!(composer_tout(fonctions)(2), -30);
        assert_eq!(composer_tout(vec![])(42), 42);
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        jeux::plusieurs(-50, 50).iter().for_each(|jeu| {
            let tout = composer_tout(fabriquer(jeu));
            let fonctions = fabriquer(jeu);
            (-5..=5).for_each(|x| {
                assert_eq!(
                    tout(x),
                    appliquer_tout_imperatif(&fonctions, x),
                    "jeu : {jeu:?}, x = {x}"
                )
            });
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e03_composer.rs"),
            &["composer", "composer_tout"],
        );
    }
}
