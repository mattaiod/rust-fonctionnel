//! Exercice 2 : le score d'une partie. Prendre l'état, rendre le nouvel état.
//!
//! Un coup réussi rapporte `10 × série` points, où `série` compte les réussites d'affilée (1, 2, 3…) ; un coup
//! raté remet la série à zéro. La version impérative fait évoluer un `Score` passé en `&mut`. La version pure
//! est une fonction `(ancien état, coup) -> nouvel état` : elle ne modifie rien, elle fabrique un `Score`.
//! Rejouer toute la partie revient alors à enchaîner cette fonction sur la liste des coups : c'est un `fold`,
//! dont l'accumulateur est l'état.

/// L'état de la partie.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Score {
    pub points: i64,
    pub serie: i64,
}

/// Version impérative d'un coup : modifie le score de l'appelant. Ne pas modifier.
pub fn jouer_coup_imperatif(score: &mut Score, reussi: bool) {
    if reussi {
        score.serie += 1;
        score.points += 10 * score.serie;
    } else {
        score.serie = 0;
    }
}

/// Version impérative de la partie : un état mutable, une boucle. Ne pas modifier.
pub fn partie_imperatif(coups: &[bool]) -> Score {
    let mut score = Score::default();
    for reussi in coups {
        jouer_coup_imperatif(&mut score, *reussi);
    }
    score
}

/// Version pure d'un coup : rend le score après le coup.
///
/// **Aide Rust** (ch00, sections 2, 7)
/// - `score` n'est pas `mut` et ne doit pas le devenir : on ne change pas ses champs, on fabrique un nouveau
///   `Score { points: …, serie: … }`.
/// - Si le compilateur dit ``E0594: cannot assign to `score.serie`, as `score` is not declared as mutable`` :
///   remplacez `score.serie += 1` par un `Score { … }` neuf.
/// - Si le compilateur dit ``cannot use a comma after the base struct`` : `..score` se met en DERNIER, sans virgule
///   après.
/// - `Score` est `Copy` : l'appelant garde son ancien score, aucun `clone()` à écrire.
pub fn jouer_coup(score: Score, reussi: bool) -> Score {
    // Un `if` qui rend un `Score { … }` dans chaque branche ; `..score` recopie les champs inchangés.
    todo!()
}

/// Version pure de la partie : le score final, en partant de `Score::default()`.
///
/// **Aide Rust** (ch00, sections 3, 7)
/// - `coups.iter()` donne des `&bool`, pas des `bool` : le `&reussi` de la consigne « déballe » la référence dans les
///   paramètres de la closure.
/// - Si le compilateur dit ``E0308: expected `bool`, found `&bool` `` : écrivez `|score, &reussi|`, ou passez
///   `*reussi` à `jouer_coup`.
pub fn partie(coups: &[bool]) -> Score {
    // Outils : `iter()`, `fold(Score::default(), |score, &reussi| …)`, `jouer_coup`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    #[test]
    fn cas_connus() {
        let depart = Score::default();
        assert_eq!(
            jouer_coup(depart, true),
            Score {
                points: 10,
                serie: 1
            }
        );
        assert_eq!(
            jouer_coup(
                Score {
                    points: 30,
                    serie: 2
                },
                false
            ),
            Score {
                points: 30,
                serie: 0
            }
        );
        assert_eq!(
            partie(&[true, true, false, true]),
            Score {
                points: 40,
                serie: 1
            }
        );
        assert_eq!(partie(&[]), Score::default());
    }

    #[test]
    fn l_ancien_etat_reste_utilisable() {
        let avant = Score {
            points: 10,
            serie: 1,
        };
        let apres = jouer_coup(avant, true);
        assert_eq!(
            avant,
            Score {
                points: 10,
                serie: 1
            }
        );
        assert_eq!(
            apres,
            Score {
                points: 30,
                serie: 2
            }
        );
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        (0..20).for_each(|graine| {
            let coups: Vec<bool> = jeux::entiers(graine, 40, 0, 2)
                .iter()
                .map(|n| *n > 0)
                .collect();
            assert_eq!(
                partie(&coups),
                partie_imperatif(&coups),
                "coups : {coups:?}"
            );
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(include_str!("e02_score.rs"), &["jouer_coup", "partie"]);
    }
}
