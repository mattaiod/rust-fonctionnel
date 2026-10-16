//! Exercice 2 : compression RLE (run-length encoding).
//!
//! `"aaabcc"` devient `"a3b1c2"` : chaque suite de caractères identiques est remplacée par le caractère et sa
//! longueur. Les comptes peuvent avoir plusieurs chiffres (`"a12"`). La décompression lit une entrée qui vient de
//! l'extérieur : elle peut être fausse, elle rend donc un `Result`, avec une erreur qui dit précisément quoi.
//! La meilleure preuve que les deux fonctions sont justes : décompresser ce qu'on a compressé rend l'original.

/// Ce qui peut être faux dans un texte compressé.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurRle {
    /// Le texte commence par un compte, sans caractère devant (`"3a"`).
    CommenceParUnChiffre,
    /// Ce caractère n'est pas suivi de son compte (`"ab2"` : il manque le compte de `a`).
    CompteManquant(char),
    /// Ce compte n'est pas un entier strictement positif représentable (`"a0"`).
    CompteInvalide(String),
}

/// Compresse `texte`. Le texte ne doit pas contenir de chiffre : `"a1"` compressé en `"a111"` serait ambigu.
///
/// **Aide Rust** (ch00, sections 3, 5)
/// - `chunk_by` est une méthode des tranches (`&[T]`), pas des `&str` ni des itérateurs :
///   `E0599 no method named chunk_by found for reference &str` (ou `for struct Chars`). Collectez d'abord les
///   caractères dans un `Vec<char>`.
/// - Le résultat est une `String` : `collect()` sait concaténer des `String` en une seule. `collect::<Vec<String>>()`
///   donne `E0308 … expected String, found Vec<String>`.
pub fn compresser(texte: &str) -> String {
    todo!()
}

/// Décompresse `code`, ou dit pourquoi c'est impossible (la première erreur rencontrée).
///
/// **Aide Rust** (ch00, sections 3, 11, 12)
/// - Un plan en quatre marches : 1. `Vec<char>` ; 2. `chunk_by` qui regroupe les suites de « chiffre / pas chiffre »
///   (`a.is_ascii_digit() == b.is_ascii_digit()`), ce qui alterne lettres et comptes ; 3. si le premier groupe est un
///   compte, `CommenceParUnChiffre` ; 4. `chunks(2)` donne des paires `[lettres, chiffres]`, chacune devient un
///   `Result<String, ErreurRle>`, et `collect()` en `Result<String, _>` s'arrête à la première erreur.
/// - Un groupe de lettres de plus d'un caractère (`"ab2"`) veut dire qu'une lettre n'a pas de compte : c'est
///   `CompteManquant` de sa première lettre. Une paire réduite à `[lettres]` aussi.
/// - `'a'.repeat(n)` donne `E0599 no method named repeat found for type char` : `repeat` est une méthode de `str`,
///   passez par `to_string()`.
pub fn decompresser(code: &str) -> Result<String, ErreurRle> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    #[test]
    fn compresser_cas_connus() {
        assert_eq!(compresser("aaabcc"), "a3b1c2");
        assert_eq!(compresser(""), "");
        assert_eq!(compresser("abc"), "a1b1c1");
        assert_eq!(compresser("ééé"), "é3");
        assert_eq!(compresser(&"z".repeat(12)), "z12");
    }

    #[test]
    fn decompresser_cas_connus() {
        assert_eq!(decompresser("a3b1c2"), Ok("aaabcc".to_string()));
        assert_eq!(decompresser(""), Ok(String::new()));
        assert_eq!(decompresser("z12"), Ok("z".repeat(12)));
        assert_eq!(decompresser("é2a1"), Ok("ééa".to_string()));
    }

    #[test]
    fn refuse_un_debut_par_un_chiffre() {
        assert_eq!(decompresser("3a"), Err(ErreurRle::CommenceParUnChiffre));
    }

    #[test]
    fn refuse_un_compte_manquant() {
        assert_eq!(decompresser("ab2"), Err(ErreurRle::CompteManquant('a')));
        assert_eq!(decompresser("a2b"), Err(ErreurRle::CompteManquant('b')));
        assert_eq!(decompresser("x"), Err(ErreurRle::CompteManquant('x')));
    }

    #[test]
    fn refuse_un_compte_invalide() {
        assert_eq!(
            decompresser("a0"),
            Err(ErreurRle::CompteInvalide("0".to_string()))
        );
        let enorme = "9".repeat(30);
        assert_eq!(
            decompresser(&format!("a{enorme}")),
            Err(ErreurRle::CompteInvalide(enorme))
        );
    }

    #[test]
    fn aller_retour() {
        (0..30).for_each(|graine| {
            let texte = jeux::mots(graine, 20, 3).concat();
            assert_eq!(
                decompresser(&compresser(&texte)),
                Ok(texte.clone()),
                "texte : {texte:?}"
            );
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(include_str!("e02_rle.rs"), &["compresser", "decompresser"]);
    }
}
