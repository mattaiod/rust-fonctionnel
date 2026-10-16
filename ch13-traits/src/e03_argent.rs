//! Exercice 3 : de l'argent qui s'additionne : `Add`, `Mul`, `Sum`, `Display`.
//!
//! Des centimes en `i64` nu, c'est le bug assuré (« 12 » : euros ou centimes ?). Le newtype `Argent` stocke des
//! centimes et n'offre que les opérations qui ont un sens : additionner deux montants (`Add`), multiplier un
//! prix par une quantité (`Mul<u32>`, pas `Argent * Argent`, qui ne veut rien dire). `Sum` permet `.sum()` sur
//! un itérateur d'`Argent` ; il s'écrit avec un `fold` et l'`Add` qu'on vient d'écrire.
//! Piège de l'affichage : -305 centimes s'écrit `-3,05 €`, pas `-3,-5 €`.

use std::fmt;
use std::iter::Sum;
use std::ops::{Add, Mul};

/// Version naïve : des `i64` sans unité et une boucle. Ne pas modifier.
pub fn total_naif(lignes: &[(i64, u32)]) -> i64 {
    let mut total = 0;
    for (prix_centimes, quantite) in lignes {
        total += prix_centimes * *quantite as i64;
    }
    total
}

/// Version naïve. Ne pas modifier.
pub fn afficher_naif(centimes: i64) -> String {
    let signe = if centimes < 0 { "-" } else { "" };
    let absolu = centimes.abs();
    format!("{signe}{},{:02} €", absolu / 100, absolu % 100)
}

/// Un montant en centimes d'euro (négatif pour un avoir).
///
/// Ce qui compile :
///
/// ```no_run
/// use ch13_traits::e03_argent::Argent;
/// let total = Argent(1250) + Argent(199) * 3;
/// ```
///
/// Multiplier deux montants ne compile pas : seul `Mul<u32>` est implémenté.
///
/// ```compile_fail,E0308
/// use ch13_traits::e03_argent::Argent;
/// let absurde = Argent(1250) * Argent(199);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Argent(pub i64);

impl Add for Argent {
    type Output = Argent;

    /// **Aide Rust** (ch00, section 1)
    /// - `self.0 + autre.0` est un `i64` : `E0308 … expected Argent, found i64`. Emballez le résultat.
    /// - `self + autre` rappelle ce même `add` sans fin : `warning: function cannot return without recursing`.
    ///   Additionnez les champs.
    fn add(self, autre: Argent) -> Argent {
        todo!()
    }
}

impl Mul<u32> for Argent {
    type Output = Argent;

    /// Prix unitaire × quantité.
    ///
    /// **Aide Rust** (ch00, section 1)
    /// - `self.0 * quantite` donne `E0308 … expected i64, found u32` puis `E0277 cannot multiply i64 by u32` : Rust
    ///   ne mélange pas les types d'entiers.
    ///   `i64::from(quantite)` convertit sans perte.
    fn mul(self, quantite: u32) -> Argent {
        todo!()
    }
}

impl Sum for Argent {
    /// **Aide Rust** (ch00, sections 7, 9)
    /// - Le point de départ du `fold` est un `Argent`, pas `0` : `E0308 … expected Argent, found integer`. `Argent`
    ///   dérive `Default`.
    /// - Piège qui compile : `montants.sum()` dans `sum` se rappelle sans fin (pas d'avertissement cette fois,
    ///   `stack overflow` aux tests).
    fn sum<I: Iterator<Item = Argent>>(montants: I) -> Argent {
        // But : un `fold` qui part de zéro et réutilise `Add`.
        todo!()
    }
}

impl fmt::Display for Argent {
    /// `12,50 €`, `0,07 €`, `-3,05 €`.
    ///
    /// **Aide Rust** (ch00, section 1)
    /// - `-305 / 100` vaut `-3`, mais `-305 % 100` vaut `-5` : on obtiendrait `-3,-5 €`. Séparez le signe, puis
    ///   travaillez sur la valeur absolue (`abs()` ou `unsigned_abs()`).
    /// - `{:02}` pour les centimes : `7` doit s'écrire `07`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// Le total d'un panier de lignes `(prix unitaire, quantité)`.
///
/// **Aide Rust** (ch00, sections 3, 4)
/// - `iter()` donne des `&(Argent, u32)` : `|(prix, quantite)| prix * quantite` donne
///   `E0369 cannot multiply &Argent by &u32`. Les deux sont `Copy` : déstructurez avec `|&(prix, quantite)|`.
/// - `lignes.iter().sum()` direct donne
///   `E0277 a value of type Argent cannot be made by summing an iterator over elements of type &(Argent, u32)` :
///   passez par un `map` avant.
pub fn total_panier(lignes: &[(Argent, u32)]) -> Argent {
    // But : une ligne, grâce à `Mul` et `Sum`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    #[test]
    fn operateurs() {
        assert_eq!(Argent(1250) + Argent(-300), Argent(950));
        assert_eq!(Argent(199) * 3, Argent(597));
        assert_eq!(
            [Argent(1), Argent(2), Argent(3)]
                .into_iter()
                .sum::<Argent>(),
            Argent(6)
        );
        assert_eq!(Vec::<Argent>::new().into_iter().sum::<Argent>(), Argent(0));
    }

    #[test]
    fn affichage() {
        assert_eq!(Argent(1250).to_string(), "12,50 €");
        assert_eq!(Argent(7).to_string(), "0,07 €");
        assert_eq!(Argent(-305).to_string(), "-3,05 €");
        assert_eq!(Argent(0).to_string(), "0,00 €");
    }

    #[test]
    fn meme_resultat_que_le_naif() {
        jeux::plusieurs(-5_000, 5_000).iter().for_each(|jeu| {
            let lignes: Vec<(i64, u32)> = jeu
                .iter()
                .map(|&p| (p, (p.unsigned_abs() % 7) as u32))
                .collect();
            let typees: Vec<(Argent, u32)> = lignes.iter().map(|&(p, q)| (Argent(p), q)).collect();
            let total = total_panier(&typees);
            assert_eq!(total.0, total_naif(&lignes));
            assert_eq!(total.to_string(), afficher_naif(total_naif(&lignes)));
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e03_argent.rs"),
            &["add", "mul", "sum", "fmt", "total_panier"],
        );
    }
}
