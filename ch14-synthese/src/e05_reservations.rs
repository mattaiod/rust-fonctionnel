//! Exercice 5 : mini-moteur de réservation de salles. Vous écrivez aussi les tests.
//!
//! Les types et les signatures sont posés, quelques tests aussi. À vous : les corps des fonctions, puis les tests
//! marqués `#[ignore = "à écrire par l'élève"]` en bas du fichier. Pour chacun, le commentaire dit le
//! comportement à vérifier : remplacez le `todo!()` par le test, retirez la ligne `#[ignore]`, lancez-le.
//! Un bon test se lit comme une phrase : on prépare (un planning), on agit (réserver), on vérifie (`assert_eq!`).
//! Écrivez chaque test AVANT la fonction qu'il vérifie et regardez-le échouer : un test qui n'a jamais échoué
//! ne prouve rien.

/// Un créneau dans une journée, en minutes depuis minuit : `debut` < `fin` <= 1440. Les champs sont privés : un
/// créneau vide ou à l'envers ne peut pas exister.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Creneau {
    debut: u16,
    fin: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reservation {
    pub salle: String,
    pub creneau: Creneau,
    pub organisateur: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurReservation {
    /// `debut` >= `fin`.
    CreneauVide,
    /// La fin dépasse minuit (1440 minutes).
    HorsJournee,
    /// La salle est déjà prise sur une partie du créneau, par cette réservation.
    Conflit(Reservation),
}

/// Toutes les réservations. Invariant : deux réservations d'une même salle ne se chevauchent jamais.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Planning {
    reservations: Vec<Reservation>,
}

/// Minutes dans une journée.
pub const JOURNEE: u16 = 24 * 60;

impl Creneau {
    /// Le seul chemin pour construire un créneau.
    pub fn nouveau(debut: u16, fin: u16) -> Result<Creneau, ErreurReservation> {
        todo!()
    }

    pub fn debut(self) -> u16 {
        self.debut
    }

    pub fn fin(self) -> u16 {
        self.fin
    }

    /// Vrai si les deux créneaux ont au moins une minute en commun. 10 h-11 h et 11 h-12 h ne se chevauchent pas.
    ///
    /// **Aide Rust** (ch00, section 2)
    /// - Les créneaux sont des intervalles `[debut, fin[` : 10 h-11 h et 11 h-12 h ne se touchent pas. Des `<`
    ///   stricts.
    pub fn chevauche(&self, autre: &Creneau) -> bool {
        todo!()
    }
}

impl Planning {
    /// Un nouveau planning avec cette réservation en plus, ou le conflit qui l'empêche (la première réservation
    /// de la même salle qui chevauche).
    ///
    /// **Aide Rust** (ch00, sections 2, 6)
    /// - `find` rend un `&Reservation`, l'erreur en possède une : `Conflit(c)` donne
    ///   `E0308 … expected Reservation, found &Reservation`. `c.clone()`.
    /// - Le nouveau planning : les anciennes réservations clonées (`iter().cloned()`) suivies de la nouvelle
    ///   (`chain`).
    pub fn reserver(&self, reservation: Reservation) -> Result<Planning, ErreurReservation> {
        todo!()
    }

    /// Un nouveau planning sans la réservation de cette salle sur exactement ce créneau (inchangé s'il n'y en a pas).
    ///
    /// **Aide Rust** (ch00, sections 2, 3)
    /// - `r.creneau == creneau` donne `E0308 … expected Creneau, found &Creneau` : `*creneau`.
    /// - `filter` garde des `&Reservation` : `collect()` donne
    ///   `E0277 a value of type Vec<Reservation> cannot be built from an iterator over elements of type &Reservation`.
    ///   `.cloned()` avant.
    pub fn annuler(&self, salle: &str, creneau: &Creneau) -> Planning {
        todo!()
    }

    /// Les réservations d'une salle, de la plus tôt à la plus tard.
    pub fn reservations_de(&self, salle: &str) -> Vec<&Reservation> {
        todo!()
    }

    /// Les créneaux libres d'une salle pendant ses heures d'ouverture, dans l'ordre. Une réservation qui déborde
    /// de l'ouverture ne compte que pour sa partie à l'intérieur.
    ///
    /// **Aide Rust** (ch00, sections 7, 12)
    /// - Le plan : les réservations de la salle (déjà triées par `reservations_de`), sans celles qui ne chevauchent
    ///   pas l'ouverture ; un `fold` dont l'accumulateur est `(curseur, libres)`, le curseur partant du début de
    ///   l'ouverture. Chaque réservation qui commence après le curseur laisse un trou `[curseur, son début[` ; le
    ///   curseur passe ensuite au plus grand de lui-même et de sa fin. À la fin, un dernier trou
    ///   `[curseur, fin de l'ouverture[` s'il reste du temps.
    /// - Dans ce module, vous pouvez écrire `Creneau { debut, fin }` directement (champs privés mais module
    ///   courant) ; `Creneau::nouveau` rend un `Result`
    ///   (`E0308 … expected Creneau, found Result<Creneau, ErreurReservation>`).
    /// - `|(curseur, mut libres), creneau|` est permis : c'est le `mut` d'un accumulateur de `fold`.
    pub fn creneaux_libres(&self, salle: &str, ouverture: Creneau) -> Vec<Creneau> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::style;

    /// `h(9, 30)` : 9 h 30 en minutes.
    fn h(heures: u16, minutes: u16) -> u16 {
        heures * 60 + minutes
    }

