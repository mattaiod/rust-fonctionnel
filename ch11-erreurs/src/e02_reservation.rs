//! Exercice 2 : `Display` et `Error` à la main.
//!
//! L'`enum` d'erreur dit la cause au code ; `Display` la dit à l'humain. `Display` ne se dérive pas : c'est vous
//! qui écrivez la phrase, dans `fmt`, avec `write!(f, …)`. `impl std::error::Error` (vide : toutes ses méthodes
//! ont une version par défaut) fait entrer votre type dans l'écosystème : `Box<dyn Error>`, `anyhow`, `?` dans
//! un `main`. En pratique, `thiserror` écrit ces deux `impl` pour vous ; ici on les écrit une fois pour voir.

use std::error::Error;
use std::fmt;

/// Nombre maximal de convives à une même table.
pub const MAX_PAR_TABLE: u32 = 8;

/// Version naïve : chaque erreur est une phrase fabriquée sur place. Ne pas modifier.
pub fn reserver_naif(nom: &str, personnes: u32, places_restantes: u32) -> Result<u32, String> {
    if nom.trim().is_empty() {
        return Err("nom vide".to_string());
    }
    if personnes > MAX_PAR_TABLE {
        return Err(format!(
            "{personnes} personnes demandées, {MAX_PAR_TABLE} au plus par table"
        ));
    }
    if personnes > places_restantes {
        return Err(format!(
            "complet : {personnes} places demandées, {places_restantes} restantes"
        ));
    }
    Ok(places_restantes - personnes)
}

/// Les refus possibles d'une réservation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurReservation {
    NomVide,
    TropDePersonnes { demandees: u32 },
    Complet { demandees: u32, restantes: u32 },
}

impl fmt::Display for ErreurReservation {
    /// Les mêmes phrases que la version naïve.
    ///
    /// **Aide Rust** (ch00, section 1)
    /// - Préfixez les variantes (`Self::NomVide`). Sans préfixe, `NomVide =>` devient une variable qui attrape tout :
    ///   `E0170 pattern binding NomVide is named the same as one of the variants`.
    /// - `TropDePersonnes` a un champ nommé : on le déstructure avec des accolades,
    ///   `Self::TropDePersonnes { demandees }`. Avec des parenthèses :
    ///   `E0164 expected tuple struct or tuple variant, found struct variant`.
    /// - Chaque bras rend le `fmt::Result` de `write!(f, ...)`, pas une `&str`
    ///   (`E0308 … expected Result<(), Error>, found &str`). Dans la chaîne de format, `{demandees}` marche,
    ///   `{self.champ}` non : `invalid format string: field access isn't supported`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // But : une phrase par variante, mot pour mot celles de `reserver_naif`.
        todo!()
    }
}

// Une ligne suffit : `source` et `description` ont une implémentation par défaut.
impl Error for ErreurReservation {}

/// Réserve `personnes` places ; rend le nombre de places restantes après la réservation.
///
/// **Aide Rust** (ch00, section 1)
/// - Les champs de `Complet` s'appellent `demandees` et `restantes` : `Complet { personnes, places_restantes }` donne
///   `E0559 variant ErreurReservation::Complet has no field named personnes`. Écrivez `demandees: personnes`.
/// - `places_restantes - personnes` panique (`attempt to subtract with overflow`) si on n'a pas d'abord vérifié qu'il
///   reste assez de places. Gardez l'ordre des règles de la version naïve.
pub fn reserver(
    nom: &str,
    personnes: u32,
    places_restantes: u32,
) -> Result<u32, ErreurReservation> {
    // But : les règles de `reserver_naif`, dans le même ordre, avec `ErreurReservation`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::style;

    #[test]
    fn cas_connus() {
        assert_eq!(reserver("Alice", 4, 10), Ok(6));
        assert_eq!(reserver("Alice", 4, 4), Ok(0));
        assert_eq!(reserver("  ", 2, 10), Err(ErreurReservation::NomVide));
        assert_eq!(
            reserver("Bob", 9, 50),
            Err(ErreurReservation::TropDePersonnes { demandees: 9 })
        );
        assert_eq!(
            reserver("Bob", 3, 2),
            Err(ErreurReservation::Complet {
                demandees: 3,
                restantes: 2
            })
        );
    }

    #[test]
    fn messages() {
        assert_eq!(ErreurReservation::NomVide.to_string(), "nom vide");
        assert_eq!(
            ErreurReservation::Complet {
                demandees: 3,
                restantes: 2
            }
            .to_string(),
            "complet : 3 places demandées, 2 restantes"
        );
    }

    #[test]
    fn entre_dans_box_dyn_error() {
        let erreur: Box<dyn Error> = Box::new(ErreurReservation::TropDePersonnes { demandees: 12 });
        assert_eq!(
            erreur.to_string(),
            "12 personnes demandées, 8 au plus par table"
        );
    }

    #[test]
    fn meme_resultat_que_le_naif() {
        ["Alice", "", " "].iter().for_each(|nom| {
            (0..12).for_each(|personnes| {
                (0..12).for_each(|restantes| {
                    assert_eq!(
                        reserver(nom, personnes, restantes).map_err(|e| e.to_string()),
                        reserver_naif(nom, personnes, restantes),
                        "{nom:?}, {personnes}, {restantes}"
                    )
                })
            })
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(include_str!("e02_reservation.rs"), &["reserver", "fmt"]);
    }
}
