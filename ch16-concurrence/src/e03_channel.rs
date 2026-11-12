//! # Exercice 3 : un channel, un producteur, un consommateur
//!
//! Au lieu de partager une variable, un thread **envoie** des messages à un autre.
//!
//! **À faire** : `convertir` lance un thread qui envoie, une par une, les mesures converties en Fahrenheit
//! (`c * 9.0 / 5.0 + 32.0`). Le thread principal les reçoit et les rend dans un `Vec`, dans l'ordre d'envoi.
//!
//! **Ce qu'il faut savoir**
//! - Créer : `let (tx, rx) = std::sync::mpsc::channel();`
//! - Le thread producteur reçoit `tx` par `move` et fait `tx.send(valeur).unwrap()` pour chaque mesure.
//! - Le consommateur : `rx.iter().collect()`. Cette boucle s'arrête quand plus aucun `tx` n'existe, c'est-à-dire
//!   quand le thread producteur a fini (son `tx` est détruit avec lui).
//!
//! Test : `cargo test -p ch16-concurrence e03`

use std::sync::mpsc;
use std::thread;

/// Les mesures converties en Fahrenheit, envoyées par un thread à travers un channel.
pub fn convertir(celsius: Vec<f64>) -> Vec<f64> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dans_l_ordre() {
        assert_eq!(convertir(vec![0.0, 100.0, -40.0]), [32.0, 212.0, -40.0]);
        assert_eq!(convertir(vec![]), Vec::<f64>::new());
    }
}
