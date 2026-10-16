//! Exercice 1 : une fabrique de remises.
//!
//! La version impérative passe le pourcentage à chaque appel. La version fonctionnelle le fixe une fois :
//! `remise(20.0)` rend une closure « prix → prix remisé de 20 % », qu'on peut nommer, stocker, passer à `map`.
//! La closure utilise `pourcent`, un paramètre qui meurt à la fin de `remise` alors qu'elle lui survit : d'où le
//! `move`, qui copie `pourcent` DANS la closure. Retirez-le pour voir l'erreur `E0373`.
//! Formule : `prix × (1 − pourcent / 100)`.

/// Le prix remisé. Ne pas modifier.
pub fn prix_remise(prix: f64, pourcent: f64) -> f64 {
    prix * (1.0 - pourcent / 100.0)
}

/// Version impérative : la fonction à deux paramètres appelée en boucle. Ne pas modifier.
pub fn appliquer_remise_imperatif(prix: &[f64], pourcent: f64) -> Vec<f64> {
    let mut remises = Vec::new();
    for p in prix {
        remises.push(prix_remise(*p, pourcent));
    }
    remises
}

/// Une fonction « prix → prix remisé de `pourcent` % ».
///
/// **Aide Rust** (ch00, sections 0, 8)
/// - Tout est en `f64` : écrivez `1.0` et `100.0`, pas `1` et `100`. Sinon :
///   ``E0277: cannot divide `f64` by `{integer}` ``.
/// - Le `move` est déjà écrit ; sans lui :
///   ``E0373: closure may outlive the current function, but it borrows `pourcent` ``.
pub fn remise(pourcent: f64) -> impl Fn(f64) -> f64 {
    move |prix| {
        // Le corps de la closure : la formule, avec le `pourcent` capturé.
        todo!()
    }
}

/// Version fonctionnelle : la remise fabriquée une fois, appliquée à chaque prix.
///
/// **Aide Rust** (ch00, sections 3, 9)
/// - `prix.iter()` donne des `&f64`, et la closure de `remise` attend un `f64`.
/// - Si le compilateur dit ``E0277: expected a `FnMut(&f64)` closure, found `impl Fn(f64) -> f64` `` : ajoutez
///   `.copied()` avant le `map`.
/// - `map(remise(pourcent))` passe la closure elle-même : pas de `|p| …` autour.
pub fn appliquer_remise(prix: &[f64], pourcent: f64) -> Vec<f64> {
    // Outils : `iter()`, `copied()`, `map(remise(pourcent))`, `collect()`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    fn proches(a: &[f64], b: &[f64]) -> bool {
        a.len() == b.len() && a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-9)
    }

    #[test]
    fn cas_connus() {
        let soldes = remise(20.0);
        assert!((soldes(100.0) - 80.0).abs() < 1e-9);
        assert!((soldes(50.0) - 40.0).abs() < 1e-9);
        assert!((remise(0.0)(12.5) - 12.5).abs() < 1e-9);
        assert!(proches(
            &appliquer_remise(&[10.0, 200.0], 50.0),
            &[5.0, 100.0]
        ));
        assert!(appliquer_remise(&[], 10.0).is_empty());
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        jeux::plusieurs(0, 10_000).iter().for_each(|jeu| {
            let prix: Vec<f64> = jeu.iter().map(|n| *n as f64 / 100.0).collect();
            [0.0, 5.0, 33.0, 100.0].iter().for_each(|pourcent| {
                assert!(
                    proches(
                        &appliquer_remise(&prix, *pourcent),
                        &appliquer_remise_imperatif(&prix, *pourcent)
                    ),
                    "prix : {prix:?}, remise : {pourcent}"
                )
            })
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e01_remise.rs"),
            &["remise", "appliquer_remise"],
        );
    }
}
