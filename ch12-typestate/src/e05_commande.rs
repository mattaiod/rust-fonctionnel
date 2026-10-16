//! Exercice 5 : une commande, du panier au paiement, sans version naïve : à vous.
//!
//! Panier → validée → payée. Deux transitions peuvent échouer, et l'échec ne doit pas faire perdre la
//! commande : `self` a été consommé, il faut donc le rendre dans l'erreur. Valider un panier vide rend le panier
//! (`Err(Commande<Panier>)`) ; payer le mauvais montant rend la commande validée avec la raison
//! (`Err((Commande<Validee>, ErreurPaiement))`). L'appelant peut corriger et réessayer.

/// Une ligne du panier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ligne {
    pub article: String,
    pub prix_centimes: u32,
}

/// Pourquoi un paiement est refusé.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurPaiement {
    MontantIncorrect { attendu: u32, recu: u32 },
}

/// État : on remplit le panier.
#[derive(Debug)]
pub struct Panier;

/// État : panier figé, en attente de paiement.
#[derive(Debug)]
pub struct Validee;

/// État : payée, avec la référence du paiement.
#[derive(Debug)]
pub struct Payee {
    reference: String,
}

/// Une commande.
///
/// Ce qui compile :
///
/// ```no_run
/// use ch12_typestate::e05_commande::Commande;
/// let validee = Commande::ouvrir("ines").ajouter("cahier", 350).valider().unwrap();
/// let payee = validee.payer(350).unwrap();
/// println!("{}", payee.reference());
/// ```
///
/// Ajouter un article à une commande validée ne compile pas.
///
/// ```compile_fail,E0599
/// use ch12_typestate::e05_commande::Commande;
/// let validee = Commande::ouvrir("ines").ajouter("cahier", 350).valider().unwrap();
/// let triche = validee.ajouter("stylo", 0);
/// ```
///
/// Payer un panier non validé non plus.
///
/// ```compile_fail,E0599
/// use ch12_typestate::e05_commande::Commande;
/// let payee = Commande::ouvrir("ines").ajouter("cahier", 350).payer(350);
/// ```
#[derive(Debug)]
pub struct Commande<E> {
    client: String,
    lignes: Vec<Ligne>,
    etat: E,
}

impl<E> Commande<E> {
    /// Les lignes, dans l'ordre d'ajout.
    ///
    /// **Aide Rust** (ch00, sections 2, 3)
    /// - On prête les lignes : `self.lignes` donne `E0308 … expected &[Ligne], found Vec<Ligne>`. Un `&Vec<Ligne>` se
    ///   convertit tout seul en `&[Ligne]`.
    pub fn lignes(&self) -> &[Ligne] {
        todo!()
    }

    /// La somme des prix, en centimes.
    pub fn total(&self) -> u32 {
        todo!()
    }
}

impl Commande<Panier> {
    /// Un panier vide pour `client`.
    pub fn ouvrir(client: &str) -> Self {
        todo!()
    }

    /// Ajoute une ligne à la fin du panier.
    ///
    /// **Aide Rust** (ch00, sections 3, 7)
    /// - Comme `entete` à l'exercice 3 : pas de `push` (`E0596`), un nouveau `Vec` fait de l'ancien (`into_iter()`)
    ///   et de la nouvelle ligne.
    pub fn ajouter(self, article: &str, prix_centimes: u32) -> Self {
        todo!()
    }

    /// Panier → validée ; un panier vide est refusé et rendu tel quel.
    ///
    /// **Aide Rust** (ch00, sections 2, 6)
    /// - Décidez AVANT de déplacer les champs : construire la `Commande<Validee>` puis rendre `Err(self)` donne
    ///   `E0382 use of partially moved value: self`.
    pub fn valider(self) -> Result<Commande<Validee>, Commande<Panier>> {
        todo!()
    }
}

impl Commande<Validee> {
    /// Validée → payée si `montant_centimes` est exactement le total. La référence vaut
    /// `CMD-<client en majuscules>-<total>`, par exemple `CMD-INES-350`.
    ///
    /// **Aide Rust** (ch00, sections 2, 6)
    /// - L'erreur est un tuple `(commande, raison)` : `Err(ErreurPaiement::...)` seul donne
    ///   `E0308 … expected (Commande<Validee>, ErreurPaiement), found ErreurPaiement`.
    /// - Calculez le total et la référence dans des `let` avant de déplacer `self`. Sinon :
    ///   `E0382 borrow of moved value: self` (si `self` part dans le `Err` avant `self.total()`), ou
    ///   `E0382 borrow of moved value: self.client` (si `client: self.client` passe avant le `format!`).
    pub fn payer(
        self,
        montant_centimes: u32,
    ) -> Result<Commande<Payee>, (Commande<Validee>, ErreurPaiement)> {
        todo!()
    }
}

impl Commande<Payee> {
    /// La référence du paiement.
    pub fn reference(&self) -> &str {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::style;

    #[test]
    fn parcours_complet() {
        let panier = Commande::ouvrir("ines")
            .ajouter("cahier", 350)
            .ajouter("stylo", 120);
        assert_eq!(panier.total(), 470);
        assert_eq!(panier.lignes()[1].article, "stylo");
        let payee = panier.valider().unwrap().payer(470).unwrap();
        assert_eq!(payee.reference(), "CMD-INES-470");
        assert_eq!(payee.lignes().len(), 2);
    }

    #[test]
    fn panier_vide_rendu() {
        let rendu = Commande::ouvrir("ines").valider().unwrap_err();
        let payee = rendu
            .ajouter("gomme", 80)
            .valider()
            .unwrap()
            .payer(80)
            .unwrap();
        assert_eq!(payee.reference(), "CMD-INES-80");
    }

    #[test]
    fn mauvais_montant_puis_bon() {
        let validee = Commande::ouvrir("tom")
            .ajouter("livre", 1990)
            .valider()
            .unwrap();
        let (validee, erreur) = validee.payer(1900).unwrap_err();
        assert_eq!(
            erreur,
            ErreurPaiement::MontantIncorrect {
                attendu: 1990,
                recu: 1900
            }
        );
        assert_eq!(validee.payer(1990).unwrap().reference(), "CMD-TOM-1990");
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e05_commande.rs"),
            &[
                "lignes",
                "total",
                "ouvrir",
                "ajouter",
                "valider",
                "payer",
                "reference",
            ],
        );
    }
}