    fn creneau(debut: u16, fin: u16) -> Creneau {
        Creneau::nouveau(debut, fin).unwrap()
    }

    fn reservation(salle: &str, debut: u16, fin: u16, organisateur: &str) -> Reservation {
        Reservation {
            salle: salle.to_string(),
            creneau: creneau(debut, fin),
            organisateur: organisateur.to_string(),
        }
    }

    // Tests fournis.

    #[test]
    fn creneaux_impossibles() {
        assert_eq!(
            Creneau::nouveau(h(10, 0), h(10, 0)),
            Err(ErreurReservation::CreneauVide)
        );
        assert_eq!(
            Creneau::nouveau(h(11, 0), h(10, 0)),
            Err(ErreurReservation::CreneauVide)
        );
        assert_eq!(
            Creneau::nouveau(h(23, 0), JOURNEE + 1),
            Err(ErreurReservation::HorsJournee)
        );
        assert!(Creneau::nouveau(0, JOURNEE).is_ok());
    }

    #[test]
    fn chevauchements() {
        let matin = creneau(h(9, 0), h(12, 0));
        assert!(matin.chevauche(&creneau(h(11, 0), h(13, 0))));
        assert!(matin.chevauche(&creneau(h(10, 0), h(10, 30))));
        assert!(creneau(h(10, 0), h(10, 30)).chevauche(&matin));
        assert!(!matin.chevauche(&creneau(h(12, 0), h(13, 0))));
        assert!(!matin.chevauche(&creneau(h(7, 0), h(9, 0))));
    }

    #[test]
    fn reserver_dans_un_planning_vide() {
        let r = reservation("Turing", h(9, 0), h(10, 0), "Alice");
        let planning = Planning::default().reserver(r.clone()).unwrap();
        assert_eq!(planning.reservations_de("Turing"), vec![&r]);
        assert!(planning.reservations_de("Lovelace").is_empty());
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e05_reservations.rs"),
            &[
                "nouveau",
                "chevauche",
                "reserver",
                "annuler",
                "reservations_de",
                "creneaux_libres",
            ],
        );
    }

    // Tests à écrire. Une fois la ligne `#[ignore]` retirée, `cargo test -p ch14-synthese e05` lance le test ;
    // `cargo test -p ch14-synthese e05 -- --ignored` lance ceux qui restent à écrire (voir le README).

    /// **Aide Rust** (ch00, sections 6, 11)
    /// - `reserver` prend la réservation par valeur : après `reserver(alice)`, réutiliser `alice` dans l'`assert_eq!`
    ///   donne `E0382 use of moved value: alice`. Passez `alice.clone()` à `reserver`.
    /// - `reserver` rend un `Result<Planning, _>` : `.unwrap()` dans un test est permis. Enchaîner
    ///   `.reserver(...).reserver(...)` donne `E0599 no method named reserver found for enum Result` : `.unwrap()`
    ///   entre les deux, ou `.and_then(|p| p.reserver(...))`.
    #[test]
    #[ignore = "à écrire par l'élève"]
    fn refuse_un_chevauchement_dans_la_meme_salle() {
        // Turing réservée 9 h-11 h par Alice ; Bob demande Turing 10 h-12 h : refusé, et l'erreur désigne la
        // réservation d'Alice.
        todo!()
    }

    #[test]
    #[ignore = "à écrire par l'élève"]
    fn meme_creneau_dans_deux_salles() {
        // Le même créneau dans deux salles différentes : les deux réservations passent.
        todo!()
    }

    #[test]
    #[ignore = "à écrire par l'élève"]
    fn creneaux_bout_a_bout_acceptes() {
        // 10 h-11 h puis 11 h-12 h dans la même salle : pas de chevauchement, les deux passent.
        todo!()
    }

    #[test]
    #[ignore = "à écrire par l'élève"]
    fn reserver_ne_modifie_pas_le_planning_d_origine() {
        // Après une réservation réussie, l'ancien planning est inchangé (toujours vide, par exemple).
        todo!()
    }

    /// **Aide Rust** (ch00, section 11)
    /// - `reservations_de` rend des `&Reservation` : comparer à un `vec![reservation(...)]` donne
    ///   `E0277 can't compare &Reservation with Reservation`. Comparez les débuts (`r.creneau.debut()`) ou un
    ///   `vec![&r]`.
    #[test]
    #[ignore = "à écrire par l'élève"]
    fn reservations_triees_par_heure() {
        // Réserver 14 h, puis 9 h, puis 11 h : `reservations_de` les rend dans l'ordre 9 h, 11 h, 14 h.
        todo!()
    }

    #[test]
    #[ignore = "à écrire par l'élève"]
    fn annuler_libere_le_creneau() {
        // Annuler la réservation d'Alice permet à Bob de prendre le même créneau ; annuler un créneau qui n'existe
        // pas ne change rien.
        todo!()
    }

    #[test]
    #[ignore = "à écrire par l'élève"]
    fn salle_vide_libre_toute_l_ouverture() {
        // Sans réservation, le seul créneau libre est l'ouverture entière.
        todo!()
    }

    #[test]
    #[ignore = "à écrire par l'élève"]
    fn creneaux_libres_entre_et_autour_des_reservations() {
        // Ouverture 8 h-18 h ; réservations 7 h-9 h (déborde avant l'ouverture), 12 h-13 h, 13 h-14 h et 17 h-20 h
        // (déborde après) ; une réservation dans une autre salle ne compte pas.
        // Libres attendus : 9 h-12 h et 14 h-17 h.
        todo!()
    }
}
