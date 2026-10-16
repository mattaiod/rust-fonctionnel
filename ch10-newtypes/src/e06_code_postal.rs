//! Exercice 6 : `CodePostal`, sans version naïve : à vous.
//!
//! Un code postal français : exactement 5 chiffres, et pas `00` en tête (aucun département 00). Ce n'est pas un
//! nombre : `01000` (Bourg-en-Bresse) perdrait son zéro dans un `u32`. Le département se lit sur les 2 premiers
//! chiffres, sauf outre-mer où les codes commencent par `97` ou `98` et le département en prend 3 (`97400` → `974`).
//!
//! Vous écrivez tout ce qui est entre les marqueurs : la règle (`try_from`), les deux portes d'entrée (`FromStr`),
//! l'affichage, et les lectures qui exploitent l'invariant.
//!
//! ```compile_fail,E0603
//! use ch10_newtypes::e06_code_postal::CodePostal;
//! let faux = CodePostal("ABCDE".to_string()); // E0603 : champ privé
//! ```

use std::fmt;
use std::str::FromStr;

/// Un code postal valide : 5 chiffres ASCII, ne commençant pas par `00`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CodePostal(String);

/// Ce qui ne va pas, testé dans cet ordre.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErreurCodePostal {
    /// Pas 5 caractères (après retrait des espaces autour) : la longueur trouvée.
    Longueur(usize),
    /// Le premier caractère qui n'est pas un chiffre.
    CaractereInterdit(char),
    /// Commence par `00`.
    DepartementInexistant,
}

impl TryFrom<&str> for CodePostal {
    type Error = ErreurCodePostal;

    /// **Aide Rust** (ch00, sections 1, 5)
    /// - `len()` compte des octets : `"5é100".len()` vaut 6, et le test attend `CaractereInterdit('é')`, pas
    ///   `Longueur(6)`. Comptez des caractères avec `chars().count()`.
    /// - `CodePostal(texte)` donne `E0308 … expected String, found &str` : `to_string()` au dernier moment.
    /// - Pour trouver le premier caractère fautif, `find` rend une `Option<char>` : `if let Some(c) = ...` puis
    ///   `return Err(...)`.
    fn try_from(texte: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl FromStr for CodePostal {
    type Err = ErreurCodePostal;

    fn from_str(texte: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl fmt::Display for CodePostal {
    /// **Aide Rust** (ch00, section 1)
    /// - Comme à l'exercice 1 : écrire dans `f` et rendre le `fmt::Result` (pas de `;` final), jamais
    ///   `write!(f, "{}", self)` qui se rappelle à l'infini.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl CodePostal {
    /// Le numéro de département : 2 chiffres, ou 3 outre-mer (codes en `97` et `98`).
    ///
    /// **Aide Rust** (ch00, sections 5, 13)
    /// - `self.0[..3]` est une `str` (sans `&`) : `E0308 … expected &str, found str`, corrigé par `&self.0[..3]`.
    /// - `&self.0[..3].to_string()` donne `E0515 cannot return reference to temporary value` : la `String` créée
    ///   meurt à la fin de la fonction. Prêtez un morceau de `self.0`, rien de neuf.
    pub fn departement(&self) -> &str {
        todo!()
    }

    /// Vrai pour les codes d'outre-mer.
    pub fn est_outre_mer(&self) -> bool {
        todo!()
    }
}

/// Les codes valides d'une liste saisie (séparés par des virgules), triés et sans doublon ; les invalides sont
/// ignorés.
///
/// **Aide Rust** (ch00, section 12)
/// - `collect()` suivi d'une autre méthode donne `E0282 type annotations needed` : dites dans quoi collecter, avec
///   `collect::<...>()`.
/// - Une collection de la bibliothèque standard trie et dédoublonne toute seule (`std::collections`). Sans `use`,
///   `E0425 cannot find type ... in this scope` : écrivez le chemin complet ou ajoutez le `use`.
pub fn codes_valides(saisie: &str) -> Vec<CodePostal> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::style;

    fn cp(texte: &str) -> CodePostal {
        texte.parse().unwrap()
    }

    #[test]
    fn cas_connus() {
        assert_eq!(cp("51100").to_string(), "51100");
        assert_eq!(cp(" 01000 ").to_string(), "01000");
        assert_eq!(cp("51100").departement(), "51");
        assert_eq!(cp("01000").departement(), "01");
        assert_eq!(cp("97400").departement(), "974");
        assert_eq!(cp("98800").departement(), "988");
        assert!(cp("97400").est_outre_mer());
        assert!(!cp("75001").est_outre_mer());
    }

    #[test]
    fn chaque_erreur_a_son_nom() {
        assert_eq!(
            CodePostal::try_from("5110"),
            Err(ErreurCodePostal::Longueur(4))
        );
        assert_eq!(
            CodePostal::try_from("511000"),
            Err(ErreurCodePostal::Longueur(6))
        );
        assert_eq!(CodePostal::try_from(""), Err(ErreurCodePostal::Longueur(0)));
        assert_eq!(
            CodePostal::try_from("2A004"),
            Err(ErreurCodePostal::CaractereInterdit('A'))
        );
        assert_eq!(
            CodePostal::try_from("5é100"),
            Err(ErreurCodePostal::CaractereInterdit('é'))
        );
        assert_eq!(
            CodePostal::try_from("00100"),
            Err(ErreurCodePostal::DepartementInexistant)
        );
        assert_eq!(
            "51 00".parse::<CodePostal>(),
            Err(ErreurCodePostal::CaractereInterdit(' '))
        );
    }

    #[test]
    fn liste_saisie() {
        assert_eq!(
            codes_valides("75001, 51100,abc, 01000, 51100,00000"),
            vec![cp("01000"), cp("51100"), cp("75001")]
        );
        assert_eq!(codes_valides(""), vec![]);
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e06_code_postal.rs"),
            &[
                "try_from",
                "from_str",
                "fmt",
                "departement",
                "est_outre_mer",
                "codes_valides",
            ],
        );
    }
}
