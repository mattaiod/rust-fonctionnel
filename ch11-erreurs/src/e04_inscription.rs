//! Exercice 4 : un formulaire qui rend TOUTES ses erreurs.
//!
//! `?` s'arrête à la première erreur : parfait pour une suite d'étapes dépendantes, frustrant pour un
//! formulaire, où l'utilisateur veut voir tous ses champs en rouge d'un coup. Les champs sont indépendants :
//! on les valide tous, puis on regarde le tuple des trois `Result`. Tous `Ok` : on construit. Sinon, on garde
//! les `Err` (`Result::err` rend une `Option`, et un itérateur d'`Option` s'aplatit avec `flatten`).

/// Une inscription validée : si elle existe, ses trois champs sont bons.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inscription {
    pub pseudo: String,
    pub email: String,
    pub age: u8,
}

/// Une erreur sur un champ du formulaire.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurChamp {
    /// Le pseudo doit faire de 3 à 16 caractères ; on garde la longueur reçue.
    PseudoLongueur(usize),
    /// Seuls les lettres et chiffres ASCII et `_` sont permis ; on garde le premier caractère fautif.
    PseudoCaractere(char),
    /// Pas exactement un `@`, partie locale vide, ou domaine sans point (ou avec un morceau vide).
    EmailInvalide(String),
    /// L'âge n'est pas un entier entre 0 et 255.
    AgeIllisible(String),
    /// Moins de 15 ans.
    AgeTropJeune(u8),
}

/// Âge minimal pour s'inscrire.
pub const AGE_MINIMAL: u8 = 15;

/// Version naïve : s'arrête à la première erreur et la rend en texte. L'utilisateur corrige le pseudo, renvoie,
/// découvre l'email, renvoie, découvre l'âge. Ne pas modifier.
pub fn inscrire_naif(pseudo: &str, email: &str, age: &str) -> Result<Inscription, String> {
    let longueur = pseudo.chars().count();
    if !(3..=16).contains(&longueur) {
        return Err(format!("pseudo de {longueur} caractères"));
    }
    for c in pseudo.chars() {
        if !(c.is_ascii_alphanumeric() || c == '_') {
            return Err(format!("caractère interdit : {c}"));
        }
    }
    let morceaux: Vec<&str> = email.split('@').collect();
    if morceaux.len() != 2 || morceaux[0].is_empty() {
        return Err(format!("email invalide : {email}"));
    }
    let domaine: Vec<&str> = morceaux[1].split('.').collect();
    if domaine.len() < 2 || domaine.iter().any(|m| m.is_empty()) {
        return Err(format!("email invalide : {email}"));
    }
    let age: u8 = match age.trim().parse() {
        Ok(a) => a,
        Err(_) => return Err(format!("âge illisible : {age}")),
    };
    if age < AGE_MINIMAL {
        return Err(format!("trop jeune : {age} ans"));
    }
    Ok(Inscription {
        pseudo: pseudo.to_string(),
        email: email.to_string(),
        age,
    })
}

/// Le pseudo, s'il est valide.
///
/// **Aide Rust** (ch00, sections 4, 11)
/// - Dans `find(|c| ...)`, `c` est un `&char` : `c == '_'` donne `E0277 can't compare &char with char`. Écrivez
///   `*c == '_'`.
/// - `find` rend une `Option<char>`, la fonction un `Result` :
///   `E0308 … expected Result<String, ErreurChamp>, found Option<Result<_, ErreurChamp>>`. `map_or` (ou un `match`)
///   décide des deux cas d'un coup.
/// - La longueur se compte en caractères (`chars().count()`), pas en octets.
pub fn valider_pseudo(pseudo: &str) -> Result<String, ErreurChamp> {
    // But : d'abord la longueur (en caractères), puis le premier caractère interdit.
    todo!()
}

/// L'email, s'il est valide (mêmes règles que la version naïve).
///
/// **Aide Rust** (ch00, section 5)
/// - `EmailInvalide(email)` donne `E0308 … expected String, found &str` : la variante possède son texte.
/// - `split('@')` sur `a@b@c.fr` donne trois morceaux : vérifiez qu'il y a exactement un `@`, pas seulement qu'il y
///   en a un.
pub fn valider_email(email: &str) -> Result<String, ErreurChamp> {
    // But : un seul `@`, une partie locale non vide, un domaine en au moins deux morceaux non vides.
    todo!()
}

