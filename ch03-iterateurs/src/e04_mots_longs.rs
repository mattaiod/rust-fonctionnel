//! Exercice 4 : les mots d'au moins `longueur_min` lettres.
//!
//! Le prédicat de `filter` est une closure qui CAPTURE `longueur_min` : elle lit une variable de la fonction qui
//! l'entoure (chapitre 2). La longueur se compte en caractères (`chars().count()`), pas en octets (`len()`) :
//! « élève » fait 5 caractères mais 7 octets en UTF-8.

/// Version impérative. Ne pas modifier.
pub fn mots_longs_imperatif(texte: &str, longueur_min: usize) -> Vec<String> {
    let mut mots = Vec::new();
    for mot in texte.split_whitespace() {
        if mot.chars().count() >= longueur_min {
            mots.push(mot.to_string());
        }
    }
    mots
}

/// Version fonctionnelle.
///
/// **Aide Rust** (ch00, sections 5, 12)
/// - `split_whitespace` donne des `&str` (des morceaux de `texte`) ; le retour veut des `String` possédées :
///   convertissez chaque mot (`.to_string()` ou `String::from`).
/// - Si le compilateur dit
///   ``E0277: a value of type `Vec<String>` cannot be built from an iterator over elements of type `&str` `` : la
///   conversion manque ; `map(|s| s)` ne convertit rien.
/// - `mot.len()` compile mais compte des octets : le test `meme_resultat_que_l_imperatif` échoue sur les accents.
///   Comptez avec `chars().count()`.
pub fn mots_longs(texte: &str, longueur_min: usize) -> Vec<String> {
    // Outils : `split_whitespace`, `filter`, `map`, `collect`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    #[test]
    fn cas_connus() {
        assert_eq!(
            mots_longs("le petit élève apprend Rust", 5),
            vec!["petit", "élève", "apprend"]
        );
        assert_eq!(mots_longs("  ", 1), Vec::<String>::new());
        assert_eq!(mots_longs("a bb ccc", 0), vec!["a", "bb", "ccc"]);
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        (0..20).for_each(|graine| {
            let texte = jeux::mots(graine, 30, 6).join(" ");
            (0..10).for_each(|n| {
                assert_eq!(
                    mots_longs(&texte, n),
                    mots_longs_imperatif(&texte, n),
                    "texte : {texte:?}, n = {n}"
                )
            });
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(include_str!("e04_mots_longs.rs"), &["mots_longs"]);
    }
}
