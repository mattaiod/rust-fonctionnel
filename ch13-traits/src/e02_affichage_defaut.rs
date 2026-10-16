//! Exercice 2 : `Display` et `Default` : montrer et préremplir.
//!
//! `Display` remplace la fonction `formater_xxx` qu'on oublie d'appeler : on écrit `fmt` une fois, et `{}`,
//! `format!`, `to_string()` marchent partout. `Default` remplace le constructeur à six `Option` : on écrit les
//! valeurs par défaut une fois, et l'appelant ne change que ce qu'il veut avec `..Default::default()`.
//! Dans `fmt`, `{:02}` complète à deux chiffres avec des zéros.

use std::fmt;

/// Version naïve : une fonction à part, une `String` mutable. Ne pas modifier.
pub fn formater_duree_naif(secondes: u64) -> String {
    let heures = secondes / 3600;
    let minutes = (secondes % 3600) / 60;
    let reste = secondes % 60;
    let mut texte = String::new();
    if heures > 0 {
        texte.push_str(&format!("{heures} h {minutes:02} min "));
    } else if minutes > 0 {
        texte.push_str(&format!("{minutes} min "));
    }
    if heures > 0 || minutes > 0 {
        texte.push_str(&format!("{reste:02} s"));
    } else {
        texte.push_str(&format!("{reste} s"));
    }
    texte
}

/// Ordre d'une liste de résultats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tri {
    #[default]
    Recent,
    Ancien,
    Pertinence,
}

/// Réglages de pagination d'une liste.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pagination {
    pub page: u32,
    pub par_page: u32,
    pub tri: Tri,
}

/// Version naïve : chaque réglage est une `Option`, et les valeurs par défaut sont enfouies ici. Ne pas
/// modifier.
pub fn pagination_naive(page: Option<u32>, par_page: Option<u32>, tri: Option<Tri>) -> Pagination {
    Pagination {
        page: page.unwrap_or(1),
        par_page: par_page.unwrap_or(20),
        tri: tri.unwrap_or(Tri::Recent),
    }
}

/// Une durée, à la seconde.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Duree {
    pub secondes: u64,
}

impl fmt::Display for Duree {
    /// Le même texte que `formater_duree_naif` : `9 s`, `5 min 09 s`, `2 h 05 min 09 s`.
    ///
    /// **Aide Rust** (ch00, section 1)
    /// - `format!` fabrique une `String` : `E0308 … expected Result<(), Error>, found String`. Dans `fmt`, on écrit
    ///   avec `write!(f, ...)`.
    /// - Un `match` sur le tuple `(heures, minutes)` doit tout couvrir :
    ///   `E0004 non-exhaustive patterns: (1_u64..=u64::MAX, _) not covered`. Le dernier bras est un `_`.
    /// - `{m:02}` complète à deux chiffres ; `{s}` seul n'ajoute pas de zéro. Comparez avec `formater_duree_naif`
    ///   pour savoir quand mettre quoi.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // But : trois cas, un `write!` chacun.
        todo!()
    }
}

impl Default for Pagination {
    /// Page 1, 20 résultats par page, les plus récents d'abord.
    ///
    /// **Aide Rust** (ch00, section 1)
    /// - Tous les champs : `E0063 missing field tri in initializer of Pagination`. `Tri` a déjà son `Default`.
    /// - Piège : `..Default::default()` ici rappelle ce même `default` sans fin. Le compilateur prévient
    ///   (`warning: function cannot return without recursing`) ; à l'exécution, `stack overflow`.
    fn default() -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::style;

    #[test]
    fn cas_connus() {
        assert_eq!(Duree { secondes: 0 }.to_string(), "0 s");
        assert_eq!(Duree { secondes: 59 }.to_string(), "59 s");
        assert_eq!(Duree { secondes: 309 }.to_string(), "5 min 09 s");
        assert_eq!(Duree { secondes: 7509 }.to_string(), "2 h 05 min 09 s");
        assert_eq!(Duree { secondes: 3600 }.to_string(), "1 h 00 min 00 s");
        assert_eq!(format!("[{}]", Duree { secondes: 61 }), "[1 min 01 s]");
    }

    #[test]
    fn meme_texte_que_le_naif() {
        (0..20_000_u64)
            .step_by(7)
            .chain([86_399, 86_400, 1_000_000])
            .for_each(|s| assert_eq!(Duree { secondes: s }.to_string(), formater_duree_naif(s)));
    }

    #[test]
    fn valeurs_par_defaut() {
        assert_eq!(Pagination::default(), pagination_naive(None, None, None));
        assert_eq!(
            Pagination {
                page: 3,
                ..Default::default()
            },
            pagination_naive(Some(3), None, None)
        );
        assert_eq!(
            Pagination {
                tri: Tri::Pertinence,
                par_page: 50,
                ..Default::default()
            },
            pagination_naive(None, Some(50), Some(Tri::Pertinence))
        );
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(include_str!("e02_affichage_defaut.rs"), &["fmt", "default"]);
    }
}
