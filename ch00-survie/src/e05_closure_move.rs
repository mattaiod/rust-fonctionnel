//! Exercice 5 : une fonction qui fabrique et rend une closure.
//!
//! `filtre_interdits` reçoit une liste de mots interdits et rend une closure qui dit si un mot en fait partie. La
//! closure survit à la fonction qui l'a créée : elle doit donc **posséder** la liste, d'où `move` (section 8). Le
//! `Vec<String>` passe à l'intérieur de la closure, qui le garde aussi longtemps qu'elle existe. La closure ne
//! fait que lire la liste : elle est `Fn`, on peut l'appeler autant de fois qu'on veut (section 9).

/// Version impérative : la liste est relue à chaque appel. Ne pas modifier.
pub fn est_interdit_imperatif(interdits: &[String], mot: &str) -> bool {
    for interdit in interdits {
        if interdit == mot {
            return true;
        }
    }
    false
}

/// **Aide Rust** (ch00, sections 8 et 9)
/// - Le `move` est déjà écrit. Sans lui : E0373 « closure may outlive the current function, but it borrows
///   `interdits`, which is owned by the current function ».
/// - Dans le corps, ne consommez pas `interdits` (pas de `into_iter()`) : la closure ne pourrait plus servir
///   qu'une fois et ne serait plus `Fn`, E0507 « cannot move out of `interdits`, a captured variable in an `Fn`
///   closure ». Lisez avec `iter()`.
/// - `interdit == mot` compare un `&String` à un `&str` : ça marche tel quel.
///
/// Une closure qui rend `true` si `mot` fait partie de `interdits`.
pub fn filtre_interdits(interdits: Vec<String>) -> impl Fn(&str) -> bool {
    move |mot| {
        // Outils : `interdits.iter()`, `any(|interdit| …)`.
        todo!()
    }
}

/// Version impérative. Ne pas modifier.
pub fn censurer_imperatif(texte: &str, interdits: Vec<String>) -> Vec<&str> {
    let mut gardes = Vec::new();
    for mot in texte.split_whitespace() {
        if !est_interdit_imperatif(&interdits, mot) {
            gardes.push(mot);
        }
    }
    gardes
}

/// **Aide Rust** (ch00, sections 4 et 8)
/// - Fabriquez la closure **une fois**, avant la chaîne : `let est_interdit = filtre_interdits(interdits);`. Le
///   `Vec` y est déplacé : `interdits` n'est plus utilisable ensuite (E0382 « borrow of moved value: `interdits` »), et c'est voulu.
/// - Dans `filter`, `mot` est un `&&str` et `est_interdit` attend un `&str` : `est_interdit(mot)` passe (le
///   compilateur retire une référence tout seul), sinon écrivez `|&mot|`.
///
/// Les mots de `texte` qui ne sont pas interdits, dans l'ordre.
pub fn censurer(texte: &str, interdits: Vec<String>) -> Vec<&str> {
    // Outils : `filtre_interdits(…)` dans un `let`, `split_whitespace()`, `filter`, `collect()`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    fn liste(mots: &[&str]) -> Vec<String> {
        mots.iter().map(|mot| mot.to_string()).collect()
    }

    #[test]
    fn cas_connus() {
        let est_interdit = filtre_interdits(liste(&["zut", "flûte"]));
        assert!(est_interdit("zut"));
        assert!(est_interdit("flûte"));
        assert!(!est_interdit("bonjour"));
        assert!(!est_interdit("Zut"));
        assert!(!filtre_interdits(Vec::new())("zut"));
        assert_eq!(
            censurer("zut alors flûte encore zut", liste(&["zut", "flûte"])),
            vec!["alors", "encore"]
        );
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        (0..20).for_each(|graine| {
            let interdits = jeux::mots(graine + 100, 5, 3);
            let texte = jeux::mots(graine, 30, 3).join(" ");
            let est_interdit = filtre_interdits(interdits.clone());
            texte.split_whitespace().for_each(|mot| {
                assert_eq!(
                    est_interdit(mot),
                    est_interdit_imperatif(&interdits, mot),
                    "mot : {mot:?}, interdits : {interdits:?}"
                )
            });
            assert_eq!(
                censurer(&texte, interdits.clone()),
                censurer_imperatif(&texte, interdits)
            );
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e05_closure_move.rs"),
            &["filtre_interdits", "censurer"],
        );
    }
}
