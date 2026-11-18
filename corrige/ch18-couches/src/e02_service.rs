//! # Exercice 2 : la couche service, générique sur le repository
//!
//! Le service contient les règles de l'application. Il ne connaît ni HTTP ni le stockage réel : il reçoit
//! **n'importe quel** `R: MesuresRepository`. En production, ce serait une base de données ; dans les tests, la
//! mémoire de l'exercice 1. Le même code marche pour les deux.
//!
//! **À faire** : `stats_capteur` rend `None` si le capteur n'existe pas, sinon un `StatsCapteur` avec :
//! `capteur_id`, `nb_mesures` (le nombre de valeurs), `moyenne`, `min`, `max` (`None` s'il n'y a aucune mesure).
//!
//! **Ce qu'il faut savoir**
//! - `repo.capteur_existe(id)` puis `let valeurs = repo.valeurs_de(id);`
//! - Les trois calculs sont fournis : `moyenne(&valeurs)`, `min(&valeurs)`, `max(&valeurs)`.
//! - Rendre `None` tôt : `if !repo.capteur_existe(capteur_id) { return None; }`
//!
//! Test : `cargo test -p ch18-couches e02` (le service est testé sans serveur ni base).

use serde::Serialize;

use crate::e01_repository::MesuresRepository;

/// Les statistiques d'un capteur. `Serialize` : le contrôleur pourra le rendre en JSON.
#[derive(Debug, PartialEq, Serialize)]
pub struct StatsCapteur {
    pub capteur_id: u32,
    pub nb_mesures: usize,
    pub moyenne: Option<f64>,
    pub min: Option<f64>,
    pub max: Option<f64>,
}

/// Fourni : ajouter des mesures. `None` si le capteur n'existe pas.
pub fn ajouter<R: MesuresRepository>(
    repo: &R,
    capteur_id: u32,
    valeurs: Vec<f64>,
) -> Option<usize> {
    repo.ajouter(capteur_id, valeurs)
}

/// Les statistiques du capteur, `None` s'il n'existe pas.
pub fn stats_capteur<R: MesuresRepository>(repo: &R, capteur_id: u32) -> Option<StatsCapteur> {
    // Le service dépend du trait, pas de `MesuresEnMemoire` : les tests le font tourner sans base ni serveur.
    // `valeurs_de` n'est appelé qu'une fois : les trois calculs travaillent sur la même copie.
    if !repo.capteur_existe(capteur_id) {
        return None;
    }
    let valeurs = repo.valeurs_de(capteur_id);
    Some(StatsCapteur {
        capteur_id,
        nb_mesures: valeurs.len(),
        moyenne: moyenne(&valeurs),
        min: min(&valeurs),
        max: max(&valeurs),
    })
}

/// Fourni.
pub fn moyenne(valeurs: &[f64]) -> Option<f64> {
    (!valeurs.is_empty()).then(|| valeurs.iter().sum::<f64>() / valeurs.len() as f64)
}

/// Fourni.
pub fn min(valeurs: &[f64]) -> Option<f64> {
    valeurs.iter().copied().reduce(f64::min)
}

/// Fourni.
pub fn max(valeurs: &[f64]) -> Option<f64> {
    valeurs.iter().copied().reduce(f64::max)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::e01_repository::MesuresEnMemoire;

    #[test]
    fn stats_d_un_capteur() {
        let repo = MesuresEnMemoire::avec_capteurs(&[1]);
        ajouter(&repo, 1, vec![12.0, 20.0, 10.0]);
        let attendu = StatsCapteur {
            capteur_id: 1,
            nb_mesures: 3,
            moyenne: Some(14.0),
            min: Some(10.0),
            max: Some(20.0),
        };
        assert_eq!(stats_capteur(&repo, 1), Some(attendu));
    }

    #[test]
    fn capteur_sans_mesure() {
        let repo = MesuresEnMemoire::avec_capteurs(&[1]);
        let attendu = StatsCapteur {
            capteur_id: 1,
            nb_mesures: 0,
            moyenne: None,
            min: None,
            max: None,
        };
        assert_eq!(stats_capteur(&repo, 1), Some(attendu));
    }

    #[test]
    fn capteur_inconnu() {
        let repo = MesuresEnMemoire::avec_capteurs(&[1]);
        assert_eq!(stats_capteur(&repo, 9), None);
    }
}
