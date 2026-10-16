//! Exercice 1 : une porte qu'on ne peut pas ouvrir quand elle est verrouillée.
//!
//! Avec un `enum` d'état, une transition interdite est un `panic!` qu'on découvre à l'exécution. Avec un type
//! par état, elle n'existe pas : `ouvrir` n'est défini que sur `Porte<Fermee>`, et l'erreur tombe à la
//! compilation. Les états ne portent aucune donnée : ce sont des structs vides, et le champ `PhantomData<E>`
//! « utilise » le paramètre `E` sans rien stocker. Chaque transition prend `self` : l'ancienne porte est
//! consommée, on ne peut plus s'en servir.

use std::marker::PhantomData;

/// Version naïve : l'état est une valeur.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EtatNaif {
    Ouverte,
    Fermee,
    Verrouillee,
}

/// Version naïve : une action est une valeur, toutes les actions sont toujours appelables.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Ouvrir,
    Fermer,
    Verrouiller,
    Deverrouiller,
}

/// Version naïve : rien n'empêche d'écrire `actionner(Action::Ouvrir)` sur une porte verrouillée ; on le
/// découvre à l'exécution, par un `panic!`. Ne pas modifier.
#[derive(Debug)]
pub struct PorteNaive {
    pub etat: EtatNaif,
    pub ouvertures: u32,
}

impl PorteNaive {
    /// Une porte neuve est fermée (et pas verrouillée). Ne pas modifier.
    pub fn neuve_naive() -> Self {
        PorteNaive {
            etat: EtatNaif::Fermee,
            ouvertures: 0,
        }
    }

    /// Applique une action ; panique si elle est interdite dans l'état courant. Ne pas modifier.
    pub fn actionner(&mut self, action: Action) {
        self.etat = match (self.etat, action) {
            (EtatNaif::Fermee, Action::Ouvrir) => {
                self.ouvertures += 1;
                EtatNaif::Ouverte
            }
            (EtatNaif::Ouverte, Action::Fermer) => EtatNaif::Fermee,
            (EtatNaif::Fermee, Action::Verrouiller) => EtatNaif::Verrouillee,
            (EtatNaif::Verrouillee, Action::Deverrouiller) => EtatNaif::Fermee,
            (etat, action) => panic!("{action:?} impossible : la porte est {etat:?}"),
        }
    }
}

/// État : porte ouverte.
#[derive(Debug)]
pub struct Ouverte;
/// État : porte fermée, pas verrouillée.
#[derive(Debug)]
pub struct Fermee;
/// État : porte fermée à clé.
#[derive(Debug)]
pub struct Verrouillee;

/// La porte typée : son état est dans son type, `Porte<Fermee>` et `Porte<Verrouillee>` sont deux types
/// différents. Elle compte combien de fois elle a été ouverte.
///
/// Ce qui compile :
///
/// ```no_run
/// use ch12_typestate::e01_porte::Porte;
/// let porte = Porte::neuve().ouvrir().fermer().verrouiller().deverrouiller();
/// assert_eq!(porte.ouvertures(), 1);
/// ```
///
/// Ouvrir une porte verrouillée ne compile pas : `ouvrir` n'existe pas sur `Porte<Verrouillee>`.
///
/// ```compile_fail,E0599
/// use ch12_typestate::e01_porte::Porte;
/// let porte = Porte::neuve().verrouiller().ouvrir();
/// ```
///
/// Se servir d'une porte après une transition non plus : elle a été consommée.
///
/// ```compile_fail,E0382
/// use ch12_typestate::e01_porte::Porte;
/// let fermee = Porte::neuve();
/// let ouverte = fermee.ouvrir();
/// let verrouillee = fermee.verrouiller();
/// ```
#[derive(Debug)]
pub struct Porte<E> {
    ouvertures: u32,
    _etat: PhantomData<E>,
}

impl<E> Porte<E> {
    /// Nombre d'ouvertures depuis la pose, dans n'importe quel état.
    pub fn ouvertures(&self) -> u32 {
        todo!()
    }
}

