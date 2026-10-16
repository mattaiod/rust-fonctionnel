//! Exercice 4 : panier e-commerce typé de bout en bout.
//!
//! Un prix est un nombre entier de centimes (jamais un `f64` : 0,1 + 0,2 ≠ 0,3), une quantité est strictement
//! positive, une référence produit (SKU) a un format. Chaque règle vit dans un type : une fois le panier construit,
//! il ne peut plus être faux. Le panier ne se modifie pas : chaque opération rend un nouveau panier.
//! Un code promo est une donnée (`enum`) ; l'appliquer, c'est le transformer en fonction
//! `Addition -> Addition`. Plusieurs codes = ces fonctions composées les unes après les autres.
//! `Pourcentage` est donné en exemple de newtype : faites pareil pour `Quantite` et `Sku`.

use std::iter::Sum;
use std::num::NonZeroU32;
use std::ops::Add;

/// Une somme d'argent, en centimes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Centimes(pub u64);

/// Une quantité strictement positive. Le champ est privé : on ne la fabrique que par `Quantite::nouvelle`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Quantite(NonZeroU32);

/// Une référence produit : non vide, faite de majuscules ASCII, de chiffres et de tirets (`"LIVRE-042"`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sku(String);

/// Un pourcentage de 0 à 100.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pourcentage(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Categorie {
    Livres,
    Vetements,
    Maison,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurPanier {
    QuantiteNulle,
    SkuInvalide(String),
    PourcentageInvalide(u8),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Article {
    pub sku: Sku,
    pub categorie: Categorie,
    pub prix_unitaire: Centimes,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ligne {
    pub article: Article,
    pub quantite: Quantite,
}

/// Un panier : au plus une ligne par SKU. Le champ est privé, seules les opérations ci-dessous le construisent.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Panier {
    lignes: Vec<Ligne>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodePromo {
    /// Remise de ce pourcentage sur les articles de cette catégorie (arrondie au centime inférieur).
    PourcentageSur(Categorie, Pourcentage),
    /// Remise de ce montant sur le panier.
    MontantFixe(Centimes),
    /// Frais de port offerts.
    LivraisonOfferte,
}

/// Le détail du prix à payer, avant de calculer le total.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Addition {
    pub sous_total: Centimes,
    pub remise: Centimes,
    pub livraison: Centimes,
}

/// Frais de port d'un panier non vide.
pub const FRAIS_DE_PORT: Centimes = Centimes(490);

impl Pourcentage {
    /// Exemple de constructeur intelligent : le seul chemin d'entrée vérifie la règle.
    pub fn sur_cent(valeur: u8) -> Result<Pourcentage, ErreurPanier> {
        (valeur <= 100)
            .then_some(Pourcentage(valeur))
            .ok_or(ErreurPanier::PourcentageInvalide(valeur))
    }

    pub fn valeur(self) -> u8 {
        self.0
    }
}

impl Add for Centimes {
    type Output = Centimes;

    /// **Aide Rust** (ch00, section 1)
    /// - Additionnez les champs (`self.0`), pas `self + autre`, qui rappellerait ce même `add` sans fin.
    fn add(self, autre: Centimes) -> Centimes {
        todo!()
    }
}

impl Sum for Centimes {
    /// La somme d'une suite de montants (`Centimes(0)` si elle est vide).
    ///
    /// **Aide Rust** (ch00, section 7)
    /// - Un `fold` qui part de `Centimes(0)` et réutilise `Add`. `montants.sum()` ici se rappellerait lui-même
    ///   (`stack overflow`).
    fn sum<I: Iterator<Item = Centimes>>(montants: I) -> Centimes {
        todo!()
    }
}

impl Centimes {
    /// Le prix de `quantite` articles à ce prix unitaire.
    ///
    /// **Aide Rust** (ch00, section 1)
    /// - `self.0 * quantite` donne `E0277 cannot multiply u64 by Quantite` : passez par `quantite.valeur()` (un
    ///   `u32`), puis `u64::from(...)`.
    pub fn fois(self, quantite: Quantite) -> Centimes {
        todo!()
    }
}

impl Quantite {
    /// Une quantité, refusée si elle vaut 0.
    ///
    /// **Aide Rust** (ch00, section 11)
    /// - `NonZeroU32::new` rend une `Option` : `Quantite(NonZeroU32::new(valeur))` donne
    ///   `E0308 … expected NonZero<u32>, found Option<NonZero<u32>>`. Un nom de tuple struct est une fonction :
    ///   `.map(Quantite)` marche.
    pub fn nouvelle(valeur: u32) -> Result<Quantite, ErreurPanier> {
        todo!()
    }

    pub fn valeur(self) -> u32 {
        self.0.get()
    }

    /// La somme de deux quantités (plafonnée à `u32::MAX`).
    ///
    /// **Aide Rust** (ch00, section 1)
    /// - `NonZeroU32` n'a pas de `+` : `E0369 cannot add NonZero<u32> to NonZero<u32>`. Il a en revanche une addition
    ///   plafonnée qui prend un `u32` (voir sa doc).
    pub fn plus(self, autre: Quantite) -> Quantite {
        todo!()
    }

    /// Ce qui reste après en avoir retiré `autre` ; `None` s'il ne reste rien.
    ///
    /// **Aide Rust** (ch00, section 11)
    /// - `self.valeur() - autre.valeur()` panique si on retire plus qu'il n'y a
    ///   (`attempt to subtract with overflow`) : `checked_sub` rend une `Option`, et un reste de 0 doit aussi donner
    ///   `None`.
    pub fn moins(self, autre: Quantite) -> Option<Quantite> {
        todo!()
    }
}

impl Sku {
    /// Une référence produit, refusée si elle ne respecte pas le format.
    ///
    /// **Aide Rust** (ch00, sections 5, 9)
    /// - `bool::then` attend une closure : `.then(Sku(...))` donne `E0277 expected a FnOnce() closure, found Sku`.
    ///   Écrivez `.then(|| ...)`, ou `then_some(...)` avec une valeur.
    /// - `is_ascii_uppercase`, pas `is_uppercase` : « É » est une majuscule, mais pas ASCII.
    pub fn depuis(code: &str) -> Result<Sku, ErreurPanier> {
        todo!()
    }
}

impl Panier {
    pub fn lignes(&self) -> &[Ligne] {
        &self.lignes
    }

    /// Un nouveau panier avec `quantite` articles de plus. Si le SKU est déjà là, sa quantité augmente
    /// (l'article déjà présent est gardé) ; sinon une ligne s'ajoute à la fin.
    ///
    /// **Aide Rust** (ch00, sections 2, 3, 6)
    /// - `self` est prêté : `self.lignes.push(...)` donne
    ///   `E0596 cannot borrow self.lignes as mutable, as it is behind a & reference`. On construit un nouveau
    ///   `Panier`.
    /// - `iter()` donne des `&Ligne` : `Ligne { quantite: ..., ..ligne }` donne
    ///   `E0308 … expected Ligne, found &Ligne` (écrivez `..ligne.clone()`), et
    ///   `chain(once(Ligne { article, quantite }))` derrière `iter()` donne `E0271 … expected &Ligne, found Ligne`
    ///   (ajoutez `.cloned()` avant `chain`).
    /// - Ne gardez pas d'emprunt de `article` (par exemple `article.sku` dans une closure) dans la même chaîne qui
    ///   déplace `article` : `E0505 cannot move out of article because it is borrowed`. Calculez d'abord un booléen
    ///   « déjà là ? », puis deux branches.
    pub fn ajouter(&self, article: Article, quantite: Quantite) -> Panier {
        todo!()
    }

    /// Un nouveau panier avec `quantite` articles de ce SKU en moins. La ligne disparaît s'il n'en reste plus ;
    /// un SKU absent ne change rien.
    ///
    /// **Aide Rust** (ch00, sections 2, 4, 11)
    /// - `ligne.article.sku == sku` compare un `Sku` à un `&Sku` : `E0308 … expected Sku, found &Sku`. Écrivez
    ///   `*sku`.
    /// - `filter_map` : chaque ligne devient `Some(ligne gardée ou réduite)` ou `None` (ligne disparue) ;
    ///   `Quantite::moins` rend justement une `Option`.
    pub fn retirer(&self, sku: &Sku, quantite: Quantite) -> Panier {
        todo!()
    }

    /// Prix des articles, sans remise ni livraison.
    ///
    /// **Aide Rust** (ch00, section 3)
    /// - Pas de `*` entre `Centimes` et `Quantite` (`E0369 cannot multiply Centimes by Quantite`) : c'est `fois`. Le
    ///   `.sum()` final marche grâce à votre `impl Sum`.
    pub fn sous_total(&self) -> Centimes {
        todo!()
    }

    /// Le total à payer : sous-total moins les remises (le sous-total ne descend pas sous zéro), plus la
    /// livraison (`FRAIS_DE_PORT` si le panier n'est pas vide). Les codes s'appliquent tous, dans l'ordre.
    ///
    /// **Aide Rust** (ch00, sections 9, 10)
    /// - `Centimes` n'a pas de `-` : `E0369 cannot subtract Centimes from Centimes`. Soustrayez les `u64` avec
    ///   `saturating_sub` (jamais sous zéro).
    /// - Appliquer les codes dans l'ordre : un `fold` qui part de l'`Addition` de départ et passe l'addition à
    ///   `appliquer(code, self)`, qui est une fonction.
    pub fn total(&self, codes: &[CodePromo]) -> Centimes {
        todo!()
    }
}

/// L'effet d'un code promo sur une addition, pour ce panier.
///
/// **Aide Rust** (ch00, sections 4, 8, 10, 13)
/// - `match code` sur un `&CodePromo` donne des références : `categorie` est un `&Categorie` et `montant` un
///   `&Centimes`. Les comparer ou les additionner tels quels donne `E0308 … expected Categorie, found &Categorie` :
///   déréférencez (`*categorie`, `*montant`).
/// - La closure est déjà ouverte (`move |addition| { ... }`) : écrivez seulement son corps. `Addition` est `Copy`,
///   `Addition { remise: ..., ..addition }` recopie le reste.
/// - Remise en pourcentage : `montant * pourcentage / 100` en `u64`, la division entière arrondit au centime
///   inférieur.
pub fn appliquer<'a>(
    code: &'a CodePromo,
    panier: &'a Panier,
) -> impl Fn(Addition) -> Addition + 'a {
    move |addition| todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::style;

    fn article(sku: &str, categorie: Categorie, prix: u64) -> Article {
        Article {
            sku: Sku::depuis(sku).unwrap(),
            categorie,
            prix_unitaire: Centimes(prix),
        }
    }

    fn quantite(n: u32) -> Quantite {
        Quantite::nouvelle(n).unwrap()
    }

    fn livre() -> Article {
        article("LIVRE-042", Categorie::Livres, 2000)
    }

    fn tshirt() -> Article {
        article("TSHIRT-M", Categorie::Vetements, 1500)
    }

    /// 2 livres à 20 € et 1 t-shirt à 15 € : 55 € d'articles.
    fn panier() -> Panier {
        Panier::default()
            .ajouter(livre(), quantite(2))
            .ajouter(tshirt(), quantite(1))
    }

    #[test]
    fn constructeurs_intelligents() {
        assert_eq!(Quantite::nouvelle(0), Err(ErreurPanier::QuantiteNulle));
        assert_eq!(Quantite::nouvelle(3).map(Quantite::valeur), Ok(3));
        assert!(Sku::depuis("A-1").is_ok());
        assert_eq!(
            Sku::depuis(""),
            Err(ErreurPanier::SkuInvalide(String::new()))
        );
        assert_eq!(
            Sku::depuis("livre 42"),
            Err(ErreurPanier::SkuInvalide("livre 42".to_string()))
        );
        assert_eq!(
            Pourcentage::sur_cent(101),
            Err(ErreurPanier::PourcentageInvalide(101))
        );
    }

    #[test]
    fn arithmetique_des_centimes_et_quantites() {
        assert_eq!(Centimes(150) + Centimes(250), Centimes(400));
        assert_eq!(
            [Centimes(1), Centimes(2), Centimes(3)]
                .into_iter()
                .sum::<Centimes>(),
            Centimes(6)
        );
        assert_eq!(
            std::iter::empty::<Centimes>().sum::<Centimes>(),
            Centimes(0)
        );
        assert_eq!(Centimes(199).fois(quantite(3)), Centimes(597));
        assert_eq!(quantite(2).plus(quantite(3)), quantite(5));
        assert_eq!(quantite(5).moins(quantite(2)), Some(quantite(3)));
        assert_eq!(quantite(2).moins(quantite(2)), None);
        assert_eq!(quantite(2).moins(quantite(7)), None);
    }

    #[test]
    fn ajouter_rend_un_nouveau_panier() {
        let vide = Panier::default();
        let plein = vide.ajouter(livre(), quantite(1));
        assert!(vide.lignes().is_empty());
        assert_eq!(plein.lignes().len(), 1);
    }

    #[test]
    fn ajouter_deux_fois_le_meme_sku_cumule() {
        let p = panier().ajouter(livre(), quantite(3));
        assert_eq!(p.lignes().len(), 2);
        assert_eq!(p.lignes()[0].quantite, quantite(5));
        assert_eq!(p.lignes()[1].article, tshirt());
    }

    #[test]
    fn retirer_des_articles() {
        let sku_livre = Sku::depuis("LIVRE-042").unwrap();
        let moins_un = panier().retirer(&sku_livre, quantite(1));
        assert_eq!(moins_un.lignes()[0].quantite, quantite(1));
        let sans_livre = panier().retirer(&sku_livre, quantite(9));
        assert_eq!(sans_livre.lignes().len(), 1);
        assert_eq!(sans_livre.lignes()[0].article, tshirt());
        let absent = Sku::depuis("ABSENT").unwrap();
        assert_eq!(panier().retirer(&absent, quantite(1)), panier());
    }

    #[test]
    fn sous_total_et_total_sans_code() {
        assert_eq!(panier().sous_total(), Centimes(5500));
        assert_eq!(panier().total(&[]), Centimes(5990));
        assert_eq!(Panier::default().total(&[]), Centimes(0));
    }

    #[test]
    fn codes_promo() {
        let dix_pourcent_livres =
            CodePromo::PourcentageSur(Categorie::Livres, Pourcentage::sur_cent(10).unwrap());
        assert_eq!(panier().total(&[dix_pourcent_livres]), Centimes(5590));
        assert_eq!(
            panier().total(&[CodePromo::MontantFixe(Centimes(1000))]),
            Centimes(4990)
        );
        assert_eq!(
            panier().total(&[CodePromo::LivraisonOfferte]),
            Centimes(5500)
        );
        let maison =
            CodePromo::PourcentageSur(Categorie::Maison, Pourcentage::sur_cent(50).unwrap());
        assert_eq!(panier().total(&[maison]), Centimes(5990));
    }

    #[test]
    fn la_remise_ne_rend_pas_le_total_negatif() {
        assert_eq!(
            panier().total(&[CodePromo::MontantFixe(Centimes(100_000))]),
            FRAIS_DE_PORT
        );
    }

    #[test]
    fn codes_cumules_dans_n_importe_quel_ordre() {
        let codes = [
            CodePromo::LivraisonOfferte,
            CodePromo::PourcentageSur(Categorie::Vetements, Pourcentage::sur_cent(20).unwrap()),
            CodePromo::MontantFixe(Centimes(500)),
        ];
        let inverses: Vec<CodePromo> = codes.iter().rev().cloned().collect();
        assert_eq!(panier().total(&codes), Centimes(4700));
        assert_eq!(panier().total(&inverses), Centimes(4700));
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e04_panier.rs"),
            &[
                "add",
                "sum",
                "fois",
                "nouvelle",
                "plus",
                "moins",
                "depuis",
                "ajouter",
                "retirer",
                "sous_total",
                "total",
                "appliquer",
            ],
        );
    }
}
