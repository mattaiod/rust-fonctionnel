//! # Exercice 2 : partager un compteur entre threads, `Arc<Mutex<T>>`
//!
//! Plusieurs capteurs envoient chacun un lot de mesures. Un thread par lot, et tous ajoutent leur nombre de mesures
//! au **même** compteur.
//!
//! **À faire** : `compter_mesures` rend le nombre total de mesures, en lançant un thread par lot.
//!
//! **Ce qu'il faut savoir**
//! - `Mutex` : un seul thread à la fois modifie la valeur. `Arc` : plusieurs threads possèdent le même `Mutex`.
//! - Créer : `let total = Arc::new(Mutex::new(0));`
//! - Pour chaque lot : `let total = Arc::clone(&total);` puis `thread::spawn(move || { ... })`.
//! - Dans le thread : `*total.lock().unwrap() += lot.len();`
//! - Garder les `JoinHandle` dans un `Vec`, faire `join()` sur chacun, **puis** lire `*total.lock().unwrap()`.
//!
//! Test : `cargo test -p ch16-concurrence e02`

use std::sync::{Arc, Mutex};
use std::thread;

/// Le nombre total de mesures, un thread par lot.
pub fn compter_mesures(lots: Vec<Vec<f64>>) -> usize {
    let total = Arc::new(Mutex::new(0));
    let threads: Vec<_> = lots
        .into_iter()
        .map(|lot| {
            let total = Arc::clone(&total);
            thread::spawn(move || *total.lock().unwrap() += lot.len())
        })
        .collect();
    threads.into_iter().for_each(|t| t.join().unwrap());
    *total.lock().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total() {
        assert_eq!(compter_mesures(vec![vec![1.0, 2.0], vec![3.0], vec![]]), 3);
        assert_eq!(compter_mesures(vec![]), 0);
        let beaucoup = (0..50).map(|_| vec![0.0; 100]).collect();
        assert_eq!(compter_mesures(beaucoup), 5000);
    }
}
