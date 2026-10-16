//! Exercice 5 : valider un CSV ligne à ligne, sans version naïve : à vous.
//!
//! Un fichier de mesures arrive d'un capteur. On veut soit toutes les mesures (si tout est bon), soit TOUTES les
//! lignes fautives avec leur numéro, pour que l'expéditeur corrige tout en une fois. `partition(Result::is_ok)`
//! sépare un itérateur de `Result` en deux `Vec`. Le piège : les numéros de ligne. Numérotez AVANT de filtrer
//! les lignes vides, sinon « ligne 4 » ne désigne plus la quatrième ligne du fichier.

use std::error::Error;
use std::fmt;

/// Une mesure valide : un capteur nommé et une valeur finie.
#[derive(Debug, Clone, PartialEq)]
pub struct Mesure {
    pub capteur: String,
    pub valeur: f64,
}

/// Ce qui ne va pas dans une ligne.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurMesure {
    /// Il faut exactement 2 champs séparés par `,` ; on garde le nombre reçu.
    NombreDeChamps(usize),
    CapteurVide,
    /// La valeur ne se lit pas comme un `f64` ; on garde le texte (sans les blancs autour).
    ValeurIllisible(String),
    /// `"NaN"` et `"inf"` se lisent très bien comme des `f64`… et n'ont rien à faire dans une mesure.
    ValeurNonFinie,
}

/// Une erreur et la ligne du fichier où elle est (la première ligne, l'en-tête, porte le numéro 1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErreurLigne {
    pub numero: usize,
    pub erreur: ErreurMesure,
}

impl fmt::Display for ErreurMesure {
    /// « 3 champs au lieu de 2 », « capteur vide », « valeur illisible : abc », « valeur non finie ».
    ///
    /// **Aide Rust** (ch00, section 1)
    /// - Un bras par variante, un `write!(f, ...)` chacun, sans `;` final. `{n}` marche dans la chaîne si `n` vient
    ///   du motif.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Error for ErreurMesure {}

impl fmt::Display for ErreurLigne {
    /// « ligne 4 : capteur vide ».
    ///
    /// **Aide Rust** (ch00, section 1)
    /// - `{self.numero}` dans la chaîne de format donne `invalid format string: field access isn't supported` :
    ///   passez les champs en arguments, `write!(f, "... {} ...", self.numero, ...)`. `self.erreur` s'affiche avec
    ///   `{}` grâce au `Display` écrit juste au-dessus.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Error for ErreurLigne {}

/// Lit une ligne `capteur,valeur` (blancs autour de chaque champ tolérés).
///
/// **Aide Rust** (ch00, sections 5, 12)
/// - `match` sur un `Vec` avec un motif `[capteur, valeur]` donne
///   `E0529 expected an array or slice, found Vec<&str>` : matchez `champs.as_slice()`.
/// - `valeur.parse()?` donne `E0271 … expected ErreurMesure, found ParseFloatError` : traduisez avec `map_err`. Et
///   `"NaN"` se parse très bien : testez `is_finite()` ensuite.
/// - Le motif donne des `&&str` : `capteur: c` donne `E0308 … expected String, found &&str`, d'où `c.to_string()`.
pub fn lire_mesure(ligne: &str) -> Result<Mesure, ErreurMesure> {
    todo!()
}

/// Toutes les mesures du fichier, ou toutes les erreurs avec leur numéro de ligne. La première ligne est un
/// en-tête, ignoré ; les lignes vides (ou blanches) aussi.
///
/// **Aide Rust** (ch00, sections 3, 12)
/// - `partition` ne sait pas dans quoi ranger : `E0282 type annotations needed for (_, _)`. Annotez
///   `let (bonnes, mauvaises): (Vec<_>, Vec<_>) = ...`.
/// - Les deux `Vec` contiennent encore des `Result`
///   (`E0308 … expected Vec<Mesure>, found Vec<Result<Mesure, ErreurLigne>>`) : il reste à extraire les `Ok` d'un
///   côté, les `Err` de l'autre.
/// - `enumerate()` avant de sauter l'en-tête et de filtrer les lignes vides, et `+ 1` : les numéros commencent à 1.
pub fn valider_csv(contenu: &str) -> Result<Vec<Mesure>, Vec<ErreurLigne>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::style;

    fn mesure(capteur: &str, valeur: f64) -> Mesure {
        Mesure {
            capteur: capteur.to_string(),
            valeur,
        }
    }

    #[test]
    fn une_ligne() {
        assert_eq!(lire_mesure(" t1 , 21.5 "), Ok(mesure("t1", 21.5)));
        assert_eq!(lire_mesure("t1"), Err(ErreurMesure::NombreDeChamps(1)));
        assert_eq!(lire_mesure("t1,2,3"), Err(ErreurMesure::NombreDeChamps(3)));
        assert_eq!(lire_mesure(" ,2"), Err(ErreurMesure::CapteurVide));
        assert_eq!(
            lire_mesure("t1, abc"),
            Err(ErreurMesure::ValeurIllisible("abc".to_string()))
        );
        assert_eq!(lire_mesure("t1,NaN"), Err(ErreurMesure::ValeurNonFinie));
        assert_eq!(lire_mesure("t1,-inf"), Err(ErreurMesure::ValeurNonFinie));
    }

    #[test]
    fn fichier_valide() {
        let contenu = "capteur,valeur\nt1,21.5\n\nt2,-3\n";
        assert_eq!(
            valider_csv(contenu),
            Ok(vec![mesure("t1", 21.5), mesure("t2", -3.0)])
        );
    }

    #[test]
    fn en_tete_seul_ou_vide() {
        assert_eq!(valider_csv("capteur,valeur"), Ok(vec![]));
        assert_eq!(valider_csv(""), Ok(vec![]));
    }

    #[test]
    fn toutes_les_erreurs_avec_le_bon_numero() {
        let contenu = "capteur,valeur\nt1,21.5\n\n,4\nt3,abc\nt4,1\n   \nt5\nt6,inf";
        assert_eq!(
            valider_csv(contenu),
            Err(vec![
                ErreurLigne {
                    numero: 4,
                    erreur: ErreurMesure::CapteurVide
                },
                ErreurLigne {
                    numero: 5,
                    erreur: ErreurMesure::ValeurIllisible("abc".to_string())
                },
                ErreurLigne {
                    numero: 8,
                    erreur: ErreurMesure::NombreDeChamps(1)
                },
                ErreurLigne {
                    numero: 9,
                    erreur: ErreurMesure::ValeurNonFinie
                },
            ])
        );
    }

    #[test]
    fn messages() {
        let erreur = ErreurLigne {
            numero: 5,
            erreur: ErreurMesure::ValeurIllisible("abc".to_string()),
        };
        assert_eq!(erreur.to_string(), "ligne 5 : valeur illisible : abc");
        assert_eq!(
            ErreurMesure::NombreDeChamps(3).to_string(),
            "3 champs au lieu de 2"
        );
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(include_str!("e05_csv.rs"), &["lire_mesure", "valider_csv"]);
    }
}
