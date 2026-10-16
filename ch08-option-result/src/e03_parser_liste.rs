//! Exercice 3 : lire « 12, 7, x » sans `unwrap`.
//!
//! Un itérateur de `Result<T, E>` se `collect` en `Result<Vec<T>, E>` : `Ok` avec tous les éléments si tout a
//! réussi, sinon la première erreur (et le parcours s'arrête là). L'erreur d'origine (`ParseIntError`) ne dit ni
//! où ni quoi : à vous de la remplacer par une erreur qui le dit (`map_err`).

/// Version naïve : une boucle et un `unwrap` qui fait tomber le programme sur « x ». Ne pas modifier.
pub fn parser_liste_naif(texte: &str) -> Vec<i64> {
    let mut nombres = Vec::new();
    if texte.trim().is_empty() {
        return nombres;
    }
    for morceau in texte.split(',') {
        nombres.push(morceau.trim().parse::<i64>().unwrap());
    }
    nombres
}

/// Un morceau illisible : sa position (à partir de 1, pour un humain) et le texte fautif, sans les espaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErreurNombre {
    pub position: usize,
    pub valeur: String,
}

/// Les entiers séparés par des virgules (espaces tolérés) ; un texte vide donne une liste vide.
///
/// **Aide Rust** (ch00, sections 5, 12)
/// - Si le compilateur dit `` error[E0277]: a value of type `Result<Vec<i64>, ErreurNombre>` cannot be built from an
///   iterator over elements of type `Result<i64, ParseIntError>` `` : l'erreur de `parse` n'est pas la vôtre. `map_err`
///   la remplace par une `ErreurNombre`.
/// - Si le compilateur dit `` expected `String`, found `&str` `` dans `ErreurNombre { valeur: morceau, … }` :
///   `morceau.to_string()`.
/// - `"".split(',')` donne **un** morceau vide, pas zéro : le texte vide est un cas à part. Et `enumerate` compte à
///   partir de 0, la position humaine à partir de 1.
pub fn parser_liste(texte: &str) -> Result<Vec<i64>, ErreurNombre> {
    // But : une seule chaîne, qui s'arrête au premier morceau illisible et dit lequel.
    todo!()
}

/// La somme des entiers du texte, ou l'erreur de lecture.
///
/// **Aide Rust** (ch00, sections 11, 12)
/// - Si le compilateur dit `` error[E0277]: a value of type `Result<i64, ErreurNombre>` cannot be made by summing an
///   iterator over elements of type `&i64` `` : `?` a déjà ouvert le `Result`, la somme est un simple `i64`.
///   Remettez-la dans un `Ok(…)`.
pub fn somme_liste(texte: &str) -> Result<i64, ErreurNombre> {
    // But : réutiliser `parser_liste` ; une ligne suffit.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    fn erreur(position: usize, valeur: &str) -> ErreurNombre {
        ErreurNombre {
            position,
            valeur: valeur.to_string(),
        }
    }

    #[test]
    fn cas_connus() {
        assert_eq!(parser_liste("12, 7, -3"), Ok(vec![12, 7, -3]));
        assert_eq!(parser_liste("42"), Ok(vec![42]));
        assert_eq!(parser_liste("   "), Ok(vec![]));
        assert_eq!(somme_liste("1,2, 3"), Ok(6));
    }

    #[test]
    fn dit_ou_et_quoi() {
        assert_eq!(parser_liste("12, 7, x"), Err(erreur(3, "x")));
        assert_eq!(parser_liste("a, b"), Err(erreur(1, "a")));
        assert_eq!(parser_liste("1,,2"), Err(erreur(2, "")));
        assert_eq!(parser_liste("1, 2.5"), Err(erreur(2, "2.5")));
        assert_eq!(somme_liste("1, deux"), Err(erreur(2, "deux")));
    }

    #[test]
    #[should_panic]
    fn le_naif_panique() {
        parser_liste_naif("12, 7, x");
    }

    #[test]
    fn meme_resultat_que_le_naif_sur_du_valide() {
        jeux::plusieurs(-500, 500).iter().for_each(|jeu| {
            let texte = jeu
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(" , ");
            assert_eq!(
                parser_liste(&texte),
                Ok(parser_liste_naif(&texte)),
                "texte : {texte:?}"
            );
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e03_parser_liste.rs"),
            &["parser_liste", "somme_liste"],
        );
    }
}
