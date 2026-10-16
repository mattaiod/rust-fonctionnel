//! Exercice 1 : nettoyer un fichier de configuration, en pipeline de fonctions nommées.
//!
//! Chaque ligne passe par les mêmes étapes : retirer les blancs autour, jeter les lignes vides et les commentaires
//! (`#` en début de ligne), passer en minuscules. La boucle impérative mélange les quatre. Le pipeline les aligne,
//! et quand chaque étape est une **fonction nommée**, on la passe sans closure : `.map(str::trim)` plutôt que
//! `.map(|l| l.trim())`. C'est le style *point-free* : le code dit quoi faire, sans nommer la donnée qui passe.
//! Le piège : `filter` donne à son prédicat une référence vers l'élément. Sur des `&str`, il reçoit un `&&str`, d'où
//! la signature de `est_utile`.

/// Version impérative : quatre étapes emmêlées dans une boucle. Ne pas modifier.
pub fn nettoyer_imperatif(texte: &str) -> Vec<String> {
    let mut lignes = Vec::new();
    for ligne in texte.lines() {
        let ligne = ligne.trim();
        if ligne.is_empty() || ligne.starts_with('#') {
            continue;
        }
        lignes.push(ligne.to_lowercase());
    }
    lignes
}

/// Vraie si la ligne (déjà débarrassée de ses blancs) n'est ni vide ni un commentaire.
///
/// **Aide Rust** (ch00, section 4)
/// - Le paramètre est un `&&str` (une référence vers une `&str`) : `ligne.is_empty()` et `ligne.starts_with('#')`
///   marchent quand même, l'appel de méthode déréférence tout seul.
pub fn est_utile(ligne: &&str) -> bool {
    // Une ligne utile n'est pas vide et ne commence pas par `#`.
    todo!()
}

/// Le pipeline : chaque étape est une fonction nommée, aucune closure.
///
/// **Aide Rust** (ch00, sections 4, 5, 12)
/// - L'ordre des étapes compte pour les types : `est_utile` attend un `&&str`, donc le `filter` doit venir **avant**
///   `str::to_lowercase`, qui fabrique des `String`.
/// - Si le compilateur dit `` error[E0631]: type mismatch in function arguments … expected function signature `for<'a>
///   fn(&'a String) -> _` `` : vous filtrez des `String` avec `est_utile`. Filtrez avant la mise en minuscules.
/// - Si le compilateur dit `` error[E0277]: a value of type `Vec<String>` cannot be built from an iterator over
///   elements of type `&str` `` : il manque l'étape qui fabrique les `String`.
pub fn nettoyer(texte: &str) -> Vec<String> {
    // Découper en lignes, puis enchaîner les étapes en passant des fonctions par leur nom (`str::…`, `est_utile`), sans écrire de `|…|`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    #[test]
    fn cas_connus() {
        let config = "# Réglages\n  Port = 8080\n\n\tHOTE = Localhost  \n   # fin\n";
        assert_eq!(nettoyer(config), vec!["port = 8080", "hote = localhost"]);
        assert_eq!(nettoyer(""), Vec::<String>::new());
    }

    #[test]
    fn est_utile_cas_connus() {
        assert!(est_utile(&"a = 1"));
        assert!(!est_utile(&""));
        assert!(!est_utile(&"# commentaire"));
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        (0..20).for_each(|graine| {
            // Des lignes de mots, dont certaines vides ou commentées, avec des blancs autour.
            let texte = jeux::mots(graine, 30, 4)
                .iter()
                .enumerate()
                .map(|(i, mot)| match i % 5 {
                    0 => format!("# {mot}"),
                    1 => "   ".to_string(),
                    2 => format!("  {}  ", mot.to_uppercase()),
                    _ => mot.clone(),
                })
                .collect::<Vec<_>>()
                .join("\n");
            assert_eq!(
                nettoyer(&texte),
                nettoyer_imperatif(&texte),
                "texte : {texte:?}"
            );
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(include_str!("e01_nettoyer.rs"), &["est_utile", "nettoyer"]);
    }
}