/// L'âge, s'il est lisible (blancs autour tolérés) et d'au moins `AGE_MINIMAL` ans.
///
/// **Aide Rust** (ch00, sections 9, 12)
/// - `map_err(ErreurChamp::AgeIllisible)` donne `E0631 type mismatch in function arguments` : la variante attend une
///   `String`, `map_err` lui passe une `ParseIntError`. Écrivez la closure `|_| ...` qui fabrique la variante avec le
///   texte reçu.
pub fn valider_age(age: &str) -> Result<u8, ErreurChamp> {
    // But : lisible d'abord, assez âgé ensuite.
    todo!()
}

/// L'inscription, ou toutes les erreurs, dans l'ordre des champs (pseudo, email, âge).
///
/// **Aide Rust** (ch00, sections 6, 11)
/// - `?` s'arrêterait à la première erreur, et le compilateur refuse de toute façon :
///   `E0277 ? couldn't convert the error to Vec<ErreurChamp>`.
/// - Ne lisez pas les trois `Result` après le `match` qui les a consommés : `E0382 use of moved value`. Le bras
///   attrape-tout reçoit les trois `Result` en nommant ses variables : `(p, e, a) => ...`.
/// - `[p.err(), e.err(), a.err()]` contient des `Option` : `collect` seul donne
///   `E0277 a value of type Vec<ErreurChamp> cannot be built from an iterator over elements of type Option<ErreurChamp>`.
///   Aplatissez-les d'abord (cours de l'exercice).
pub fn inscrire(pseudo: &str, email: &str, age: &str) -> Result<Inscription, Vec<ErreurChamp>> {
    // But : valider les trois champs, puis `match` sur le tuple des trois résultats.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::style;

    #[test]
    fn inscription_valide() {
        assert_eq!(
            inscrire("lea_42", "lea@exemple.fr", " 19 "),
            Ok(Inscription {
                pseudo: "lea_42".to_string(),
                email: "lea@exemple.fr".to_string(),
                age: 19
            })
        );
    }

    #[test]
    fn rend_toutes_les_erreurs() {
        assert_eq!(
            inscrire("é", "pas-un-email", "12"),
            Err(vec![
                ErreurChamp::PseudoLongueur(1),
                ErreurChamp::EmailInvalide("pas-un-email".to_string()),
                ErreurChamp::AgeTropJeune(12),
            ])
        );
        assert_eq!(
            inscrire("léa", "lea@exemple.fr", "vingt"),
            Err(vec![
                ErreurChamp::PseudoCaractere('é'),
                ErreurChamp::AgeIllisible("vingt".to_string()),
            ])
        );
    }

    #[test]
    fn regles_de_l_email() {
        ["a@b.fr", "a.b@c.d.e"]
            .iter()
            .for_each(|e| assert!(valider_email(e).is_ok(), "{e}"));
        [
            "", "@b.fr", "a@b", "a@b.", "a@.fr", "a@@b.fr", "a@b@c.fr", "a@b..fr",
        ]
        .iter()
        .for_each(|e| assert!(valider_email(e).is_err(), "{e}"));
    }

    #[test]
    fn meme_acceptation_que_le_naif() {
        let pseudos = [
            "lea_42",
            "lu",
            "un_pseudo_beaucoup_trop_long",
            "a b",
            "élève",
            "Bob",
        ];
        let emails = ["lea@exemple.fr", "x", "a@b", "@b.fr", "a@b@c.fr", "a@b.c"];
        let ages = ["19", "15", "14", "-1", "300", " 40 ", "", "vingt"];
        pseudos.iter().for_each(|p| {
            emails.iter().for_each(|e| {
                ages.iter().for_each(|a| {
                    assert_eq!(
                        inscrire(p, e, a).ok(),
                        inscrire_naif(p, e, a).ok(),
                        "{p} {e} {a}"
                    )
                })
            })
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e04_inscription.rs"),
            &["valider_pseudo", "valider_email", "valider_age", "inscrire"],
        );
    }
}
