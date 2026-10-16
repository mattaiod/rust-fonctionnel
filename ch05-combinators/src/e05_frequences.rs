//! Exercice 5 : combien de fois chaque mot apparaît.
//!
//! On replie le texte dans une `HashMap<String, usize>` : l'accumulateur du `fold` est la table elle-même.
//! Modifier la table à chaque étape est permis ici : `|mut acc, mot|` déclare que la closure possède l'accumulateur
//! et le rend à la fin, personne d'autre ne le voit changer (mutation locale, la fonction reste pure).
//! Pour « ajouter 1, ou partir de 0 si la clé n'existe pas », l'API `entry` évite le `if contains_key` :
//! `*acc.entry(cle).or_insert(0) += 1`.
//! On compte les mots en minuscules : « Rust » et « rust » sont le même mot.

use std::collections::HashMap;

/// Version impérative : un `if` pour savoir si la clé existe déjà. Ne pas modifier.
pub fn frequences_imperatif(texte: &str) -> HashMap<String, usize> {
    let mut table = HashMap::new();
    for mot in texte.split_whitespace() {
        let mot = mot.to_lowercase();
        if let Some(compte) = table.get_mut(&mot) {
            *compte += 1;
        } else {
            table.insert(mot, 1);
        }
    }
    table
}

/// Nombre d'occurrences de chaque mot (séparé par des blancs), en minuscules.
///
/// **Aide Rust** (ch00, sections 5, 7, 12)
/// - L'accumulateur du `fold` est modifié sur place : il faut `|mut table, mot|`, et la closure doit **rendre** la
///   table à la fin.
/// - Si le compilateur dit `` error[E0596]: cannot borrow `table` as mutable, as it is not declared as mutable `` :
///   ajoutez `mut` devant le paramètre de la closure (c'est permis ; `let mut` ne l'est pas).
/// - Si le compilateur dit `` error[E0308]: mismatched types … expected `String`, found `&str` `` sur `entry(mot)` : la
///   table a des clés `String`, `split_whitespace` donne des `&str`. `to_lowercase()` rend justement une `String` :
///   faites-la avant `entry`.
/// - Si le compilateur dit `` expected `HashMap<String, usize>`, found `()` `` : la dernière ligne de la closure doit
///   être `table`, sans point-virgule.
pub fn frequences(texte: &str) -> HashMap<String, usize> {
    // Replier les mots dans une table : pour chaque mot, incrémenter sa case, en la créant à 0 si besoin.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    #[test]
    fn cas_connus() {
        let table = frequences("le chat et le chien et LE rat");
        assert_eq!(table.len(), 5);
        assert_eq!(table["le"], 3);
        assert_eq!(table["et"], 2);
        assert_eq!(table["chat"], 1);
        assert_eq!(table.get("souris"), None);
    }

    #[test]
    fn texte_vide() {
        assert!(frequences("").is_empty());
        assert!(frequences("   \n ").is_empty());
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        (0..20).for_each(|graine| {
            let texte = jeux::mots(graine, 40, 3).join(" ");
            assert_eq!(
                frequences(&texte),
                frequences_imperatif(&texte),
                "texte : {texte:?}"
            )
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(include_str!("e05_frequences.rs"), &["frequences"]);
    }
}
