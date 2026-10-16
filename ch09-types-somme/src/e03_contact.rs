//! Exercice 3 : un moyen de contact : email, téléphone, ou les deux, jamais aucun.
//!
//! Deux `Option` font 2 × 2 = 4 états, dont un illégal (ni email ni téléphone). Le bon type en a 3. La règle
//! « au moins un » ne se vérifie plus partout où l'on lit un contact : elle se vérifie **une fois**, à la
//! construction (`depuis_options`), et ensuite elle est vraie par construction. Un `match` sur un couple
//! `(Option, Option)` traite les quatre cas d'un coup d'œil.

/// Version naïve : rien n'empêche un contact vide. Ne pas modifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContactNaif {
    pub email: Option<String>,
    pub telephone: Option<String>,
}

/// Version naïve du canal à utiliser : l'email d'abord. Panique sur un contact vide. Ne pas modifier.
pub fn canal_naif(contact: &ContactNaif) -> String {
    if let Some(email) = &contact.email {
        email.clone()
    } else if let Some(telephone) = &contact.telephone {
        telephone.clone()
    } else {
        panic!("contact sans aucun moyen de le joindre")
    }
}

/// Un moyen de contact : jamais vide.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Contact {
    Email(String),
    Telephone(String),
    LesDeux { email: String, telephone: String },
}

impl Contact {
    /// Le contact correspondant aux deux champs d'un formulaire, `None` s'ils sont tous deux absents.
    ///
    /// **Aide Rust** (ch00, section 2)
    /// - `match (email, telephone)` reçoit les deux `Option` par valeur : les `String` se déplacent dans les variantes,
    ///   aucun `clone` nécessaire.
    pub fn depuis_options(email: Option<String>, telephone: Option<String>) -> Option<Contact> {
        // But : un `match` sur le couple, quatre bras.
        todo!()
    }

    /// L'email, s'il y en a un.
    ///
    /// **Aide Rust** (ch00, sections 5, 11)
    /// - Si le compilateur dit `` error[E0027]: pattern does not mention field `telephone` `` : dans `LesDeux { email,
    ///   .. }`, le `..` ignore les autres champs.
    /// - Dans un motif `A(x) | B { x, .. }`, chaque côté doit lier les mêmes noms, sinon `` error[E0408]: variable `…`
    ///   is not bound in all patterns ``.
    /// - Ici `email` est un `&String`, et `Some(email)` devient tout seul une `Option<&str>`. `Some(email.clone())`
    ///   donnerait une `Option<String>` : `` expected `&str`, found `String` ``.
    pub fn email(&self) -> Option<&str> {
        // But : un motif `|` regroupe les variantes qui portent un email.
        todo!()
    }

    /// Le canal à utiliser : l'email s'il existe, le téléphone sinon. Jamais d'échec possible.
    ///
    /// **Aide Rust** (ch00, section 2)
    /// - Si le compilateur dit `` error[E0507]: cannot move out of `self` as enum variant `Email` which is behind a
    ///   shared reference `` : `match *self` essaie de sortir la `String`. `match self` lie des références, et c'est
    ///   une référence qu'on rend.
    pub fn canal(&self) -> &str {
        // But : le même ordre de préférence que la version naïve, sans `Option` ni `panic!`.
        todo!()
    }

    /// Ajoute un téléphone (le remplace s'il y en avait déjà un).
    ///
    /// **Aide Rust** (ch00, sections 2, 6)
    /// - `self` sans `&` : la méthode consomme le contact, on peut donc déplacer `email` dans le nouveau, sans `clone`.
    ///   Avec `&self`, ce serait `` expected `String`, found `&String` ``.
    pub fn avec_telephone(self, nouveau: String) -> Contact {
        // But : consommer l'ancien contact et en rendre un nouveau.
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::style;

    fn s(texte: &str) -> String {
        texte.to_string()
    }

    #[test]
    fn construction() {
        assert_eq!(Contact::depuis_options(None, None), None);
        assert_eq!(
            Contact::depuis_options(Some(s("a@b.fr")), None),
            Some(Contact::Email(s("a@b.fr")))
        );
        assert_eq!(
            Contact::depuis_options(None, Some(s("0601"))),
            Some(Contact::Telephone(s("0601")))
        );
        assert_eq!(
            Contact::depuis_options(Some(s("a@b.fr")), Some(s("0601"))),
            Some(Contact::LesDeux {
                email: s("a@b.fr"),
                telephone: s("0601")
            })
        );
    }

    #[test]
    fn lecture() {
        let deux = Contact::LesDeux {
            email: s("a@b.fr"),
            telephone: s("0601"),
        };
        assert_eq!(deux.email(), Some("a@b.fr"));
        assert_eq!(Contact::Telephone(s("0601")).email(), None);
        assert_eq!(Contact::Telephone(s("0601")).canal(), "0601");
        assert_eq!(deux.canal(), "a@b.fr");
    }

    #[test]
    fn ajouter_un_telephone() {
        assert_eq!(
            Contact::Email(s("a@b.fr")).avec_telephone(s("0602")),
            Contact::LesDeux {
                email: s("a@b.fr"),
                telephone: s("0602")
            }
        );
        assert_eq!(
            Contact::Telephone(s("0601")).avec_telephone(s("0602")),
            Contact::Telephone(s("0602"))
        );
    }

    #[test]
    fn meme_canal_que_le_naif_sur_les_contacts_valides() {
        let options = [None, Some(s("a@b.fr"))];
        let telephones = [None, Some(s("0601"))];
        options.iter().for_each(|email| {
            telephones.iter().for_each(|telephone| {
                let naif = ContactNaif {
                    email: email.clone(),
                    telephone: telephone.clone(),
                };
                match Contact::depuis_options(email.clone(), telephone.clone()) {
                    Some(contact) => assert_eq!(contact.canal(), canal_naif(&naif)),
                    None => assert!(email.is_none() && telephone.is_none()),
                }
            })
        });
    }

    #[test]
    #[should_panic(expected = "sans aucun moyen")]
    fn le_naif_accepte_un_contact_vide() {
        canal_naif(&ContactNaif {
            email: None,
            telephone: None,
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e03_contact.rs"),
            &["depuis_options", "email", "canal", "avec_telephone"],
        );
    }
}
