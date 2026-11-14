//! # Exercice 1 : la couche repository, le stockage derrière un trait
//!
//! Le service ne doit pas savoir **où** sont rangées les mesures (SQLite, Postgres, mémoire…). Il ne connaît que le
//! trait `MesuresRepository`. Ici on écrit l'implémentation en mémoire : une `BTreeMap` (id du capteur → ses
//! mesures) dans un `Arc<Mutex<…>>`, pour qu'Axum puisse la partager entre toutes les requêtes.
//!
//! **À faire** : les trois méthodes de `impl MesuresRepository for MesuresEnMemoire`.
//! 1. `capteur_existe` : `true` si l'id est une clé de la map.
//! 2. `ajouter` : ajoute les valeurs à la fin des mesures du capteur et rend `Some(nombre ajouté)`, ou `None` si le
//!    capteur n'existe pas.
//! 3. `valeurs_de` : une copie des mesures du capteur (vide s'il n'existe pas).
//!
//! **Ce qu'il faut savoir**
//! - `self.donnees()` (fourni) verrouille la map et la rend, comme une `BTreeMap` normale.
//! - `contains_key(&id)` ; `get(&id)` rend une `Option<&Vec<f64>>` ; `get_mut(&id)` une `Option<&mut Vec<f64>>`.
//! - Ajouter à un `Vec` : `mesures.extend(valeurs)`.
//! - Copier un `Option<&Vec<f64>>` en `Vec<f64>` : `.cloned().unwrap_or_default()`.
//!
//! Test : `cargo test -p ch18-couches e01`

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, MutexGuard};

/// Le contrat du stockage. `Clone + Send + Sync + 'static` : Axum le copie dans chaque requête, sur plusieurs threads.
pub trait MesuresRepository: Clone + Send + Sync + 'static {
    fn capteur_existe(&self, capteur_id: u32) -> bool;
    fn ajouter(&self, capteur_id: u32, valeurs: Vec<f64>) -> Option<usize>;
    fn valeurs_de(&self, capteur_id: u32) -> Vec<f64>;
}

/// Le stockage en mémoire. `Clone` copie l'`Arc`, pas les données : tous les clones voient la même map.
#[derive(Clone, Default)]
pub struct MesuresEnMemoire {
    donnees: Arc<Mutex<BTreeMap<u32, Vec<f64>>>>,
}

impl MesuresEnMemoire {
    /// Fourni : un stockage où ces capteurs existent, sans mesure.
    pub fn avec_capteurs(ids: &[u32]) -> Self {
        let donnees = ids.iter().map(|&id| (id, Vec::new())).collect();
        Self {
            donnees: Arc::new(Mutex::new(donnees)),
        }
    }

    /// Fourni : la map, verrouillée le temps de s'en servir.
    fn donnees(&self) -> MutexGuard<'_, BTreeMap<u32, Vec<f64>>> {
        self.donnees.lock().unwrap()
    }
}

impl MesuresRepository for MesuresEnMemoire {
    fn capteur_existe(&self, capteur_id: u32) -> bool {
        self.donnees().contains_key(&capteur_id)
    }

    fn ajouter(&self, capteur_id: u32, valeurs: Vec<f64>) -> Option<usize> {
        let nombre = valeurs.len();
        self.donnees().get_mut(&capteur_id).map(|mesures| {
            mesures.extend(valeurs);
            nombre
        })
    }

    fn valeurs_de(&self, capteur_id: u32) -> Vec<f64> {
        self.donnees().get(&capteur_id).cloned().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ajouter_puis_relire() {
        let repo = MesuresEnMemoire::avec_capteurs(&[1]);
        assert!(repo.capteur_existe(1));
        assert_eq!(repo.ajouter(1, vec![21.5, 22.0]), Some(2));
        assert_eq!(repo.ajouter(1, vec![19.8]), Some(1));
        assert_eq!(repo.valeurs_de(1), [21.5, 22.0, 19.8]);
    }

    #[test]
    fn capteur_inconnu() {
        let repo = MesuresEnMemoire::avec_capteurs(&[1]);
        assert!(!repo.capteur_existe(7));
        assert_eq!(repo.ajouter(7, vec![1.0]), None);
        assert_eq!(repo.valeurs_de(7), Vec::<f64>::new());
    }

    #[test]
    fn les_clones_partagent_les_donnees() {
        let repo = MesuresEnMemoire::avec_capteurs(&[1]);
        repo.clone().ajouter(1, vec![5.0]);
        assert_eq!(repo.valeurs_de(1), [5.0]);
    }
}
