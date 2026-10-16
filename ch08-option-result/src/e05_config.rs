//! Exercice 5 : une configuration avec valeurs par défaut, sans version naïve : à vous.
//!
//! Deux cas à ne pas confondre : une clé **absente** prend sa valeur par défaut ; une clé **présente mais
//! illisible** (`port = "huit"`) est une erreur, qu'on ne remplace surtout pas en silence par le défaut. Le premier
//! se traite avec `unwrap_or` et ses cousins, le second avec `Result` et `?`.
//!
//! Piège : `unwrap_or(x)` évalue `x` tout de suite, même si l'option est pleine. Pour une valeur à construire
//! (une `String`), `unwrap_or_else(|| …)` ne la construit que si besoin.

use std::collections::HashMap;
use std::str::FromStr;

/// La configuration lue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    /// Défaut : `"localhost"`.
    pub hote: String,
    /// Défaut : `8080`.
    pub port: u16,
    /// Défaut : `false`.
    pub journal: bool,
    /// Défaut : `500`.
    pub delai_ms: u64,
}

/// Une valeur présente mais illisible.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErreurConfig {
    pub cle: String,
    pub valeur: String,
}

/// La valeur de `cle` lue comme un `T` (espaces autour ignorés), `defaut` si la clé est absente.
///
/// `<T: FromStr>` rend la fonction générique : `T` est n'importe quel type qui implémente le trait `FromStr`,
/// c'est-à-dire qu'on sait lire depuis du texte avec `parse` (`u16`, `bool`, `u64`…). Le compilateur choisit `T` à
/// chaque appel, d'après le type attendu : dans le champ `port: u16`, `lire_valeur(brut, "port", 8080)` lit un `u16`.
///
/// **Aide Rust** (ch00, sections 5, 6, 12)
/// - `valeur.parse()` rend un `Result<T, T::Err>` : l'erreur de `T`, inconnue ici. `` error[E0271]: type mismatch
///   resolving `<T as FromStr>::Err == ErreurConfig` `` veut dire « remplacez cette erreur par la vôtre » :
///   `map_err(|_| ErreurConfig { … })`.
/// - Dans `ErreurConfig { … }` : `` expected `String`, found `&str` `` pour la clé → `cle.to_string()` ; `` expected
///   `String`, found `&String` `` pour la valeur → `valeur.clone()`.
/// - Absent → `Ok(defaut)`, présent → le parsing : c'est un `map_or` (ou un `match` sur `brut.get(cle)`).
pub fn lire_valeur<T: FromStr>(
    brut: &HashMap<String, String>,
    cle: &str,
    defaut: T,
) -> Result<T, ErreurConfig> {
    // But : absent → `Ok(defaut)` ; présent → parsé, ou l'erreur qui nomme la clé et la valeur.
    todo!()
}

/// La configuration complète, ou la première valeur illisible (dans l'ordre hote, port, journal, delai_ms).
///
/// **Aide Rust** (ch00, sections 5, 11)
/// - Si le compilateur dit `` expected `u16`, found `Result<{integer}, ErreurConfig>` `` : `lire_valeur(…)?` ouvre le
///   résultat, et sort de la fonction à la première erreur.
/// - Si le compilateur dit `` error[E0308]: mismatched types … expected `&String`, found `&str` `` sur
///   `unwrap_or("localhost")` : `get` rend une `Option<&String>`. Passez d'abord en `Option<String>` avec `map`, puis
///   donnez un défaut qui est aussi une `String`.
pub fn lire_config(brut: &HashMap<String, String>) -> Result<Config, ErreurConfig> {
    // But : le `struct` rempli champ par champ, sans un seul `match`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::style;

    fn brut(paires: &[(&str, &str)]) -> HashMap<String, String> {
        paires
            .iter()
            .map(|(c, v)| (c.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn tout_par_defaut() {
        assert_eq!(
            lire_config(&HashMap::new()),
            Ok(Config {
                hote: "localhost".to_string(),
                port: 8080,
                journal: false,
                delai_ms: 500,
            })
        );
    }

    #[test]
    fn valeurs_fournies() {
        let config = lire_config(&brut(&[
            ("hote", " exemple.fr "),
            ("port", "443"),
            ("journal", "true"),
            ("delai_ms", " 20 "),
            ("inconnue", "ignorée"),
        ]));
        assert_eq!(
            config,
            Ok(Config {
                hote: "exemple.fr".to_string(),
                port: 443,
                journal: true,
                delai_ms: 20,
            })
        );
    }

    #[test]
    fn une_valeur_illisible_est_une_erreur_pas_un_defaut() {
        let erreur = |cle: &str, valeur: &str| {
            Err(ErreurConfig {
                cle: cle.to_string(),
                valeur: valeur.to_string(),
            })
        };
        assert_eq!(
            lire_config(&brut(&[("port", "huit")])),
            erreur("port", "huit")
        );
        assert_eq!(
            lire_config(&brut(&[("port", "70000")])),
            erreur("port", "70000")
        );
        assert_eq!(
            lire_config(&brut(&[("journal", "oui")])),
            erreur("journal", "oui")
        );
        assert_eq!(
            lire_config(&brut(&[("delai_ms", "-1")])),
            erreur("delai_ms", "-1")
        );
    }

    #[test]
    fn la_premiere_erreur_dans_l_ordre_des_champs() {
        let config = lire_config(&brut(&[("delai_ms", "x"), ("port", "y")]));
        assert_eq!(config.map_err(|e| e.cle), Err("port".to_string()));
    }

    #[test]
    fn lire_valeur_seule() {
        let b = brut(&[("n", "12")]);
        assert_eq!(lire_valeur(&b, "n", 0_i32), Ok(12));
        assert_eq!(lire_valeur(&b, "absente", 7_i32), Ok(7));
        assert!(lire_valeur(&b, "n", 0_u8).is_ok());
        assert!(lire_valeur(&brut(&[("n", "300")]), "n", 0_u8).is_err());
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e05_config.rs"),
            &["lire_valeur", "lire_config"],
        );
    }
}
