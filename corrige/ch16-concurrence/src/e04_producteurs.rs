//! # Exercice 4 : plusieurs producteurs, un consommateur
//!
//! Chaque capteur a son thread, et tous envoient dans **le même** channel des messages `(id du capteur, valeur)`.
//!
//! **À faire** : `collecter` lance un thread par capteur, chacun envoie toutes ses mesures, et la fonction rend
//! tous les messages reçus, **triés** (les threads arrivent dans n'importe quel ordre).
//!
//! **Ce qu'il faut savoir**
//! - Un `tx` par producteur : `let tx = tx.clone();` juste avant chaque `thread::spawn(move || ...)`.
//! - **Le piège** : le `tx` d'origine existe encore après la boucle, donc `rx.iter()` attendrait pour toujours.
//!   Faites `drop(tx);` avant de lire.
//! - Trier des `(u32, f64)` : `messages.sort_by(|a, b| a.partial_cmp(b).unwrap());`
//!
//! Test : `cargo test -p ch16-concurrence e04`

use std::sync::mpsc;
use std::thread;

/// Tous les `(id, valeur)` envoyés par les capteurs, triés.
pub fn collecter(capteurs: Vec<(u32, Vec<f64>)>) -> Vec<(u32, f64)> {
    // Un clone de `tx` par thread, puis `drop(tx)` sur l'original : sinon `rx.iter()` attendrait pour toujours.
    // L'ordre d'arrivée dépend de l'ordonnanceur : le tri rend un résultat reproductible. `partial_cmp(...).unwrap()`
    // panique sur un `NaN` ; sans panique : `a.0.cmp(&b.0).then(a.1.total_cmp(&b.1))`.
    let (tx, rx) = mpsc::channel();
    capteurs.into_iter().for_each(|(id, valeurs)| {
        let tx = tx.clone();
        thread::spawn(move || valeurs.into_iter().for_each(|v| tx.send((id, v)).unwrap()));
    });
    drop(tx);
    let mut messages: Vec<(u32, f64)> = rx.iter().collect();
    messages.sort_by(|a, b| a.partial_cmp(b).unwrap());
    messages
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tous_les_messages() {
        let recus = collecter(vec![(2, vec![18.0, 18.5]), (1, vec![21.5]), (3, vec![])]);
        assert_eq!(recus, [(1, 21.5), (2, 18.0), (2, 18.5)]);
    }

    #[test]
    fn aucun_capteur() {
        assert_eq!(collecter(vec![]), []);
    }
}
