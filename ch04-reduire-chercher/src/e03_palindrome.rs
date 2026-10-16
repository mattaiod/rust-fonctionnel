//! Exercice 3 : palindromes.
//!
//! Inverser une chaîne : `chars()` puis `rev()` puis `collect::<String>()`. Jamais par octets : « été » fait
//! 3 caractères mais 5 octets.
//! Un palindrome se lit pareil dans les deux sens, en ignorant la casse, les espaces et la ponctuation :
//! « Engage le jeu que je le gagne. ». Les accents, eux, comptent : « é » et « e » sont deux lettres
//! différentes, donc « Ésope reste ici et se repose » n'en est pas un pour la machine.
//! On ne garde que les caractères alphanumériques (`char::is_alphanumeric`), passés en minuscules
//! (`char::to_lowercase`, qui rend un itérateur : d'où `flat_map`). Puis on compare la suite à son envers : deux
//! itérateurs se comparent élément par élément avec `eq`.

/// Version impérative. Ne pas modifier.
pub fn est_palindrome_imperatif(texte: &str) -> bool {
    let mut lettres = Vec::new();
    for c in texte.chars() {
        if c.is_alphanumeric() {
            for minuscule in c.to_lowercase() {
                lettres.push(minuscule);
            }
        }
    }
    let mut gauche = 0;
    let mut droite = lettres.len();
    while gauche + 1 < droite {
        if lettres[gauche] != lettres[droite - 1] {
            return false;
        }
        gauche += 1;
        droite -= 1;
    }
    true
}

/// La chaîne à l'envers.
///
/// **Aide Rust** (ch00, sections 3, 5)
/// - Si le compilateur dit ``E0599: `&str` is not an iterator`` : un `&str` ne se parcourt pas directement, passez
///   par `chars()`.
pub fn inverser(texte: &str) -> String {
    // Outils : `chars()`, `rev()`, `collect()`.
    todo!()
}

/// Vrai si le texte est un palindrome (casse, espaces et ponctuation ignorés).
///
/// **Aide Rust** (ch00, sections 3, 12)
/// - Si le compilateur dit
///   ``E0277: a value of type `Vec<char>` cannot be built from an iterator over elements of type `ToLowercase` `` :
///   `to_lowercase` peut rendre plusieurs caractères, d'où `flat_map(char::to_lowercase)` et non `map`.
/// - Si le compilateur dit ``E0382: use of moved value`` : un itérateur ne se parcourt qu'une fois. Rangez les
///   lettres dans un `Vec<char>` annoté (sinon ``E0282: type annotations needed``).
/// - Si le compilateur dit ``E0277: can't compare `Vec<char>` with `Rev<…>` `` : `==` compare deux valeurs du même
///   type ; deux itérateurs se comparent avec `.eq(…)`.
pub fn est_palindrome(texte: &str) -> bool {
    // Outils : `chars()`, `filter(|c| c.is_alphanumeric())`, `flat_map(char::to_lowercase)`, `collect::<Vec<char>>()`, puis `iter().eq(….iter().rev())`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    #[test]
    fn cas_connus() {
        assert_eq!(inverser("Rust"), "tsuR");
        assert_eq!(inverser("été !"), "! été");
        assert_eq!(inverser(""), "");
        assert!(est_palindrome("kayak"));
        assert!(est_palindrome("Engage le jeu que je le gagne."));
        assert!(est_palindrome("Was it a car or a cat I saw?"));
        assert!(!est_palindrome("Ésope reste ici et se repose"));
        assert!(!est_palindrome("Rust"));
        assert!(est_palindrome(""));
        assert!(est_palindrome("!?"));
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        (0..20).for_each(|graine| {
            jeux::mots(graine, 10, 2).iter().for_each(|mot| {
                let miroir = format!("{mot}, {}!", inverser(mot).to_uppercase());
                [mot.clone(), miroir].iter().for_each(|texte| {
                    assert_eq!(
                        est_palindrome(texte),
                        est_palindrome_imperatif(texte),
                        "texte : {texte:?}"
                    )
                });
            })
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e03_palindrome.rs"),
            &["inverser", "est_palindrome"],
        );
    }
}
