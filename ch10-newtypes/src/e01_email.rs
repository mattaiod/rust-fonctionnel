//! Exercice 1 : `Email` : valider ou parser ?
//!
//! La version naïve **valide** : une fonction rend un `bool`, puis le reste du programme manipule une `&str` en
//! espérant qu'on a bien vérifié avant. La version typée **parse** : `Email::try_from` rend un `Email` ou une
//! erreur, et un `Email` qui existe est valide, pour toujours. Le champ est privé : hors de ce module, impossible
//! d'écrire `Email(n_importe_quoi)`, il faut passer par `try_from` (ou `parse`, grâce à `FromStr`).
//!
//! ```compile_fail,E0603
//! use ch10_newtypes::e01_email::Email;
//! let faux = Email("pas un email".to_string()); // E0603 : constructeur privé
//! ```

use std::fmt;
use std::str::FromStr;

/// Version naïve : la validation. Ne pas modifier.
pub fn est_email_valide_naif(texte: &str) -> bool {
    let texte = texte.trim();
    if texte.is_empty() || texte.contains(char::is_whitespace) {
        return false;
    }
    let mut arobases = 0;
    for c in texte.chars() {
        if c == '@' {
            arobases += 1;
        }
    }
    if arobases != 1 {
        return false;
    }
    let (local, domaine) = texte.split_once('@').unwrap();
    !local.is_empty() && domaine.contains('.') && domaine.split('.').all(|p| !p.is_empty())
}

/// Version naïve : le domaine d'un email... supposé déjà validé. Panique sinon. Ne pas modifier.
pub fn domaine_naif(email: &str) -> String {
    email.split('@').nth(1).unwrap().to_string()
}

/// Ce qui ne va pas dans un email.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErreurEmail {
    Vide,
    ContientUnEspace,
    ArobaseManquante,
    PlusieursArobases,
    PartieLocaleVide,
    /// Le domaine doit avoir au moins un point, et aucun morceau vide (`a@.fr`, `a@b.`, `a@b..fr` sont refusés).
    DomaineInvalide,
}

/// Une adresse email, vérifiée à la construction. Le champ est privé.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl TryFrom<&str> for Email {
    type Error = ErreurEmail;

    /// Les espaces autour sont retirés ; les erreurs sont testées dans l'ordre de l'enum.
    ///
    /// **Aide Rust** (ch00, sections 1, 5)
    /// - Une règle = un `if` qui sort tôt avec `return Err(...);`. Sans `return`, le compilateur dit
    ///   `E0308 mismatched types … if expressions without else arms expect their inner expression to be ()`.
    /// - Les variantes se nomment avec leur enum : `ErreurEmail::Vide`. `Vide` seul donne
    ///   `E0425 cannot find value Vide in this scope`.
    /// - `Email(texte)` avec `texte: &str` donne `E0308 … expected String, found &str` : le champ possède sa chaîne,
    ///   fabriquez la `String` (`to_string()`) seulement dans le `Ok` final.
    fn try_from(texte: &str) -> Result<Self, Self::Error> {
        // But : un retour anticipé par règle, ou un `match` sur `split_once('@')` avec des gardes.
        todo!()
    }
}

impl FromStr for Email {
    type Err = ErreurEmail;

    /// **Aide Rust** (ch00, section 1)
    /// - `try_from` rend déjà le bon `Result` : si le compilateur dit
    ///   `E0308 ? operator has incompatible types … found Email`, retirez le `?`.
    /// - `Email::from(texte)` n'existe pas (seul `TryFrom` est écrit) : `E0308 … expected Email, found &str`.
    fn from_str(texte: &str) -> Result<Self, Self::Err> {
        // But : ne pas réécrire la règle.
        todo!()
    }
}

impl fmt::Display for Email {
    /// **Aide Rust** (ch00, section 1)
    /// - `fmt` ne rend pas une chaîne : il écrit dans `f` (`write!(f, ...)` ou `f.write_str(...)`) et rend un
    ///   `fmt::Result`. `self.0` seul donne `E0308 … expected Result<(), Error>, found String` ; `println!` donne
    ///   `… found ()`.
    /// - Pas de `;` après le `write!` : avec, le compilateur dit
    ///   `E0308 … implicitly returns () as its body has no tail or return expression`.
    /// - Piège qui compile : `write!(f, "{}", self)` rappelle ce même `fmt` sans fin, et le test meurt sur
    ///   `has overflowed its stack`. Affichez le champ, pas `self`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // But : afficher l'adresse telle quelle.
        todo!()
    }
}

