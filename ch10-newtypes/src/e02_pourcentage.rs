//! Exercice 2 : `Pourcentage` : un entier entre 0 et 100.
//!
//! Avec un `u8` nu, une remise de 150 % passe à la compilation et fait déborder la soustraction à l'exécution
//! (`attempt to subtract with overflow` en debug, un prix absurde en release). Le newtype vérifie la borne **une
//! fois**, dans `try_from`. Ensuite, chaque fonction qui reçoit un `Pourcentage` peut s'y fier sans revérifier :
//! `complement` (100 - p) n'a même pas besoin de rendre un `Result`.
//!
//! ```compile_fail,E0603
//! use ch10_newtypes::e02_pourcentage::Pourcentage;
//! let p = Pourcentage(150); // E0603 : champ privé, seul `try_from` construit un Pourcentage
//! ```

/// Version naïve : la remise en `u8`, sans contrôle. Ne pas modifier.
pub fn appliquer_remise_naif(prix_centimes: u64, remise: u8) -> u64 {
    prix_centimes - prix_centimes * remise as u64 / 100
}

/// Un pourcentage entier, de 0 à 100 inclus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pourcentage(u8);

/// La valeur refusée.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HorsBornes(pub i64);

impl TryFrom<i64> for Pourcentage {
    type Error = HorsBornes;

    /// **Aide Rust** (ch00, section 1)
    /// - Le champ est un `u8`, la valeur un `i64` : `Pourcentage(valeur)` donne `E0308 … expected u8, found i64`. Une
    ///   fois la borne vérifiée, la conversion `valeur as u8` ne perd rien.
    fn try_from(valeur: i64) -> Result<Self, Self::Error> {
        // But : accepter 0..=100, refuser le reste en rendant la valeur fautive.
        todo!()
    }
}

impl Pourcentage {
    /// La valeur, entre 0 et 100.
    pub fn valeur(self) -> u8 {
        // But : lire le champ.
        todo!()
    }

    /// 100 - p : toujours un pourcentage valide, donc pas de `Result`.
    ///
    /// **Aide Rust** (ch00, section 1)
    /// - N'appelez pas `try_from` : il attend un `i64`
    ///   (`E0277 the trait bound Pourcentage: TryFrom<u8> is not satisfied`) et rendrait un `Result`. Dans ce module,
    ///   vous avez le droit de construire `Pourcentage(...)` directement : `100 - self.0` ne peut pas déborder,
    ///   l'invariant le garantit.
    pub fn complement(self) -> Pourcentage {
        // But : construire directement, l'invariant garantit le résultat.
        todo!()
    }
}

/// Le prix après remise, arrondi comme la version naïve. Ne déborde jamais (pour un prix sous `u64::MAX / 100`).
///
/// **Aide Rust** (ch00, section 1)
/// - Rust ne convertit jamais un entier tout seul. `prix_centimes * remise` donne
///   `E0277 cannot multiply u64 by Pourcentage`, et avec `remise.valeur()` : `E0277 cannot multiply u64 by u8`.
///   Convertissez sans perte avec `u64::from(...)`.
pub fn appliquer_remise(prix_centimes: u64, remise: Pourcentage) -> u64 {
    // But : la même formule, sans aucune vérification : le type l'a déjà faite.
    todo!()
}

/// Lit une remise saisie par un humain (« 15 », « 15 % », « 15% ») et l'applique.
///
/// **Aide Rust** (ch00, sections 11, 12)
/// - `parse` rend un `Result`, la fonction une `Option`. `?` sur le `Result` donne
///   `E0277 the ? operator can only be used on Options, not Results, in a function that returns Option` : passez par
///   `.ok()`.
/// - `parse()` sans type donne `E0284 type annotations needed` : écrivez `parse::<i64>()`.
pub fn remise_saisie(prix_centimes: u64, saisie: &str) -> Option<u64> {
    // But : parser à la frontière, puis appliquer ; une chaîne d'`Option`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    fn pc(n: i64) -> Pourcentage {
        Pourcentage::try_from(n).unwrap()
    }

    #[test]
    fn bornes() {
        assert_eq!(pc(0).valeur(), 0);
        assert_eq!(pc(100).valeur(), 100);
        assert_eq!(Pourcentage::try_from(101), Err(HorsBornes(101)));
        assert_eq!(Pourcentage::try_from(-1), Err(HorsBornes(-1)));
        assert_eq!(Pourcentage::try_from(i64::MAX), Err(HorsBornes(i64::MAX)));
    }

    #[test]
    fn complement_et_remise() {
        assert_eq!(pc(30).complement(), pc(70));
        assert_eq!(pc(0).complement(), pc(100));
        assert_eq!(appliquer_remise(10_000, pc(15)), 8_500);
        assert_eq!(appliquer_remise(999, pc(100)), 0);
        assert_eq!(appliquer_remise(999, pc(0)), 999);
    }

    #[test]
    fn saisie_humaine() {
        assert_eq!(remise_saisie(10_000, "15"), Some(8_500));
        assert_eq!(remise_saisie(10_000, " 15 % "), Some(8_500));
        assert_eq!(remise_saisie(10_000, "20%"), Some(8_000));
        assert_eq!(remise_saisie(10_000, "150"), None);
        assert_eq!(remise_saisie(10_000, "-5"), None);
        assert_eq!(remise_saisie(10_000, "quinze"), None);
    }

    #[test]
    #[should_panic(expected = "overflow")]
    fn le_naif_deborde_avec_150() {
        appliquer_remise_naif(10_000, 150);
    }

    #[test]
    fn meme_prix_que_le_naif_dans_les_bornes() {
        let prix = jeux::entiers(4, 50, 0, 1_000_000);
        (0..=100_u8).for_each(|r| {
            prix.iter().for_each(|&p| {
                assert_eq!(
                    appliquer_remise(p as u64, pc(r.into())),
                    appliquer_remise_naif(p as u64, r),
                    "prix {p}, remise {r}"
                )
            })
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e02_pourcentage.rs"),
            &[
                "try_from",
                "valeur",
                "complement",
                "appliquer_remise",
                "remise_saisie",
            ],
        );
    }
}
