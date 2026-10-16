//! Exercice 3 : copier des noms, et seulement quand il le faut.
//!
//! On reçoit un `&[String]` : les `String` appartiennent à l'appelant. On peut les lire, pas les lui prendre.
//! Pour rendre un `Vec<String>`, il faut donc des copies : `.cloned()` transforme les `&String` en `String`
//! (c'est le `copied()` des types qui ne sont pas `Copy`). Ce clone-là est normal. Mais quand on fabrique de
//! toute façon une `String` neuve (`to_uppercase`), cloner avant serait du gaspillage.

/// Version impérative. Ne pas modifier.
pub fn noms_longs_imperatif(noms: &[String], longueur_min: usize) -> Vec<String> {
    let mut gardes = Vec::new();
    for nom in noms {
        if nom.chars().count() >= longueur_min {
            gardes.push(nom.clone());
        }
    }
    gardes
}

/// **Aide Rust** (ch00, section 6)
/// - `.map(|nom| *nom)` essaie de sortir la `String` de derrière la référence : E0507 « cannot move out of
///   `*nom` which is behind a shared reference ». Il faut une copie.
/// - `noms.iter().clone()` clone l'itérateur, pas les éléments : E0277 « a value of type `Vec<String>` cannot be
///   built from an iterator over elements of type `&String` ». C'est `cloned()`.
/// - Filtrez d'abord, clonez ensuite : `.cloned().filter(…)` recopierait aussi les noms qu'on jette.
///
/// Les noms d'au moins `longueur_min` caractères, dans l'ordre (copies).
pub fn noms_longs(noms: &[String], longueur_min: usize) -> Vec<String> {
    // Outils : `iter()`, `filter`, `chars().count()`, `cloned()`, `collect()`.
    todo!()
}

/// Version impérative. Ne pas modifier.
pub fn en_majuscules_imperatif(noms: &[String]) -> Vec<String> {
    let mut majuscules = Vec::new();
    for nom in noms {
        majuscules.push(nom.clone().to_uppercase());
    }
    majuscules
}

/// **Aide Rust** (ch00, section 6)
/// - La version impérative fait `nom.clone().to_uppercase()` : le `clone` est en trop, `to_uppercase` lit un
///   `&str` et rend déjà une `String` neuve. Aucun `clone` ni `cloned` dans votre version.
///
/// Les noms en majuscules, dans l'ordre.
pub fn en_majuscules(noms: &[String]) -> Vec<String> {
    // Outils : `iter()`, `map(|nom| nom.to_uppercase())`, `collect()`. Sans `clone`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    fn noms(liste: &[&str]) -> Vec<String> {
        liste.iter().map(|nom| nom.to_string()).collect()
    }

    #[test]
    fn cas_connus() {
        let classe = noms(&["Léa", "Maximilien", "Jo", "Hélène"]);
        assert_eq!(noms_longs(&classe, 4), noms(&["Maximilien", "Hélène"]));
        assert_eq!(noms_longs(&classe, 0), classe);
        assert_eq!(
            en_majuscules(&classe),
            noms(&["LÉA", "MAXIMILIEN", "JO", "HÉLÈNE"])
        );
        assert_eq!(en_majuscules(&[]), Vec::<String>::new());
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        (0..20).for_each(|graine| {
            let jeu = jeux::mots(graine, 25, 5);
            (0..10).for_each(|n| {
                assert_eq!(
                    noms_longs(&jeu, n),
                    noms_longs_imperatif(&jeu, n),
                    "jeu : {jeu:?}, n = {n}"
                )
            });
            assert_eq!(en_majuscules(&jeu), en_majuscules_imperatif(&jeu));
        });
    }

    #[test]
    fn style_fonctionnel() {
        let source = include_str!("e03_noms_clones.rs");
        style::verifier(source, &["noms_longs", "en_majuscules"]);
        let corps = source
            .split("pub fn en_majuscules(")
            .nth(1)
            .and_then(|suite| suite.split("\n}").next())
            .unwrap_or("");
        let code: String = corps
            .lines()
            .map(|ligne| ligne.split("//").next().unwrap_or(""))
            .collect();
        assert!(
            !code.contains("clone"),
            "`en_majuscules` n'a besoin d'aucun clone : to_uppercase rend déjà une String neuve"
        );
    }
}
