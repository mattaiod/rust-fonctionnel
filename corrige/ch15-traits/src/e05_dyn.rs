//! # Exercice 5 : `dyn Trait`, mélanger des types dans une liste
//!
//! Un `Vec<T>` ne contient qu'un seul type. Pour ranger `Moyenne`, `Min` et `Max` **dans la même liste**, on les
//! met dans des boîtes : `Vec<Box<dyn Statistique>>`. Le bon `calculer` est choisi à l'exécution.
//!
//! **À faire** :
//! 1. `toutes` : rend la liste des trois statistiques, dans l'ordre `Moyenne`, `Min`, `Max`.
//! 2. `par_nom` : rend la statistique dont le nom vaut `nom`, `None` si aucune.
//! 3. `resumer` : pour chaque statistique de `toutes()`, la paire `(nom, valeur)`.
//!
//! **Ce qu'il faut savoir**
//! - `vec![Box::new(Moyenne), Box::new(Min), Box::new(Max)]` (le type de retour indique au compilateur qu'il faut
//!   des `Box<dyn Statistique>`).
//! - `toutes().into_iter().find(|s| s.nom() == nom)`.
//! - `toutes().iter().map(|s| (s.nom(), s.calculer(valeurs))).collect()`.
//!
//! Test : `cargo test -p ch15-traits e05`

pub trait Statistique {
    fn nom(&self) -> &'static str;
    fn calculer(&self, valeurs: &[f64]) -> Option<f64>;
}

pub struct Moyenne;
pub struct Min;
pub struct Max;

impl Statistique for Moyenne {
    fn nom(&self) -> &'static str {
        "moyenne"
    }
    fn calculer(&self, valeurs: &[f64]) -> Option<f64> {
        (!valeurs.is_empty()).then(|| valeurs.iter().sum::<f64>() / valeurs.len() as f64)
    }
}

impl Statistique for Min {
    fn nom(&self) -> &'static str {
        "min"
    }
    fn calculer(&self, valeurs: &[f64]) -> Option<f64> {
        valeurs.iter().copied().reduce(f64::min)
    }
}

impl Statistique for Max {
    fn nom(&self) -> &'static str {
        "max"
    }
    fn calculer(&self, valeurs: &[f64]) -> Option<f64> {
        valeurs.iter().copied().reduce(f64::max)
    }
}

/// Les trois statistiques : `Moyenne`, `Min`, `Max`.
pub fn toutes() -> Vec<Box<dyn Statistique>> {
    // Le type de retour annoncé permet au compilateur de convertir chaque `Box<Moyenne>`, `Box<Min>`, `Box<Max>`
    // en `Box<dyn Statistique>`.
    vec![Box::new(Moyenne), Box::new(Min), Box::new(Max)]
}

/// La statistique qui porte ce nom.
pub fn par_nom(nom: &str) -> Option<Box<dyn Statistique>> {
    // `into_iter()` pour rendre la `Box` elle-même : `iter()` ne donnerait qu'une référence vers un `Vec` local
    // détruit à la fin de la fonction.
    toutes().into_iter().find(|s| s.nom() == nom)
}

/// `[("moyenne", …), ("min", …), ("max", …)]`.
pub fn resumer(valeurs: &[f64]) -> Vec<(&'static str, Option<f64>)> {
    // Chaque `s.calculer` passe par la vtable : la méthode est choisie à l'exécution.
    // `nom()` rend un `&'static str`, le résultat ne dépend donc pas du `Vec` temporaire de `toutes()`.
    toutes()
        .iter()
        .map(|s| (s.nom(), s.calculer(valeurs)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trois_dans_l_ordre() {
        let noms: Vec<_> = toutes().iter().map(|s| s.nom()).collect();
        assert_eq!(noms, ["moyenne", "min", "max"]);
    }

    #[test]
    fn chercher_par_nom() {
        assert_eq!(
            par_nom("max").map(|s| s.calculer(&[1.0, 5.0])),
            Some(Some(5.0))
        );
        assert!(par_nom("mediane").is_none());
    }

    #[test]
    fn resume_complet() {
        assert_eq!(
            resumer(&[1.0, 3.0]),
            [
                ("moyenne", Some(2.0)),
                ("min", Some(1.0)),
                ("max", Some(3.0))
            ]
        );
        assert_eq!(
            resumer(&[]),
            [("moyenne", None), ("min", None), ("max", None)]
        );
    }
}
