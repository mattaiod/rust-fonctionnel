//! Exercice 2 : application partielle : des fonctions qui fabriquent des fonctions.
//!
//! Rust n'a pas de currying : `multiplier(3)` ne peut pas vouloir dire « la fonction qui attend le second
//! argument ». Mais une fonction peut **rendre une closure** qui a capturé le premier argument :
//! `fn multiplier(facteur: i32) -> impl Fn(i32) -> i32`. Le `move` est obligatoire : la closure survit à l'appel,
//! elle doit posséder `facteur` au lieu de l'emprunter.
//! On obtient des fonctions sur mesure qu'on passe ensuite telles quelles aux étapes d'un pipeline, sans écrire de
//! closure : `.filter(entre(0, 10)).map(multiplier(3))`.

/// Version impérative : les paramètres sont mêlés au corps de la boucle. Ne pas modifier.
pub fn garder_et_multiplier_imperatif(
    nombres: &[i32],
    min: i32,
    max: i32,
    facteur: i32,
) -> Vec<i32> {
    let mut resultat = Vec::new();
    for &n in nombres {
        if min <= n && n <= max {
            resultat.push(n * facteur);
        }
    }
    resultat
}

/// La fonction « multiplier par `facteur` ».
///
/// **Aide Rust** (ch00, sections 8, 10)
/// - Le `move` est déjà écrit. Sans lui : `` error[E0373]: closure may outlive the current function, but it borrows
///   `facteur`, which is owned by the current function ``. La closure survit à l'appel : elle doit posséder `facteur`,
///   pas l'emprunter.
pub fn multiplier(facteur: i32) -> impl Fn(i32) -> i32 {
    move |n| {
        // La closure est déjà là (avec `move` : pourquoi ?) ; à vous le calcul.
        todo!()
    }
}

/// Le prédicat « est entre `min` et `max`, bornes comprises » (il reçoit une référence, pour `filter`).
///
/// **Aide Rust** (ch00, section 2)
/// - `n` est un `&i32`, `min` et `max` des `i32` : `min <= n` donne `` error[E0308]: mismatched types … expected `i32`,
///   found `&i32` ``. Écrivez `*n`, ou utilisez `(min..=max).contains(n)`, qui prend justement une référence.
pub fn entre(min: i32, max: i32) -> impl Fn(&i32) -> bool {
    move |n| {
        // Même idée, avec deux valeurs capturées. Un intervalle `min..=max` sait dire s'il contient un nombre.
        todo!()
    }
}

/// Garde les nombres de `[min, max]` et les multiplie par `facteur`, en assemblant les deux fonctions ci-dessus.
///
/// **Aide Rust** (ch00, sections 3, 4)
/// - `nombres.iter()` donne des `&i32`, `filter` passe un `&&i32` à son prédicat ; or `entre` attend un `&i32` et
///   `multiplier` un `i32`.
/// - Si le compilateur dit `` error[E0277]: expected a `FnMut(&&i32)` closure, found `impl for<'a> Fn(&'a i32) -> bool`
///   `` : copiez les nombres juste après `iter()` (`.copied()`), et les deux fonctions s'emboîtent.
pub fn garder_et_multiplier(nombres: &[i32], min: i32, max: i32, facteur: i32) -> Vec<i32> {
    // Un pipeline dont les étapes sont les fonctions fabriquées par `entre` et `multiplier`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    #[test]
    fn cas_connus() {
        let tripler = multiplier(3);
        assert_eq!(tripler(4), 12);
        assert_eq!(multiplier(-1)(5), -5);
        let chiffre = entre(0, 9);
        assert!(chiffre(&0) && chiffre(&9));
        assert!(!chiffre(&10) && !chiffre(&-1));
        assert_eq!(
            garder_et_multiplier(&[1, 5, 12, -3, 9], 0, 10, 2),
            vec![2, 10, 18]
        );
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        jeux::plusieurs(-50, 50).iter().for_each(|jeu| {
            let nombres: Vec<i32> = jeu.iter().map(|&n| n as i32).collect();
            [(-10, 10, 3), (0, 50, -1), (5, 4, 2)]
                .iter()
                .for_each(|&(min, max, facteur)| {
                    assert_eq!(
                        garder_et_multiplier(&nombres, min, max, facteur),
                        garder_et_multiplier_imperatif(&nombres, min, max, facteur),
                        "nombres : {nombres:?}, [{min}, {max}] × {facteur}"
                    )
                });
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e02_application_partielle.rs"),
            &["multiplier", "entre", "garder_et_multiplier"],
        );
    }
}
