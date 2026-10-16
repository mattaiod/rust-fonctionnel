//! Exercice 4 : un trait métier à type associé : `Metrique`.
//!
//! Le cas d'Arena (C3, trait `Metric`). L'exactitude compare des classes (`u32`), l'erreur absolue moyenne des
//! réels (`f64`) : chaque métrique fixe le type de ses valeurs par `type Valeur`, et le compilateur refuse de
//! passer des `f64` à l'exactitude. Une seule fonction générique `rapport<M: Metrique>` remplace les deux
//! copies de la version naïve. `Option<f64>` remplace le code `-1.0` : « incalculable » n'est pas un nombre.

/// Version naïve : -1.0 si incalculable (longueurs différentes ou listes vides). Ne pas modifier.
pub fn exactitude_naif(verite: &[u32], predictions: &[u32]) -> f64 {
    if verite.len() != predictions.len() || verite.is_empty() {
        return -1.0;
    }
    let mut justes = 0;
    for i in 0..verite.len() {
        if verite[i] == predictions[i] {
            justes += 1;
        }
    }
    justes as f64 / verite.len() as f64
}

/// Version naïve, même convention. Ne pas modifier.
pub fn mae_naif(verite: &[f64], predictions: &[f64]) -> f64 {
    if verite.len() != predictions.len() || verite.is_empty() {
        return -1.0;
    }
    let mut total = 0.0;
    for i in 0..verite.len() {
        total += (verite[i] - predictions[i]).abs();
    }
    total / verite.len() as f64
}

/// Version naïve : un rapport par métrique, copié-collé. Ne pas modifier.
pub fn rapport_exactitude_naif(verite: &[u32], predictions: &[u32]) -> String {
    let valeur = exactitude_naif(verite, predictions);
    if valeur < 0.0 {
        "exactitude : incalculable".to_string()
    } else {
        format!("exactitude : {valeur:.3}")
    }
}

/// Version naïve, la copie. Ne pas modifier.
pub fn rapport_mae_naif(verite: &[f64], predictions: &[f64]) -> String {
    let valeur = mae_naif(verite, predictions);
    if valeur < 0.0 {
        "mae : incalculable".to_string()
    } else {
        format!("mae : {valeur:.3}")
    }
}

/// Une métrique d'évaluation : elle choisit le type de ses valeurs.
pub trait Metrique {
    /// Le type d'une vérité ou d'une prédiction.
    type Valeur;

    /// Le nom affiché dans les rapports.
    fn nom(&self) -> &'static str;

    /// La valeur de la métrique, `None` si les longueurs diffèrent ou si les listes sont vides.
    fn calculer(&self, verite: &[Self::Valeur], predictions: &[Self::Valeur]) -> Option<f64>;
}

/// Part des prédictions exactes.
#[derive(Debug, Clone, Copy, Default)]
pub struct Exactitude;

/// Erreur absolue moyenne.
#[derive(Debug, Clone, Copy, Default)]
pub struct Mae;

/// Part des positions où `predictions` égale `verite`.
///
/// **Aide Rust** (ch00, sections 1, 4)
/// - Un `if` sans `else` doit valoir `()` : `if ... { None }` donne
///   `E0308 … if expressions without else arms expect their inner expression to be ()`. Écrivez `return None;`, ou un
///   `if/else` complet, ou `bool::then`.
/// - `count() as f64 / verite.len()` donne `E0277 cannot divide f64 by usize` : convertissez les deux côtés. Et
///   `(count / len) as f64` divise des entiers d'abord : 0 ou 1.
pub fn exactitude(verite: &[u32], predictions: &[u32]) -> Option<f64> {
    todo!()
}

/// Moyenne des `|vérité - prédiction|`.
///
/// **Aide Rust** (ch00, section 12)
/// - `sum()` suivi d'une division ne sait pas quel type produire (`E0283 type annotations needed`) : `sum::<f64>()`.
pub fn mae(verite: &[f64], predictions: &[f64]) -> Option<f64> {
    todo!()
}

impl Metrique for Exactitude {
    type Valeur = u32;