impl Email {
    /// Le texte de l'adresse.
    ///
    /// **Aide Rust** (ch00, sections 2, 5)
    /// - `self.0` est une `String` qui appartient à `self` : on ne la rend pas, on la prête.
    ///   `E0308 … expected &str, found String` propose lui-même le remède (`consider borrowing here`).
    pub fn as_str(&self) -> &str {
        // But : un accès en lecture seule au champ privé.
        todo!()
    }

    /// Le domaine (après l'arobase). Ne peut pas échouer : l'invariant le garantit.
    ///
    /// **Aide Rust** (ch00, sections 5, 11)
    /// - `split_once` rend une `Option` : `E0308 … expected &str, found Option<&str>`. L'invariant garantit le
    ///   `Some` : ici, `expect("invariant d'Email : ...")` avec la raison est légitime.
    /// - Rendez un morceau de `self.0` (une `&str`), pas une copie : `.to_string()` donne
    ///   `E0308 … expected &str, found String`.
    pub fn domaine(&self) -> &str {
        // But : pas d'`Option` dans le type de retour ; l'invariant suffit.
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::style;

    const ENTREES: [&str; 14] = [
        "alice@exemple.fr",
        "  bob@mail.co.uk ",
        "",
        "   ",
        "a b@c.fr",
        "alice.exemple.fr",
        "a@b@c.fr",
        "@exemple.fr",
        "alice@exemple",
        "alice@.fr",
        "alice@exemple.",
        "alice@exemple..fr",
        "x@y.z",
        "prénom.nom@école.fr",
    ];

    #[test]
    fn cas_connus() {
        let email = Email::try_from(" alice@exemple.fr ").unwrap();
        assert_eq!(email.as_str(), "alice@exemple.fr");
        assert_eq!(email.domaine(), "exemple.fr");
        assert_eq!(email.to_string(), "alice@exemple.fr");
        assert_eq!(
            "bob@mail.co.uk".parse::<Email>().unwrap().domaine(),
            "mail.co.uk"
        );
    }

    #[test]
    fn chaque_erreur_a_son_nom() {
        assert_eq!(Email::try_from(""), Err(ErreurEmail::Vide));
        assert_eq!(
            Email::try_from("a b@c.fr"),
            Err(ErreurEmail::ContientUnEspace)
        );
        assert_eq!(
            Email::try_from("alice.fr"),
            Err(ErreurEmail::ArobaseManquante)
        );
        assert_eq!(
            Email::try_from("a@b@c.fr"),
            Err(ErreurEmail::PlusieursArobases)
        );
        assert_eq!(Email::try_from("@c.fr"), Err(ErreurEmail::PartieLocaleVide));
        assert_eq!(Email::try_from("a@c"), Err(ErreurEmail::DomaineInvalide));
        assert_eq!(
            Email::try_from("a@c..fr"),
            Err(ErreurEmail::DomaineInvalide)
        );
        assert_eq!("a@".parse::<Email>(), Err(ErreurEmail::DomaineInvalide));
    }

    #[test]
    fn meme_verdict_que_le_naif() {
        ENTREES.iter().for_each(|texte| {
            assert_eq!(
                Email::try_from(*texte).is_ok(),
                est_email_valide_naif(texte),
                "{texte:?}"
            );
        });
    }

    #[test]
    fn meme_domaine_que_le_naif_sur_les_emails_valides() {
        ENTREES
            .iter()
            .filter_map(|texte| Email::try_from(*texte).ok())
            .for_each(|email| assert_eq!(email.domaine(), domaine_naif(email.as_str())));
    }

    #[test]
    #[should_panic]
    fn le_naif_fait_confiance() {
        domaine_naif("pas un email");
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e01_email.rs"),
            &["try_from", "from_str", "fmt", "as_str", "domaine"],
        );
    }
}
