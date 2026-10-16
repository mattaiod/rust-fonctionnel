//! Exercice 4 : une connexion réseau, ses états et ses transitions.
//!
//! La version naïve a deux booléens, un compteur et une `Option` : rien n'empêche « connecté ET en cours », ni une
//! session alors qu'on est déconnecté. Dans le type somme, chaque donnée vit dans l'état où elle a un sens : le
//! numéro de tentative dans `EnConnexion`, la session dans `Connecte`. Une transition est une fonction pure
//! `(état, événement) → état` : on matche sur le **couple**, et rejouer une suite d'événements est un `fold`.

/// Nombre de tentatives avant d'abandonner.
pub const MAX_TENTATIVES: u32 = 3;

/// Version naïve. Ne pas modifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnexionNaive {
    pub connecte: bool,
    pub en_cours: bool,
    pub tentative: u32,
    pub session: Option<String>,
}

/// Les trois états d'une connexion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Connexion {
    Deconnecte,
    EnConnexion { tentative: u32 },
    Connecte { session: String },
}

/// Ce qui peut arriver à une connexion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Evenement {
    /// L'utilisateur demande à se connecter.
    Demarrer,
    /// La tentative en cours a échoué.
    Echec,
    /// La tentative en cours a réussi.
    Succes { session: String },
    /// On coupe, quel que soit l'état.
    Couper,
}

/// L'état après un événement.
///
/// 1. `Deconnecte` + `Demarrer` → `EnConnexion { tentative: 1 }`.
/// 2. `EnConnexion { t }` + `Echec` → `EnConnexion { t + 1 }`, ou `Deconnecte` si `t` vaut déjà `MAX_TENTATIVES`.
/// 3. `EnConnexion` + `Succes { session }` → `Connecte { session }`.
/// 4. N'importe quel état + `Couper` → `Deconnecte`.
/// 5. Tout le reste : l'état ne change pas.
///
/// **Aide Rust** (ch00, section 2)
/// - Le `match` sur le couple doit couvrir toutes les combinaisons : `` error[E0004]: non-exhaustive patterns:
///   `(Connexion::Connecte { .. }, Evenement::Demarrer)` … not covered `` → un dernier bras `(etat, _) => etat` (ici
///   `_` est un vrai « reste »).
/// - Les bras sont essayés dans l'ordre : `Couper` d'abord, et la garde de limite avant le cas général de `Echec`.
/// - État et événement sont reçus par valeur : `Succes { session }` déplace la `String` dans `Connecte { session }`,
///   sans `clone`.
pub fn reagir(connexion: Connexion, evenement: Evenement) -> Connexion {
    // But : un seul `match` sur le couple ; une garde `if` pour la limite de tentatives.
    todo!()
}

/// L'état final après une suite d'événements, en partant de `Deconnecte`.
///
/// **Aide Rust** (ch00, section 3)
/// - Si le compilateur dit `` error[E0631]: type mismatch in function arguments … expected function signature
///   `fn(Connexion, &Evenement) -> _` `` : `iter()` prête les événements, `reagir` les veut par valeur → `into_iter()`.
pub fn rejouer(evenements: Vec<Evenement>) -> Connexion {
    // But : une ligne.
    todo!()
}

/// L'état typé correspondant à un état naïf, `None` s'il est incohérent.
///
/// Cohérent : déconnecté (`!connecte && !en_cours`, tentative 0, pas de session) ; en connexion (`en_cours` seul,
/// tentative ≥ 1, pas de session) ; connecté (`connecte` seul, une session, tentative quelconque).
///
/// **Aide Rust** (ch00, section 2)
/// - Un motif de `struct` peut fixer des valeurs et en lier d'autres : `ConnexionNaive { connecte: false, session:
///   None, tentative, .. }`. Une garde `if …` se met après le motif.
/// - Si le compilateur dit `` error[E0027]: pattern does not mention fields `tentative`, `session` `` : nommez-les, ou
///   terminez le motif par `..`.
pub fn depuis_naive(naive: ConnexionNaive) -> Option<Connexion> {
    // But : un `match` sur les quatre champs, gardes comprises.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use Evenement::*;
    use pf_outils::style;

    fn succes(session: &str) -> Evenement {
        Succes {
            session: session.to_string(),
        }
    }

    #[test]
    fn transitions() {
        assert_eq!(
            reagir(Connexion::Deconnecte, Demarrer),
            Connexion::EnConnexion { tentative: 1 }
        );
        assert_eq!(
            reagir(Connexion::EnConnexion { tentative: 1 }, Echec),
            Connexion::EnConnexion { tentative: 2 }
        );
        assert_eq!(
            reagir(Connexion::EnConnexion { tentative: 3 }, Echec),
            Connexion::Deconnecte
        );
        assert_eq!(
            reagir(Connexion::EnConnexion { tentative: 2 }, succes("s1")),
            Connexion::Connecte {
                session: "s1".to_string()
            }
        );
    }

    #[test]
    fn evenements_sans_effet() {
        assert_eq!(reagir(Connexion::Deconnecte, Echec), Connexion::Deconnecte);
        assert_eq!(
            reagir(Connexion::Deconnecte, succes("x")),
            Connexion::Deconnecte
        );
        let connecte = Connexion::Connecte {
            session: "s1".to_string(),
        };
        assert_eq!(reagir(connecte.clone(), Demarrer), connecte);
        assert_eq!(reagir(connecte.clone(), succes("s2")), connecte);
        assert_eq!(reagir(connecte, Couper), Connexion::Deconnecte);
    }

    #[test]
    fn rejouer_une_histoire() {
        assert_eq!(rejouer(vec![]), Connexion::Deconnecte);
        assert_eq!(
            rejouer(vec![Demarrer, Echec, Echec, succes("ok")]),
            Connexion::Connecte {
                session: "ok".to_string()
            }
        );
        assert_eq!(
            rejouer(vec![Demarrer, Echec, Echec, Echec, succes("trop tard")]),
            Connexion::Deconnecte
        );
        assert_eq!(
            rejouer(vec![Demarrer, Echec, Echec, Echec, Demarrer]),
            Connexion::EnConnexion { tentative: 1 }
        );
    }

    #[test]
    fn le_naif_represente_des_etats_absurdes() {
        let absurde = ConnexionNaive {
            connecte: true,
            en_cours: true,
            tentative: 0,
            session: None,
        };
        assert_eq!(depuis_naive(absurde), None);
        let fantome = ConnexionNaive {
            connecte: false,
            en_cours: false,
            tentative: 0,
            session: Some("perdue".to_string()),
        };
        assert_eq!(depuis_naive(fantome), None);
        let correct = ConnexionNaive {
            connecte: false,
            en_cours: true,
            tentative: 2,
            session: None,
        };
        assert_eq!(
            depuis_naive(correct),
            Some(Connexion::EnConnexion { tentative: 2 })
        );
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e04_connexion.rs"),
            &["reagir", "rejouer", "depuis_naive"],
        );
    }
}
