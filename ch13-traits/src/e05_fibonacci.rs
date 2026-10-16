//! Exercice 5 : `Iterator` pour son propre type : Fibonacci.
//!
//! On écrit `type Item` et `next`, et on reçoit tout le reste : `take`, `take_while`, `filter`, `sum`,
//! `collect`… La boucle de la version naïve disparaît dans l'appelant. `next` prend `&mut self` : c'est le seul
//! endroit où l'état avance (la règle « pas de `let mut` » vise vos variables locales, pas les champs de
//! l'itérateur, dont c'est le rôle). Piège : F(94) dépasse `u64::MAX`. Avec `checked_add`, la suite s'arrête
//! proprement (`None`) au lieu de paniquer en debug ou de rendre n'importe quoi en release.

/// Version naïve : les `n` premiers termes (au plus 94), par une boucle. Ne pas modifier.
pub fn fibonacci_naif(n: usize) -> Vec<u64> {
    let mut termes = Vec::new();
    let (mut a, mut b) = (0_u64, 1_u64);
    for _ in 0..n.min(94) {
        termes.push(a);
        let suivant = a.wrapping_add(b);
        a = b;
        b = suivant;
    }
    termes
}

/// Version naïve : somme des termes pairs inférieurs ou égaux à `max`. Ne pas modifier.
pub fn somme_pairs_naif(max: u64) -> u64 {
    let (mut a, mut b) = (0_u64, 1_u64);
    let mut somme = 0;
    while a <= max {
        if a % 2 == 0 {
            somme += a;
        }
        match a.checked_add(b) {
            Some(suivant) => {
                a = b;
                b = suivant;
            }
            None => break,
        }
    }
    somme
}

/// La suite de Fibonacci 0, 1, 1, 2, 3, 5… jusqu'au dernier terme qui tient dans un `u64` (F(93)).
#[derive(Debug, Clone)]
pub struct Fibonacci {
    courant: Option<u64>,
    suivant: Option<u64>,
}

impl Default for Fibonacci {
    /// Le début de la suite : 0 puis 1.
    ///
    /// **Aide Rust** (ch00, section 11)
    /// - Les champs sont des `Option<u64>` : `courant: 0` donne `E0308 … expected Option<u64>, found integer`.
    fn default() -> Self {
        todo!()
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    /// **Aide Rust** (ch00, sections 7, 11)
    /// - On n'additionne pas deux `Option` (`E0369 cannot add Option<u64> to Option<u64>`), et `checked_add` est une
    ///   méthode de `u64`, pas d'`Option` (`E0599 no method named checked_add found for enum Option<T>`). Sortez
    ///   d'abord le `u64` : `let rendu = self.courant?;` termine l'itérateur si `courant` vaut `None`. Puis
    ///   `and_then` pour l'addition qui peut déborder.
    /// - Ici on modifie les champs (`self.courant = ...`), c'est le rôle de `next`. Mais pas de `let mut` : le test
    ///   de style le refuse.
    fn next(&mut self) -> Option<u64> {
        // But : rendre le terme courant, avancer d'un cran, s'arrêter au lieu de déborder.
        todo!()
    }
}

/// Les `n` premiers termes (au plus 94).
///
/// **Aide Rust** (ch00, sections 3, 12)
/// - `take(n)` est paresseux : `E0308 … expected Vec<u64>, found Take<Fibonacci>`. Collectez.
/// - Le point de départ est `Fibonacci::default()` : il n'y a pas de `new`
///   (`E0599 no function or associated item named new`), et `Fibonacci` seul n'est pas une valeur
///   (`E0423 expected value, found struct Fibonacci`).
pub fn premiers(n: usize) -> Vec<u64> {
    todo!()
}

/// Somme des termes pairs inférieurs ou égaux à `max`.
///
/// **Aide Rust** (ch00, section 4)
/// - `take_while(|t| t <= max)` donne `E0308 … expected &u64, found u64` : la closure reçoit un `&u64`. `|&t|` le
///   déstructure.
/// - `take_while` avant `filter` : il faut s'arrêter au premier terme trop grand, pas filtrer toute la suite.
pub fn somme_pairs(max: u64) -> u64 {
    // But : une chaîne d'adaptateurs sur `Fibonacci`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::style;

    #[test]
    fn cas_connus() {
        assert_eq!(premiers(10), vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
        assert_eq!(premiers(0), Vec::<u64>::new());
        assert_eq!(somme_pairs(100), 44);
        assert_eq!(somme_pairs(4_000_000), 4_613_732);
    }

    #[test]
    fn s_arrete_avant_de_deborder() {
        assert_eq!(Fibonacci::default().count(), 94);
        assert_eq!(
            Fibonacci::default().last(),
            Some(12_200_160_415_121_876_738)
        );
        assert_eq!(premiers(1000).len(), 94);
    }

    #[test]
    fn meme_resultat_que_le_naif() {
        (0..=100).for_each(|n| assert_eq!(premiers(n), fibonacci_naif(n), "n = {n}"));
        [0, 1, 2, 10, 1_000, 123_456_789, 1 << 40, 1 << 62]
            .iter()
            .for_each(|&max| assert_eq!(somme_pairs(max), somme_pairs_naif(max), "max = {max}"));
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e05_fibonacci.rs"),
            &["default", "next", "premiers", "somme_pairs"],
        );
    }
}
