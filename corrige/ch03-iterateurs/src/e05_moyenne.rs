//! Exercice 5 : moyenne d'une liste, sans version impérative. À vous.
//!
//! La moyenne d'une liste vide n'existe pas. Plutôt que de rendre `0.0` (faux) ou `NaN` (piège), le type de
//! retour le dit : `Option<f64>`. `None` veut dire « pas de moyenne », `Some(m)` « la moyenne vaut `m` » (ch00,
//! section 0). Le chapitre 8 approfondit `Option`.
//!
//! Utile : `bool::then(|| …)` rend `Some(…)` si le booléen est vrai, `None` sinon.

/// Moyenne des notes, `None` si la liste est vide.
///
/// **Aide Rust** (ch00, sections 0, 12)
/// - Si le compilateur dit
///   ``E0283: type annotations needed … cannot infer type of the type parameter `S` declared on the method `sum` `` :
///   écrivez `sum::<f64>()`.
/// - Si le compilateur dit ``E0277: cannot divide `f64` by `usize` `` : `notes.len()` est un `usize`, convertissez-le
///   avec `as f64`.
/// - Si le compilateur dit ``E0308: expected `Option<f64>`, found `f64` `` : la valeur va dans un `Some(…)` (c'est ce
///   que fait `then`).
pub fn moyenne(notes: &[f64]) -> Option<f64> {
    // `then` prend une closure : la division n'est calculée que si la liste est non vide, jamais de `0.0 / 0.0` (NaN).
    // `sum::<f64>()` fixe le type de la somme, que la division ne permet pas de déduire ; `as f64` convertit la taille.
    (!notes.is_empty()).then(|| notes.iter().sum::<f64>() / notes.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::style;

    #[test]
    fn liste_vide() {
        assert_eq!(moyenne(&[]), None);
    }

    #[test]
    fn cas_connus() {
        assert_eq!(moyenne(&[12.0]), Some(12.0));
        assert_eq!(moyenne(&[10.0, 14.0, 18.0]), Some(14.0));
        assert_eq!(moyenne(&[-1.0, 1.0]), Some(0.0));
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(include_str!("e05_moyenne.rs"), &["moyenne"]);
    }
}