impl Porte<Fermee> {
    /// Une porte neuve : fermée, jamais ouverte.
    ///
    /// **Aide Rust** (ch00, section 1)
    /// - Tous les champs sont obligatoires, `_etat` compris : `E0063 missing field _etat in initializer of Porte<_>`.
    /// - Ce champ vaut `PhantomData`, pas l'état lui-même : `_etat: Fermee` donne
    ///   `E0308 … expected PhantomData<Fermee>, found Fermee`. Le type de retour (`Self`, donc `Porte<Fermee>`)
    ///   suffit à fixer l'état.
    pub fn neuve() -> Self {
        // But : construire l'état de départ.
        todo!()
    }

    /// Fermée → ouverte (et une ouverture de plus).
    ///
    /// **Aide Rust** (ch00, sections 2, 7)
    /// - Changer d'état, c'est construire une nouvelle `Porte { ... }` d'un autre type. `self.ouvertures += 1; self`
    ///   rend une `Porte<Fermee>` : `E0308 … expected Porte<Ouverte>, found Porte<Fermee>`.
    /// - `..self` et `Self { ... }` gardent l'ancien type (ici `Self` veut dire `Porte<Fermee>`) : même erreur
    ///   `E0308`. Écrivez `Porte { ... }` et listez les champs.
    pub fn ouvrir(self) -> Porte<Ouverte> {
        // But : rendre la même porte, dans un autre type.
        todo!()
    }

    /// Fermée → verrouillée.
    ///
    /// **Aide Rust** (ch00, section 2)
    /// - Même forme que `ouvrir`, sans toucher au compteur. Valable aussi pour `fermer` et `deverrouiller`.
    /// - Côté appelant, `self` est consommé : réutiliser l'ancienne porte donne `E0382 use of moved value: fermee`.
    ///   Gardez la valeur rendue (`let ouverte = fermee.ouvrir();`).
    pub fn verrouiller(self) -> Porte<Verrouillee> {
        todo!()
    }
}

impl Porte<Ouverte> {
    /// Ouverte → fermée.
    pub fn fermer(self) -> Porte<Fermee> {
        todo!()
    }
}

impl Porte<Verrouillee> {
    /// Verrouillée → fermée.
    pub fn deverrouiller(self) -> Porte<Fermee> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::style;

    #[test]
    fn cycle_complet() {
        let porte = Porte::neuve();
        assert_eq!(porte.ouvertures(), 0);
        let porte = porte.ouvrir().fermer().ouvrir().fermer();
        assert_eq!(porte.ouvertures(), 2);
        let porte = porte.verrouiller();
        assert_eq!(porte.ouvertures(), 2);
        assert_eq!(porte.deverrouiller().ouvrir().ouvertures(), 3);
    }

    #[test]
    fn l_etat_ne_coute_rien() {
        assert_eq!(
            std::mem::size_of::<Porte<Verrouillee>>(),
            std::mem::size_of::<u32>()
        );
    }

    #[test]
    fn meme_compte_que_le_naif() {
        let mut naive = PorteNaive::neuve_naive();
        [
            Action::Ouvrir,
            Action::Fermer,
            Action::Verrouiller,
            Action::Deverrouiller,
            Action::Ouvrir,
        ]
        .into_iter()
        .for_each(|a| naive.actionner(a));
        let typee = Porte::neuve()
            .ouvrir()
            .fermer()
            .verrouiller()
            .deverrouiller()
            .ouvrir();
        assert_eq!(typee.ouvertures(), naive.ouvertures);
    }

    #[test]
    #[should_panic(expected = "Ouvrir impossible")]
    fn le_naif_panique_a_l_execution() {
        let mut naive = PorteNaive::neuve_naive();
        naive.actionner(Action::Verrouiller);
        naive.actionner(Action::Ouvrir);
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e01_porte.rs"),
            &[
                "ouvertures",
                "neuve",
                "ouvrir",
                "verrouiller",
                "fermer",
                "deverrouiller",
            ],
        );
    }
}
