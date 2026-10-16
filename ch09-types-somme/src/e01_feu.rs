//! Exercice 1 : le feu tricolore.
//!
//! Trois booléens, c'est 2 × 2 × 2 = 8 états, dont 3 seulement ont un sens (exactement une couleur allumée). Un
//! `enum` à 3 variantes en a 3, tous légaux. Le `match` sur l'enum est **exhaustif** : oubliez une couleur et le
//! compilateur refuse (E0004). N'écrivez pas de bras `_ =>` sur vos propres enums : il éteint cette protection.

/// Version naïve : une lampe par booléen. Rien n'empêche rouge ET vert. Ne pas modifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeuNaif {
    pub rouge: bool,
    pub orange: bool,
    pub vert: bool,
}

/// Version naïve de la couleur suivante. Un état absurde donne... un feu éteint. Ne pas modifier.
pub fn suivant_naif(feu: FeuNaif) -> FeuNaif {
    let eteint = FeuNaif {
        rouge: false,
        orange: false,
        vert: false,
    };
    if feu.vert && !feu.orange && !feu.rouge {
        FeuNaif {
            orange: true,
            ..eteint
        }
    } else if feu.orange && !feu.vert && !feu.rouge {
        FeuNaif {
            rouge: true,
            ..eteint
        }
    } else if feu.rouge && !feu.vert && !feu.orange {
        FeuNaif {
            vert: true,
            ..eteint
        }
    } else {
        eteint
    }
}

/// Le feu, avec ses trois seuls états.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Feu {
    Vert,
    Orange,
    Rouge,
}

/// Vert → orange → rouge → vert.
///
/// **Aide Rust** (ch00, section 1)
/// - Oubliez une couleur et le compilateur dit `` error[E0004]: non-exhaustive patterns: `Feu::Rouge` not covered `` :
///   c'est la protection que vous voulez, ajoutez le bras.
pub fn suivant(feu: Feu) -> Feu {
    // But : un `match`, un bras par couleur, sans `_`.
    todo!()
}

/// Durée de chaque couleur, en secondes : vert 30, orange 3, rouge 20.
pub fn duree(feu: Feu) -> u32 {
    // But : un `match`.
    todo!()
}

/// Le feu typé correspondant à un feu naïf, `None` pour les 5 combinaisons absurdes.
///
/// **Aide Rust** (ch00, section 1)
/// - Matchez le triplet `(feu.vert, feu.orange, feu.rouge)`. Ici, un dernier bras `_ => None` est légitime : les 5
///   combinaisons restantes sont vraiment « tout le reste ».
pub fn depuis_naif(feu: FeuNaif) -> Option<Feu> {
    // But : un `match` sur le triplet de booléens.
    todo!()
}

/// Durée d'un cycle complet, en partant de `depart` (la somme des trois durées, quel que soit le départ).
///
/// **Aide Rust** (ch00, sections 2, 3)
/// - `std::iter::successors(Some(x), f)` est l'itérateur `x`, `f(x)`, `f(f(x))`… : `f` reçoit l'élément courant et
///   rend le suivant dans une `Option` (`None` arrête la suite). Il est infini ici : `take(n)` le borne.
/// - `successors` passe une **référence** vers l'élément courant : `` error[E0308]: mismatched types … expected `Feu`,
///   found `&Feu` `` → `|&feu|` ou `*feu` (`Feu` est `Copy`).
/// - `duree` se passe par son nom à `map`.
pub fn duree_cycle(depart: Feu) -> u32 {
    // But : parcourir les trois couleurs à partir de `depart` avec `suivant`, sans boucle.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::style;

    /// Les 8 combinaisons de trois booléens.
    fn tous_les_feux_naifs() -> Vec<FeuNaif> {
        (0..8_u8)
            .map(|bits| FeuNaif {
                rouge: bits & 1 != 0,
                orange: bits & 2 != 0,
                vert: bits & 4 != 0,
            })
            .collect()
    }

    #[test]
    fn cas_connus() {
        assert_eq!(suivant(Feu::Vert), Feu::Orange);
        assert_eq!(suivant(Feu::Orange), Feu::Rouge);
        assert_eq!(suivant(Feu::Rouge), Feu::Vert);
        assert_eq!(duree(Feu::Orange), 3);
        assert_eq!(duree_cycle(Feu::Vert), 53);
        assert_eq!(duree_cycle(Feu::Rouge), 53);
    }

    #[test]
    fn trois_etats_legaux_sur_huit() {
        let legaux = tous_les_feux_naifs()
            .into_iter()
            .filter_map(depuis_naif)
            .count();
        assert_eq!(legaux, 3);
    }

    #[test]
    fn meme_cycle_que_le_naif() {
        tous_les_feux_naifs().into_iter().for_each(|naif| {
            if let Some(feu) = depuis_naif(naif) {
                assert_eq!(
                    depuis_naif(suivant_naif(naif)),
                    Some(suivant(feu)),
                    "{naif:?}"
                );
            }
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e01_feu.rs"),
            &["suivant", "duree", "depuis_naif", "duree_cycle"],
        );
    }
}
