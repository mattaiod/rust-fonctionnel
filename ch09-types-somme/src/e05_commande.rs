//! Exercice 5 : le cycle de vie d'une commande.
//!
//! Trois booléens et une `Option` : 2 × 2 × 2 × 2 = 16 états, pour 4 cas réels (en attente, payée, expédiée,
//! annulée). Payée ET annulée ? Expédiée sans numéro de suivi ? Chaque fonction naïve doit revérifier la
//! cohérence, et la moindre omission laisse passer un état absurde. Avec l'`enum`, une transition est une
//! fonction pure qui **consomme** l'ancien état et rend le nouveau, ou une erreur qui dit pourquoi c'est refusé.
//!
//! Une commande expédiée sans numéro de suivi ne se construit même pas :
//!
//! ```compile_fail,E0533
//! use ch09_types_somme::e05_commande::EtatCommande;
//! let etat = EtatCommande::Expediee; // E0533 : la variante attend `numero_suivi`
//! ```
//!
//! ```
//! use ch09_types_somme::e05_commande::EtatCommande;
//! let etat = EtatCommande::Expediee { numero_suivi: "6A123".to_string() };
//! ```

/// Version naïve. Ne pas modifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandeNaive {
    pub payee: bool,
    pub expediee: bool,
    pub annulee: bool,
    pub numero_suivi: Option<String>,
}

impl CommandeNaive {
    /// Version naïve de l'expédition : `false` si refusée, sans dire pourquoi. Ne pas modifier.
    ///
    /// Elle oublie de tester `annulee` : une commande payée puis annulée part quand même.
    pub fn expedier_naif(&mut self, numero_suivi: &str) -> bool {
        if !self.payee || self.expediee {
            return false;
        }
        self.expediee = true;
        self.numero_suivi = Some(numero_suivi.to_string());
        true
    }
}

/// Les quatre états d'une commande. Le numéro de suivi n'existe que pour une commande expédiée.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EtatCommande {
    EnAttente,
    Payee,
    Expediee { numero_suivi: String },
    Annulee,
}

/// Pourquoi une transition est refusée.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErreurCommande {
    DejaPayee,
    PasEncorePayee,
    DejaExpediee,
    DejaAnnulee,
}

/// Payer : seulement depuis `EnAttente`. Une commande payée ou expédiée → `DejaPayee` ; annulée → `DejaAnnulee`.
///
/// **Aide Rust** (ch00, section 2)
/// - `Expediee` porte un champ : `` error[E0533]: expected unit struct, unit variant or constant, found struct variant
///   `EtatCommande::Expediee` `` → `EtatCommande::Expediee { .. }`.
pub fn payer(etat: EtatCommande) -> Result<EtatCommande, ErreurCommande> {
    // But : un `match`, un bras par état.
    todo!()
}

/// Expédier : seulement depuis `Payee`. En attente → `PasEncorePayee` ; expédiée → `DejaExpediee` ;
/// annulée → `DejaAnnulee`.
///
/// **Aide Rust** (ch00, section 5)
/// - Le champ `numero_suivi` attend une `String`, le paramètre est un `&str` : `` expected `String`, found `&str` `` →
///   `.to_string()`.
pub fn expedier(etat: EtatCommande, numero_suivi: &str) -> Result<EtatCommande, ErreurCommande> {
    // But : un `match`, un bras par état.
    todo!()
}

/// Annuler : depuis `EnAttente` ou `Payee`. Expédiée → `DejaExpediee` ; annulée → `DejaAnnulee`.
pub fn annuler(etat: EtatCommande) -> Result<EtatCommande, ErreurCommande> {
    // But : un `match` ; deux états mènent au même résultat.
    todo!()
}

/// Le numéro de suivi, s'il existe.
///
/// **Aide Rust** (ch00, sections 5, 11)
/// - `match etat` sur un `&EtatCommande` lie `numero_suivi: &String` ; `Some(numero_suivi)` devient tout seul une
///   `Option<&str>`. Avec `.clone()` : `` expected `&str`, found `String` ``.
pub fn numero_suivi(etat: &EtatCommande) -> Option<&str> {
    // But : un seul état en porte un.
    todo!()
}

/// Le parcours complet : payer, puis expédier avec `numero`, en partant d'une commande en attente.
///
/// **Aide Rust** (ch00, sections 9, 11)
/// - Si le compilateur dit `error[E0593]: function is expected to take 1 argument, but it takes 2 arguments` :
///   `and_then(expedier)` ne suffit pas, `expedier` veut aussi le numéro. Une closure le lui passe.
/// - Si le compilateur dit `` found `Result<Result<EtatCommande, …>, …>` `` : `map` → `and_then`.
pub fn payer_puis_expedier(numero: &str) -> Result<EtatCommande, ErreurCommande> {
    // But : le railway, en une ligne.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use EtatCommande::*;
    use pf_outils::style;

    fn expediee() -> EtatCommande {
        Expediee {
            numero_suivi: "6A123".to_string(),
        }
    }

    #[test]
    fn chemin_nominal() {
        let etat = payer(EnAttente).and_then(|e| expedier(e, "6A123"));
        assert_eq!(etat, Ok(expediee()));
        assert_eq!(payer_puis_expedier("6A123"), Ok(expediee()));
        assert_eq!(numero_suivi(&expediee()), Some("6A123"));
        assert_eq!(numero_suivi(&Payee), None);
    }

    #[test]
    fn payer_refuse() {
        assert_eq!(payer(Payee), Err(ErreurCommande::DejaPayee));
        assert_eq!(payer(expediee()), Err(ErreurCommande::DejaPayee));
        assert_eq!(payer(Annulee), Err(ErreurCommande::DejaAnnulee));
    }

    #[test]
    fn expedier_refuse() {
        assert_eq!(
            expedier(EnAttente, "x"),
            Err(ErreurCommande::PasEncorePayee)
        );
        assert_eq!(expedier(expediee(), "x"), Err(ErreurCommande::DejaExpediee));
        assert_eq!(expedier(Annulee, "x"), Err(ErreurCommande::DejaAnnulee));
    }

    #[test]
    fn annuler_et_ses_refus() {
        assert_eq!(annuler(EnAttente), Ok(Annulee));
        assert_eq!(annuler(Payee), Ok(Annulee));
        assert_eq!(annuler(expediee()), Err(ErreurCommande::DejaExpediee));
        assert_eq!(annuler(Annulee), Err(ErreurCommande::DejaAnnulee));
    }

    #[test]
    fn le_bug_du_naif_est_impossible_ici() {
        // Naïf : payée puis annulée... et expédiée quand même.
        let mut naive = CommandeNaive {
            payee: true,
            expediee: false,
            annulee: true,
            numero_suivi: None,
        };
        assert!(naive.expedier_naif("6A123"));
        assert!(naive.annulee && naive.expediee);
        // Typé : l'état « annulée » n'a qu'une sortie possible, le refus.
        let etat = payer(EnAttente)
            .and_then(annuler)
            .and_then(|e| expedier(e, "6A123"));
        assert_eq!(etat, Err(ErreurCommande::DejaAnnulee));
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e05_commande.rs"),
            &[
                "payer",
                "expedier",
                "annuler",
                "numero_suivi",
                "payer_puis_expedier",
            ],
        );
    }
}