    /// **Aide Rust** (ch00, sections 5, 13)
    /// - Un littéral `"..."` est déjà un `&'static str`. `.to_string()` donne `E0308 … expected &str, found String`.
    fn nom(&self) -> &'static str {
        // But : `exactitude`.
        todo!()
    }

    /// **Aide Rust** (ch00, section 1)
    /// - Appelez la fonction libre écrite plus haut. Si vous écrivez `calculer(...)`, le compilateur dit
    ///   `E0425 cannot find function calculer` et propose `self.calculer(...)` : c'est un piège, cela rappelle cette
    ///   méthode à l'infini (`warning: function cannot return without recursing`).
    fn calculer(&self, verite: &[u32], predictions: &[u32]) -> Option<f64> {
        todo!()
    }
}

impl Metrique for Mae {
    type Valeur = f64;

    fn nom(&self) -> &'static str {
        // But : `mae`.
        todo!()
    }

    fn calculer(&self, verite: &[f64], predictions: &[f64]) -> Option<f64> {
        todo!()
    }
}

/// `nom : valeur` à 3 décimales, ou `nom : incalculable`. Marche pour toute métrique.
///
/// Ce qui compile :
///
/// ```no_run
/// use ch13_traits::e04_metrique::{Exactitude, Mae, rapport};
/// let a = rapport(&Exactitude, &[1, 0, 1], &[1, 1, 1]);
/// let b = rapport(&Mae, &[1.5, 2.0], &[1.0, 2.0]);
/// ```
///
/// Passer des réels à l'exactitude ne compile pas : avec `M = Exactitude`, `M::Valeur` vaut `u32`.
///
/// ```compile_fail,E0308
/// use ch13_traits::e04_metrique::{Exactitude, rapport};
/// let a = rapport(&Exactitude, &[1.5, 2.0], &[1.0, 2.0]);
/// ```
///
/// **Aide Rust** (ch00, sections 11, 12)
/// - `calculer` est une méthode : `metrique.calculer(...)`. `M::calculer(verite, predictions)` donne
///   `E0061 this function takes 3 arguments but 2 arguments were supplied` (il manque `&self`).
/// - Pas d'appel de méthode dans une chaîne de format : `{metrique.nom()}` donne
///   `invalid format string: expected }, found .`. Passez-le en argument : `format!("{} : ...", metrique.nom())`.
///   `{valeur:.3}` donne 3 décimales.
pub fn rapport<M: Metrique>(
    metrique: &M,
    verite: &[M::Valeur],
    predictions: &[M::Valeur],
) -> String {
    // But : une seule fonction pour toutes les métriques.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    #[test]
    fn cas_connus() {
        assert_eq!(Exactitude.calculer(&[1, 0, 1, 1], &[1, 1, 1, 0]), Some(0.5));
        assert_eq!(Exactitude.calculer(&[1], &[1, 0]), None);
        assert_eq!(Mae.calculer(&[1.0, 2.0], &[2.0, 4.0]), Some(1.5));
        assert_eq!(Mae.calculer(&[], &[]), None);
        assert_eq!(
            rapport(&Exactitude, &[1, 2, 3], &[1, 2, 0]),
            "exactitude : 0.667"
        );
        assert_eq!(rapport(&Mae, &[1.0], &[]), "mae : incalculable");
    }

    #[test]
    fn meme_resultat_que_le_naif() {
        (0..20_u64).for_each(|graine| {
            let taille = (graine as usize * 3) % 12;
            let v = jeux::entiers(graine, taille, 0, 3);
            let p = jeux::entiers(
                graine + 50,
                taille + usize::from(graine.is_multiple_of(4)),
                0,
                3,
            );
            let (vc, pc): (Vec<u32>, Vec<u32>) = (
                v.iter().map(|&x| x as u32).collect(),
                p.iter().map(|&x| x as u32).collect(),
            );
            let (vr, pr): (Vec<f64>, Vec<f64>) = (
                v.iter().map(|&x| x as f64 * 1.5).collect(),
                p.iter().map(|&x| x as f64).collect(),
            );
            assert_eq!(
                rapport(&Exactitude, &vc, &pc),
                rapport_exactitude_naif(&vc, &pc)
            );
            assert_eq!(rapport(&Mae, &vr, &pr), rapport_mae_naif(&vr, &pr));
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e04_metrique.rs"),
            &["exactitude", "mae", "rapport"],
        );
    }
}
