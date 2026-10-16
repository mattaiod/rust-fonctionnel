//! Exercice 2 : les positions des nombres pairs.
//!
//! On ne veut pas les nombres pairs mais leurs **positions** dans la liste. Il faut donc garder l'index à côté de
//! chaque élément : `enumerate` transforme chaque élément `x` en paire `(index, x)`, l'index en premier.
//! Ensuite, c'est un filtre sur la valeur et un `map` qui ne garde que l'index. `filter_map` fait les deux d'un coup
//! si la closure rend `Some(index)` ou `None`.

/// Version impérative : l'index est la variable de boucle. Ne pas modifier.
pub fn indices_pairs_imperatif(nombres: &[i64]) -> Vec<usize> {
    let mut indices = Vec::new();
    let mut i = 0;
    while i < nombres.len() {
        if nombres[i] % 2 == 0 {
            indices.push(i);
        }
        i += 1;
    }
    indices
}

/// Les index (à partir de 0) des éléments pairs, dans l'ordre.
///
/// **Aide Rust** (ch00, sections 3, 4)
/// - `iter().enumerate()` donne des `(usize, &i64)`, et `filter` passe à sa closure une **référence** vers cette paire
///   : la valeur y est un `&&i64`.
/// - Si le compilateur dit `` error[E0369]: cannot calculate the remainder of `&&i64` divided by `{integer}` `` :
///   déréférencez (`*n % 2`), ou déstructurez dans le motif : `|&(_, &n)|` donne un `i64`.
pub fn indices_pairs(nombres: &[i64]) -> Vec<usize> {
    // Numéroter les éléments, garder ceux dont la valeur est paire, ne retenir que le numéro.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    #[test]
    fn cas_connus() {
        assert_eq!(indices_pairs(&[4, 7, 2, 9, 0]), vec![0, 2, 4]);
        assert_eq!(indices_pairs(&[1, 3, 5]), Vec::<usize>::new());
        assert_eq!(indices_pairs(&[-2]), vec![0]);
        assert_eq!(indices_pairs(&[]), Vec::<usize>::new());
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        jeux::plusieurs(-100, 100).iter().for_each(|jeu| {
            assert_eq!(
                indices_pairs(jeu),
                indices_pairs_imperatif(jeu),
                "jeu : {jeu:?}"
            )
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(include_str!("e02_indices_pairs.rs"), &["indices_pairs"]);
    }
}
