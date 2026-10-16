//! Exercice 4 : L'email du parrain, sans pyramide de `if let`.
//!
//! Quatre étapes qui peuvent chacune ne rien donner : l'utilisateur, son parrain, la fiche du parrain, son email.
//! Avec des `if let` imbriqués, le code part vers la droite. Avec `and_then`, il reste à plat : chaque étape
//! reçoit la valeur de la précédente et rend une `Option`. L'opérateur `?` fait la même chose, en plus lisible
//! quand les étapes se multiplient. `ok_or` transforme une absence muette en erreur qui dit laquelle.

use std::collections::HashMap;

/// Une fiche de l'annuaire. Le parrain est désigné par son nom.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Utilisateur {
    pub email: Option<String>,
    pub parrain: Option<String>,
}

/// L'annuaire : nom → fiche. `type` crée un alias, un autre nom pour le même type : `Annuaire` et
/// `HashMap<String, Utilisateur>` sont interchangeables.
pub type Annuaire = HashMap<String, Utilisateur>;

/// Version naïve : `""` pour « pas trouvé », quelle que soit la raison. Ne pas modifier.
#[allow(clippy::collapsible_if)] // la pyramide est le sujet de l'exercice
pub fn email_du_parrain_naif(annuaire: &Annuaire, nom: &str) -> String {
    if let Some(utilisateur) = annuaire.get(nom) {
        if let Some(parrain) = &utilisateur.parrain {
            if let Some(fiche) = annuaire.get(parrain) {
                if let Some(email) = &fiche.email {
                    return email.clone();
                }
            }
        }
    }
    String::new()
}

/// L'email du parrain de `nom`, en une chaîne de `and_then`.
///
/// **Aide Rust** (ch00, sections 5, 11, 13)
/// - La fiche est empruntée à l'annuaire : `` error[E0507]: cannot move out of `u.parrain` which is behind a shared
///   reference `` veut dire qu'on ne peut pas en sortir la `String`. `as_deref()` donne une `Option<&str>` qui
///   emprunte.
/// - Si le compilateur dit `` expected `Option<&str>`, found `Option<&String>` `` : `as_ref()` donne `Option<&String>`,
///   `as_deref()` donne `Option<&str>`.
/// - Une étape qui rend une `Option`, c'est `and_then` ; avec `map`, on obtient une `Option<Option<…>>` qui ne
///   s'enchaîne plus.
/// - Le `'a` de la signature dit que l'email rendu est emprunté à l'annuaire, pas à `nom`. Sans lui : `error[E0106]:
///   missing lifetime specifier`.
pub fn email_du_parrain<'a>(annuaire: &'a Annuaire, nom: &str) -> Option<&'a str> {
    // But : les quatre étapes à plat, sans `if`, sans `match`, sans `?`.
    todo!()
}

/// La même chose, avec l'opérateur `?`.
///
/// **Aide Rust** (ch00, section 11)
/// - `?` sur une `Option` : si elle est vide, la fonction rend `None` tout de suite. Permis ici parce que la fonction
///   rend elle-même une `Option`. Les `as_deref()` de la version précédente restent nécessaires.
pub fn email_du_parrain_avec_q<'a>(annuaire: &'a Annuaire, nom: &str) -> Option<&'a str> {
    // But : une ligne par étape, chacune terminée par `?`.
    todo!()
}

/// Pourquoi on n'a pas trouvé d'email.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Introuvable {
    /// `nom` n'est pas dans l'annuaire.
    Utilisateur,
    /// L'utilisateur n'a pas de parrain.
    SansParrain,
    /// Le parrain cité n'a pas de fiche.
    ParrainInconnu,
    /// Le parrain n'a pas d'email.
    SansEmail,
}

/// La même recherche, mais l'échec dit à quelle étape il a eu lieu.
///
/// **Aide Rust** (ch00, section 11)
/// - Si le compilateur dit `` error[E0277]: the `?` operator can only be used on `Result`s, not `Option`s, in a
///   function that returns `Result` `` : chaque `Option` passe par `ok_or(Introuvable::…)` avant son `?`.
pub fn email_du_parrain_explique<'a>(
    annuaire: &'a Annuaire,
    nom: &str,
) -> Result<&'a str, Introuvable> {
    // But : chaque absence devient l'erreur qui la nomme.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::style;

    fn fiche(email: Option<&str>, parrain: Option<&str>) -> Utilisateur {
        Utilisateur {
            email: email.map(String::from),
            parrain: parrain.map(String::from),
        }
    }

    /// alice parraine bob ; bob parraine chloe ; dan est parrainé par un inconnu ; eve par chloe, qui n'a pas d'email.
    fn annuaire() -> Annuaire {
        HashMap::from([
            ("alice".to_string(), fiche(Some("alice@exemple.fr"), None)),
            (
                "bob".to_string(),
                fiche(Some("bob@exemple.fr"), Some("alice")),
            ),
            ("chloe".to_string(), fiche(None, Some("bob"))),
            ("dan".to_string(), fiche(None, Some("fantome"))),
            ("eve".to_string(), fiche(None, Some("chloe"))),
        ])
    }

    #[test]
    fn cas_connus() {
        let a = annuaire();
        assert_eq!(email_du_parrain(&a, "bob"), Some("alice@exemple.fr"));
        assert_eq!(email_du_parrain(&a, "chloe"), Some("bob@exemple.fr"));
        assert_eq!(email_du_parrain(&a, "alice"), None);
        assert_eq!(email_du_parrain(&a, "dan"), None);
        assert_eq!(email_du_parrain(&a, "eve"), None);
        assert_eq!(email_du_parrain(&a, "zoe"), None);
    }

    #[test]
    fn les_trois_versions_sont_d_accord() {
        let a = annuaire();
        ["alice", "bob", "chloe", "dan", "eve", "zoe"]
            .iter()
            .for_each(|nom| {
                let attendu = email_du_parrain_naif(&a, nom);
                let attendu = (!attendu.is_empty()).then_some(attendu.as_str());
                assert_eq!(email_du_parrain(&a, nom), attendu, "{nom}");
                assert_eq!(email_du_parrain_avec_q(&a, nom), attendu, "{nom}");
                assert_eq!(email_du_parrain_explique(&a, nom).ok(), attendu, "{nom}");
            });
    }

    #[test]
    fn l_erreur_dit_l_etape() {
        let a = annuaire();
        assert_eq!(email_du_parrain_explique(&a, "bob"), Ok("alice@exemple.fr"));
        assert_eq!(
            email_du_parrain_explique(&a, "zoe"),
            Err(Introuvable::Utilisateur)
        );
        assert_eq!(
            email_du_parrain_explique(&a, "alice"),
            Err(Introuvable::SansParrain)
        );
        assert_eq!(
            email_du_parrain_explique(&a, "dan"),
            Err(Introuvable::ParrainInconnu)
        );
        assert_eq!(
            email_du_parrain_explique(&a, "eve"),
            Err(Introuvable::SansEmail)
        );
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e04_parrain.rs"),
            &[
                "email_du_parrain",
                "email_du_parrain_avec_q",
                "email_du_parrain_explique",
            ],
        );
    }
}
