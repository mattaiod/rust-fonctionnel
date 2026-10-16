//! Exercice 2 : un filtre paramétré.
//!
//! `plus_grand_que(10)` rend un prédicat tout prêt pour `filter`. Après `copied()`, l'itérateur produit des `i64`
//! et `filter` passe à son prédicat une référence vers l'élément : un `&i64`, d'où la signature
//! `impl Fn(&i64) -> bool`. Le prédicat ne fait que LIRE le seuil capturé : il est `Fn`, on peut l'appeler autant
//! de fois qu'on veut.

/// Version impérative. Ne pas modifier.
pub fn garder_au_dessus_imperatif(nombres: &[i64], seuil: i64) -> Vec<i64> {
    let mut gardes = Vec::new();
    for n in nombres {
        if *n > seuil {
            gardes.push(*n);
        }
    }
    gardes
}

/// Un prédicat « strictement plus grand que `seuil` ».
///
/// **Aide Rust** (ch00, section 4)
/// - Si le compilateur dit ``E0308: expected `&i64`, found `i64` `` : `n` est une référence, comparez `*n` à `seuil`.
pub fn plus_grand_que(seuil: i64) -> impl Fn(&i64) -> bool {
    move |n: &i64| {
        // Une comparaison ; `n` est une référence, `*n` la valeur.
        todo!()
    }
}

/// Version fonctionnelle : les nombres strictement plus grands que `seuil`.
///
/// **Aide Rust** (ch00, sections 3, 4)
/// - `filter` passe une référence vers l'élément : sur `iter()` (des `&i64`), le prédicat recevrait un `&&i64`, alors
///   que `plus_grand_que` attend un `&i64`.
/// - Si le compilateur dit ``E0277: expected a `FnMut(&&i64)` closure, found `impl for<'a> Fn(&'a i64) -> bool` `` :
///   mettez `.copied()` AVANT `filter` (le `for<'a>` veut seulement dire « pour toute référence », ignorez-le).
/// - Si le compilateur dit
///   ``E0277: a value of type `Vec<i64>` cannot be built from an iterator over elements of type `&i64` `` : même
///   remède, `.copied()`.
pub fn garder_au_dessus(nombres: &[i64], seuil: i64) -> Vec<i64> {
    // Outils : `iter()`, `copied()`, `filter(plus_grand_que(seuil))`, `collect()`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    #[test]
    fn cas_connus() {
        let majeur = plus_grand_que(17);
        assert!(majeur(&18));
        assert!(!majeur(&17));
        assert_eq!(garder_au_dessus(&[5, 12, 10, 30], 10), vec![12, 30]);
        assert_eq!(garder_au_dessus(&[-3, -1], 0), Vec::<i64>::new());
    }

    #[test]
    fn le_meme_predicat_sert_plusieurs_fois() {
        let positif = plus_grand_que(0);
        let combien = [-1, 2, 3].iter().filter(|n| positif(n)).count();
        assert_eq!(combien, 2);
        assert!(positif(&1));
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        jeux::plusieurs(-50, 50).iter().for_each(|jeu| {
            [-20, 0, 7, 50].iter().for_each(|seuil| {
                assert_eq!(
                    garder_au_dessus(jeu, *seuil),
                    garder_au_dessus_imperatif(jeu, *seuil),
                    "jeu : {jeu:?}, seuil : {seuil}"
                )
            })
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e02_plus_grand_que.rs"),
            &["plus_grand_que", "garder_au_dessus"],
        );
    }
}
