//! # Exercice 1 : lancer deux threads et attendre leur résultat
//!
//! **À faire** : `somme_en_deux` coupe la série en deux moitiés, fait la somme de chaque moitié dans **son propre
//! thread**, puis additionne les deux résultats.
//!
//! **Ce qu'il faut savoir**
//! - Couper un `Vec` en deux : `let mut gauche = valeurs;` puis `let droite = gauche.split_off(gauche.len() / 2);`
//!   (`gauche` garde la première moitié).
//! - Lancer un thread qui rend une valeur : `let t = std::thread::spawn(move || gauche.iter().sum::<f64>());`
//! - `move` donne la moitié au thread : sans lui, le thread emprunterait une variable qui peut disparaître avant
//!   lui, et le compilateur refuse.
//! - Attendre le résultat : `t.join().unwrap()`.
//!
//! Test : `cargo test -p ch16-concurrence e01`

use std::thread;

/// La somme de la série, calculée en deux moitiés, chacune dans un thread.
pub fn somme_en_deux(valeurs: Vec<f64>) -> f64 {
    // `split_off` laisse deux `Vec` possédés : chacun peut être déplacé (`move`) dans son thread.
    // Les deux threads sont lancés avant le premier `join` : ils tournent en même temps. Joindre `t1` avant
    // de lancer `t2` rendrait le calcul séquentiel.
    let mut gauche = valeurs;
    let droite = gauche.split_off(gauche.len() / 2);
    let t1 = thread::spawn(move || gauche.iter().sum::<f64>());
    let t2 = thread::spawn(move || droite.iter().sum::<f64>());
    t1.join().unwrap() + t2.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meme_somme() {
        assert_eq!(somme_en_deux(vec![1.0, 2.0, 3.0, 4.0, 5.0]), 15.0);
        assert_eq!(somme_en_deux(vec![]), 0.0);
        let grande: Vec<f64> = (1..=1000).map(f64::from).collect();
        assert_eq!(somme_en_deux(grande), 500_500.0);
    }
}
