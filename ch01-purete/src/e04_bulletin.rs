//! Exercice 4 : séparer le calcul de l'affichage (« functional core, imperative shell »).
//!
//! `bulletin_imperatif` fait tout à la fois : il calcule, il écrit sur la sortie standard et il compte. Pour le
//! tester, il faudrait capturer ce qu'il affiche. On le découpe : un cœur pur qui FABRIQUE le texte et le compte
//! (testable avec un simple `assert_eq!`), et une coquille de deux lignes, `afficher_bulletin`, qui se contente
//! d'imprimer. Le format d'une ligne : `"Ada : 14/20, admis"` ou `"Alan : 9/20, ajourné"` (admis à partir de 10).
//! Le bulletin complet : les lignes, chacune terminée par `'\n'`.

/// Version impérative : calcule, affiche et compte dans la même boucle. Ne pas modifier.
pub fn bulletin_imperatif(eleves: &[(String, i64)]) -> usize {
    let mut admis = 0;
    for (nom, note) in eleves {
        if *note >= 10 {
            println!("{nom} : {note}/20, admis");
            admis += 1;
        } else {
            println!("{nom} : {note}/20, ajourné");
        }
    }
    admis
}

/// La coquille impérative : la seule fonction qui touche au monde extérieur. Rien à écrire ici.
pub fn afficher_bulletin(eleves: &[(String, i64)]) -> usize {
    print!("{}", bulletin(eleves));
    nombre_admis(eleves)
}

/// Une ligne du bulletin, sans retour à la ligne.
///
/// **Aide Rust** (ch00, section 5)
/// - Un littéral `"admis"` est un `&str` ; `format!` fabrique la `String` du retour.
/// - Si le compilateur dit ``E0308: `if` and `else` have incompatible types`` : les deux branches doivent rendre le
///   même type (deux `&str`, par exemple).
/// - Si le compilateur dit ``E0308: expected `String`, found `&str` `` : vous rendez un `&str` ; passez par `format!`
///   ou `.to_string()`.
pub fn ligne_bulletin(nom: &str, note: i64) -> String {
    // Outils : un `if` qui choisit le mot, `format!`.
    todo!()
}

/// Le bulletin complet : une ligne par élève, chacune suivie de `'\n'`.
///
/// **Aide Rust** (ch00, sections 2, 3, 5)
/// - Sur `eleves.iter()`, le motif `|(nom, note)|` donne `nom: &String` et `note: &i64`. `nom` passe tel quel là où
///   on attend un `&str` ; `note`, non.
/// - Si le compilateur dit ``E0308: expected `i64`, found `&i64` `` : passez `*note`.
/// - `collect()` sait coller des `String` en une seule : le type de retour suffit à le guider.
pub fn bulletin(eleves: &[(String, i64)]) -> String {
    // Outils : `iter()`, `map(|(nom, note)| …)`, `format!("{}\n", …)`, `collect::<String>()`.
    todo!()
}

/// Le nombre d'élèves admis (note d'au moins 10).
///
/// **Aide Rust** (ch00, section 4)
/// - Dans `filter`, l'élément arrive par référence : avec `|(_, note)|`, `note` est un `&i64`.
/// - Si le compilateur dit ``E0308: expected `&i64`, found integer`` : comparez `*note >= 10`.
pub fn nombre_admis(eleves: &[(String, i64)]) -> usize {
    // Outils : `iter()`, `filter`, `count()`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    fn classe(notes: &[i64]) -> Vec<(String, i64)> {
        notes
            .iter()
            .enumerate()
            .map(|(i, note)| (format!("élève {i}"), *note))
            .collect()
    }

    #[test]
    fn cas_connus() {
        assert_eq!(ligne_bulletin("Ada", 14), "Ada : 14/20, admis");
        assert_eq!(ligne_bulletin("Alan", 9), "Alan : 9/20, ajourné");
        assert_eq!(ligne_bulletin("Grace", 10), "Grace : 10/20, admis");
        let eleves = vec![("Ada".to_string(), 14), ("Alan".to_string(), 9)];
        assert_eq!(
            bulletin(&eleves),
            "Ada : 14/20, admis\nAlan : 9/20, ajourné\n"
        );
        assert_eq!(nombre_admis(&eleves), 1);
    }

    #[test]
    fn classe_vide() {
        assert_eq!(bulletin(&[]), "");
        assert_eq!(nombre_admis(&[]), 0);
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        jeux::plusieurs(0, 20).iter().for_each(|notes| {
            let eleves = classe(notes);
            assert_eq!(
                nombre_admis(&eleves),
                bulletin_imperatif(&eleves),
                "notes : {notes:?}"
            );
            assert_eq!(bulletin(&eleves).lines().count(), eleves.len());
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e04_bulletin.rs"),
            &["ligne_bulletin", "bulletin", "nombre_admis"],
        );
    }
}
