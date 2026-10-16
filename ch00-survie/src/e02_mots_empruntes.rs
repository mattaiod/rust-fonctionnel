//! Exercice 2 : les mots qui commencent par une lettre, sans rien recopier.
//!
//! La version impérative rend des `String` : chaque mot est recopié dans une allocation neuve. Or les mots
//! existent déjà, dans `texte`. Les morceaux que rend `split_whitespace` sont des `&str` qui pointent dans
//! `texte` : on peut les rendre tels quels, dans un `Vec<&str>`. Le compilateur comprend seul que ces `&str`
//! vivent aussi longtemps que `texte` (une seule référence en entrée : durée de vie élidée, section 13).

/// Version impérative : une `String` neuve par mot gardé. Ne pas modifier.
pub fn commencant_par_imperatif(texte: &str, lettre: char) -> Vec<String> {
    let mut mots = Vec::new();
    for mot in texte.split_whitespace() {
        if mot.starts_with(lettre) {
            mots.push(mot.to_string());
        }
    }
    mots
}

/// **Aide Rust** (ch00, sections 5 et 13)
/// - Le type de retour est `Vec<&str>` : pas de `to_string()`, pas de `String`. Avec un `.map(|mot|
///   mot.to_string())`, E0277 « a value of type `Vec<&str>` cannot be built from an iterator over elements of
///   type `String` ».
/// - Dans `filter`, `mot` est un `&&str`. L'appel de méthode `mot.starts_with(lettre)` déréférence tout seul :
///   aucune étoile à écrire.
/// - Ne transformez pas le texte avant de le découper (`texte.to_lowercase().split_whitespace()`) : les mots
///   pointeraient dans une `String` locale détruite à la fin de la fonction, E0515 « cannot return value
///   referencing temporary value ».
///
/// Les mots de `texte` qui commencent par `lettre` (sensible à la casse), dans l'ordre, empruntés à `texte`.
pub fn commencant_par(texte: &str, lettre: char) -> Vec<&str> {
    // Outils : `split_whitespace()`, `filter(|mot| …)`, `starts_with`, `collect()`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    #[test]
    fn cas_connus() {
        assert_eq!(
            commencant_par("le lapin lit le livre au lit", 'l'),
            vec!["le", "lapin", "lit", "le", "livre", "lit"]
        );
        assert_eq!(commencant_par("Élise et élodie", 'é'), vec!["élodie"]);
        assert_eq!(commencant_par("   ", 'a'), Vec::<&str>::new());
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        (0..20).for_each(|graine| {
            let texte = jeux::mots(graine, 30, 6).join(" ");
            "abcdefz".chars().for_each(|lettre| {
                assert_eq!(
                    commencant_par(&texte, lettre),
                    commencant_par_imperatif(&texte, lettre),
                    "texte : {texte:?}, lettre : {lettre}"
                )
            });
        });
    }

    #[test]
    fn rend_des_morceaux_du_texte_sans_copie() {
        let texte = String::from("alpha beta avion");
        let mots = commencant_par(&texte, 'a');
        let debut = texte.as_ptr() as usize;
        let fin = debut + texte.len();
        assert!(
            mots.iter()
                .all(|mot| (debut..fin).contains(&(mot.as_ptr() as usize))),
            "chaque &str rendu doit pointer dans `texte`"
        );
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(include_str!("e02_mots_empruntes.rs"), &["commencant_par"]);
    }
}
