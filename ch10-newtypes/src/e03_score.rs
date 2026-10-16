//! Exercice 3 : `Score` : un `f64` fini, donc triable.
//!
//! `f64` n'implémente pas `Ord` : `NaN` n'est ni plus petit, ni plus grand, ni égal à quoi que ce soit (même pas à
//! lui-même). D'où `partial_cmp`, qui rend une `Option`, et le `unwrap()` naïf qui panique le jour où un `NaN`
//! arrive. Le newtype exclut `NaN` et les infinis à la construction ; sur les valeurs qui restent, l'ordre total
//! `f64::total_cmp` est honnête, et `Score` peut implémenter `Ord` : `sort`, `max`, `BTreeMap` deviennent permis.
//!
//! Piège : `total_cmp` distingue `-0.0` de `0.0` (`-0.0 < 0.0`), alors que `-0.0 == 0.0` en `f64`. On normalise donc
//! `-0.0` en `0.0` à la construction, pour que deux scores « égaux » le soient aussi pour `Ord`.

use std::cmp::Ordering;

/// Version naïve : le classement, du meilleur au moins bon. Panique sur `NaN`. Ne pas modifier.
pub fn classer_naif(scores: &[f64]) -> Vec<f64> {
    let mut tries = scores.to_vec();
    tries.sort_by(|a, b| b.partial_cmp(a).unwrap());
    tries
}

/// Un score fini (ni `NaN`, ni infini), `-0.0` ramené à `0.0`.
#[derive(Debug, Clone, Copy)]
pub struct Score(f64);

/// La valeur refusée (`NaN` ou infinie).
#[derive(Debug, Clone, Copy)]
pub struct NonFini(pub f64);

impl TryFrom<f64> for Score {
    type Error = NonFini;

    /// **Aide Rust** (ch00, section 1)
    /// - `valeur == f64::NAN` est toujours faux (avertissement `incorrect NaN comparison`). `is_finite()` écarte
    ///   `NaN` et les deux infinis d'un coup.
    /// - `-0.0 == 0.0` est vrai en `f64` : un simple test d'égalité à `0.0` suffit pour ramener le zéro négatif à
    ///   `0.0`.
    fn try_from(valeur: f64) -> Result<Self, Self::Error> {
        // But : refuser le non-fini, normaliser le zéro négatif.
        todo!()
    }
}

impl Ord for Score {
    /// **Aide Rust** (ch00, section 1)
    /// - `self.0.cmp(&autre.0)` donne un message trompeur, `E0599 f64 is not an iterator` : `f64` n'a pas de `cmp`
    ///   (il n'est pas `Ord`), le compilateur a trouvé celui des itérateurs. `partial_cmp` rend une
    ///   `Option<Ordering>` (`E0308 … expected Ordering, found Option<Ordering>`). La méthode des flottants qui rend
    ///   un `Ordering` est dans le cours de l'exercice.
    fn cmp(&self, autre: &Self) -> Ordering {
        // But : l'ordre total des flottants, honnête ici grâce à l'invariant.
        todo!()
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, autre: &Self) -> Option<Ordering> {
        Some(self.cmp(autre))
    }
}

impl PartialEq for Score {
    fn eq(&self, autre: &Self) -> bool {
        self.cmp(autre) == Ordering::Equal
    }
}

impl Eq for Score {}

impl Score {
    /// La valeur.
    pub fn valeur(self) -> f64 {
        // But : lire le champ.
        todo!()
    }
}

/// Le classement, du meilleur au moins bon, ou la première valeur non finie rencontrée.
///
/// **Aide Rust** (ch00, sections 3, 12)
/// - `iter()` donne des `&f64` : `Score::try_from(s)` donne
///   `E0277 the trait bound Score: TryFrom<&f64> is not satisfied`. Déréférencez : `|&s|` ou `*s`.
/// - Pour enchaîner après `collect`, annotez : `collect::<Result<Vec<_>, _>>()` (sinon
///   `E0282 type annotations needed`). Un `Result<Vec<_>, _>` s'arrête à la première erreur.
/// - `v.sort()` modifie et rend `()` (`E0308 … found Result<(), NonFini>`) ; `pf_outils::trier_par` rend le `Vec`
///   trié. Donnez-lui le `Vec` lui-même, pas `v.iter()`, sinon vous obtenez des `&Score`.
pub fn classer(scores: &[f64]) -> Result<Vec<Score>, NonFini> {
    // But : parser tout, puis trier par ordre décroissant (`pf_outils::trier_par`).
    todo!()
}

/// Le meilleur score, `None` si la liste est vide.
///
/// **Aide Rust** (ch00, section 11)
/// - `iter().max()` rend une `Option<&Score>` : `E0308 … expected Option<Score>, found Option<&Score>`. `Score` est
///   `Copy` : `.copied()`.
pub fn meilleur(scores: &[Score]) -> Option<Score> {
    // But : sur des `f64`, `max()` ne compile pas ; sur un type `Ord`, c'est une ligne.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    fn s(x: f64) -> Score {
        Score::try_from(x).unwrap()
    }

    #[test]
    fn refuse_le_non_fini() {
        assert!(Score::try_from(f64::NAN).is_err());
        assert!(Score::try_from(f64::INFINITY).is_err());
        assert!(Score::try_from(f64::NEG_INFINITY).is_err());
        assert_eq!(Score::try_from(0.87).map(Score::valeur).ok(), Some(0.87));
    }

    #[test]
    fn ordre_total() {
        assert!(s(0.5) < s(0.9));
        assert!(s(-3.0) < s(-0.5));
        assert_eq!(s(-0.0), s(0.0));
        assert_eq!(s(-0.0).cmp(&s(0.0)), Ordering::Equal);
        assert_eq!(s(-0.0).valeur().to_bits(), 0.0_f64.to_bits());
    }

    #[test]
    fn classer_et_meilleur() {
        assert_eq!(
            classer(&[0.2, 0.9, 0.5]).ok(),
            Some(vec![s(0.9), s(0.5), s(0.2)])
        );
        assert_eq!(classer(&[]).ok(), Some(vec![]));
        assert!(classer(&[0.2, f64::NAN, 0.5]).is_err());
        assert_eq!(meilleur(&[s(0.2), s(0.9), s(0.5)]), Some(s(0.9)));
        assert_eq!(meilleur(&[]), None);
    }

    #[test]
    #[should_panic]
    fn le_naif_panique_sur_nan() {
        classer_naif(&[0.2, f64::NAN, 0.5]);
    }

    #[test]
    fn meme_classement_que_le_naif_sur_des_finis() {
        jeux::plusieurs(-1000, 1000).iter().for_each(|jeu| {
            let flottants: Vec<f64> = jeu.iter().map(|&n| n as f64 / 7.0).collect();
            let classes: Vec<f64> = classer(&flottants)
                .unwrap()
                .into_iter()
                .map(Score::valeur)
                .collect();
            assert_eq!(classes, classer_naif(&flottants), "jeu : {jeu:?}");
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e03_score.rs"),
            &["try_from", "cmp", "valeur", "classer", "meilleur"],
        );
    }
}
