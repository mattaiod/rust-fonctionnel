//! Exercice 4 : `IdUtilisateur` et `IdCommande` : deux `u64` qu'on ne doit plus pouvoir confondre.
//!
//! La version naïve prend deux `u64` côte à côte. Inversez-les à l'appel : ça compile, et selon les données, ça rend
//! la commande de quelqu'un d'autre (le test `le_bug_de_l_inversion` le montre). Un newtype par concept, et
//! l'inversion devient une erreur de compilation. Coût à l'exécution : zéro, un `IdCommande` a la taille d'un `u64`.
//!
//! ```compile_fail,E0308
//! use ch10_newtypes::e04_identifiants::{trouver_commande, IdCommande, IdUtilisateur};
//! // Arguments inversés : E0308, mismatched types.
//! let trouvee = trouver_commande(&[], IdCommande::new(7), IdUtilisateur::new(3));
//! ```
//!
//! ```compile_fail,E0308
//! use ch10_newtypes::e04_identifiants::{IdCommande, IdUtilisateur};
//! // Comparer un utilisateur et une commande n'a pas de sens : E0308.
//! let absurde = IdUtilisateur::new(3) == IdCommande::new(3);
//! ```

/// Version naïve d'une commande. Ne pas modifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandeNaive {
    pub id: u64,
    pub client: u64,
    pub montant_centimes: u64,
}

/// Version naïve : la commande `id_commande` si elle appartient bien à `id_client`. Ne pas modifier.
pub fn trouver_commande_naif(
    commandes: &[CommandeNaive],
    id_client: u64,
    id_commande: u64,
) -> Option<&CommandeNaive> {
    commandes
        .iter()
        .find(|c| c.id == id_commande && c.client == id_client)
}

/// L'identifiant d'un utilisateur.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IdUtilisateur(u64);

/// L'identifiant d'une commande.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IdCommande(u64);

impl IdUtilisateur {
    /// Tout `u64` est un identifiant possible : le constructeur ne peut pas échouer.
    pub fn new(brut: u64) -> Self {
        IdUtilisateur(brut)
    }
}

impl IdCommande {
    /// Tout `u64` est un identifiant possible : le constructeur ne peut pas échouer.
    pub fn new(brut: u64) -> Self {
        IdCommande(brut)
    }
}

/// Une commande, aux identifiants typés.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Commande {
    pub id: IdCommande,
    pub client: IdUtilisateur,
    pub montant_centimes: u64,
}

/// La commande `id_commande` si elle appartient bien à `id_client`.
///
/// **Aide Rust** (ch00, sections 1, 4)
/// - Si vous inversez les deux identifiants, le compilateur dit
///   `E0308 mismatched types … expected IdCommande, found IdUtilisateur` : c'est exactement le bug que l'exercice
///   rend impossible.
pub fn trouver_commande(
    commandes: &[Commande],
    id_client: IdUtilisateur,
    id_commande: IdCommande,
) -> Option<&Commande> {
    // But : la même recherche ; le compilateur vérifie maintenant ce que vous comparez.
    todo!()
}

/// Le total dépensé par un client, en centimes.
///
/// **Aide Rust** (ch00, section 3)
/// - On ne somme pas des commandes :
///   `E0277 a value of type u64 cannot be made by summing an iterator over elements of type &Commande`. Passez
///   d'abord des commandes aux montants.
pub fn total_depense(commandes: &[Commande], client: IdUtilisateur) -> u64 {
    // But : chapitre 3.
    todo!()
}

/// Les identifiants des commandes d'un client, dans l'ordre.
///
/// **Aide Rust** (ch00, sections 2, 3)
/// - `map(|c| &c.id)` donne
///   `E0277 a value of type Vec<IdCommande> cannot be built from an iterator over elements of type &IdCommande`.
///   `IdCommande` est `Copy` : `c.id` suffit, sans `&`.
pub fn commandes_de(commandes: &[Commande], client: IdUtilisateur) -> Vec<IdCommande> {
    // But : chapitre 3 ; le type de retour dit ce que contient la liste.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::style;

    fn naives() -> Vec<CommandeNaive> {
        vec![
            CommandeNaive {
                id: 7,
                client: 3,
                montant_centimes: 4_200,
            },
            CommandeNaive {
                id: 3,
                client: 7,
                montant_centimes: 99_900,
            },
            CommandeNaive {
                id: 8,
                client: 3,
                montant_centimes: 1_000,
            },
        ]
    }

    fn typees() -> Vec<Commande> {
        naives()
            .into_iter()
            .map(|c| Commande {
                id: IdCommande::new(c.id),
                client: IdUtilisateur::new(c.client),
                montant_centimes: c.montant_centimes,
            })
            .collect()
    }

    #[test]
    fn cas_connus() {
        let commandes = typees();
        let trouvee = trouver_commande(&commandes, IdUtilisateur::new(3), IdCommande::new(7));
        assert_eq!(trouvee.map(|c| c.montant_centimes), Some(4_200));
        assert_eq!(
            trouver_commande(&commandes, IdUtilisateur::new(3), IdCommande::new(3)),
            None
        );
        assert_eq!(total_depense(&commandes, IdUtilisateur::new(3)), 5_200);
        assert_eq!(total_depense(&commandes, IdUtilisateur::new(42)), 0);
        assert_eq!(
            commandes_de(&commandes, IdUtilisateur::new(3)),
            vec![IdCommande::new(7), IdCommande::new(8)]
        );
    }

    #[test]
    fn le_bug_de_l_inversion() {
        let commandes = naives();
        let client = 3;
        let commande = 7;
        // Bon ordre : la commande 7 du client 3.
        let bonne = trouver_commande_naif(&commandes, client, commande);
        assert_eq!(bonne.map(|c| c.montant_centimes), Some(4_200));
        // Arguments inversés : ça compile, et on rend la commande 3... du client 7. Fuite de données.
        let fuite = trouver_commande_naif(&commandes, commande, client);
        assert_eq!(fuite.map(|c| c.client), Some(7));
        // Version typée : la même inversion ne compile pas (voir les doc-tests `compile_fail` du module).
    }

    #[test]
    fn meme_resultat_que_le_naif_dans_le_bon_ordre() {
        let (naives, typees) = (naives(), typees());
        (0..10).for_each(|client| {
            (0..10).for_each(|commande| {
                let attendu =
                    trouver_commande_naif(&naives, client, commande).map(|c| c.montant_centimes);
                let obtenu = trouver_commande(
                    &typees,
                    IdUtilisateur::new(client),
                    IdCommande::new(commande),
                )
                .map(|c| c.montant_centimes);
                assert_eq!(obtenu, attendu, "client {client}, commande {commande}");
            })
        });
    }

    #[test]
    fn zero_cout() {
        assert_eq!(
            std::mem::size_of::<IdCommande>(),
            std::mem::size_of::<u64>()
        );
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e04_identifiants.rs"),
            &["trouver_commande", "total_depense", "commandes_de"],
        );
    }
}
