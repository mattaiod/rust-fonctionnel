//! Exercice 3 : deux couches, `From` et `?`, la chaîne des causes.
//!
//! Chaque couche a son `enum` : la lecture (`ErreurLecture`) ne connaît rien au métier, le métier
//! (`ErreurCommande`) enveloppe la lecture dans une variante. `?` appelle `From::from` sur l'erreur : avec
//! `impl From<ErreurLecture> for ErreurCommande`, une erreur de lecture remonte d'elle-même au bon type.
//! L'erreur enveloppée se rend par `Error::source` : c'est ce qui permet à un `main` (ou à `anyhow`) d'afficher
//! toute la chaîne « ligne illisible : pas un entier positif : invalid digit found in string ».

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

/// Une ligne de commande lue depuis `produit;quantité;prix en centimes`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LigneCommande {
    pub produit: String,
    pub quantite: u32,
    pub prix_centimes: u32,
}

/// Version naïve : chaque étape fabrique la phrase complète à la main, causes comprises. L'appelant ne peut
/// rien trier, et la moindre étape ajoutée oblige à recopier les préfixes partout. Ne pas modifier.
pub fn lire_commande_naif(ligne: &str) -> Result<LigneCommande, String> {
    let champs: Vec<&str> = ligne.split(';').collect();
    let produit = champs[0].trim();
    if produit.is_empty() {
        return Err("produit vide".to_string());
    }
    let quantite_brute = champs
        .get(1)
        .ok_or("ligne illisible : champ 2 manquant".to_string())?;
    let quantite: u32 = quantite_brute
        .trim()
        .parse()
        .map_err(|e| format!("ligne illisible : pas un entier positif : {e}"))?;
    if quantite == 0 {
        return Err("quantité nulle".to_string());
    }
    let prix_brut = champs
        .get(2)
        .ok_or("ligne illisible : champ 3 manquant".to_string())?;
    let prix_centimes: u32 = prix_brut
        .trim()
        .parse()
        .map_err(|e| format!("ligne illisible : pas un entier positif : {e}"))?;
    Ok(LigneCommande {
        produit: produit.to_string(),
        quantite,
        prix_centimes,
    })
}

/// Couche lecture : ce qui empêche de découper ou de lire un champ.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurLecture {
    /// Le champ numéro `numero` (à partir de 1) n'existe pas.
    ChampManquant { numero: usize },
    /// Le champ n'est pas un entier positif ; la cause vient de la std.
    PasUnEntier(ParseIntError),
}

/// Couche métier : une ligne lisible peut encore être absurde.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurCommande {
    Lecture(ErreurLecture),
    ProduitVide,
    QuantiteNulle,
}

impl fmt::Display for ErreurLecture {
    /// « champ 2 manquant », « pas un entier positif » (la cause est dans `source`, pas dans la phrase).
    ///
    /// **Aide Rust** (ch00, section 1)
    /// - Comme à l'exercice 2 : un bras par variante, chacun un `write!(f, ...)` sans `;`. Ne recopiez pas la cause
    ///   dans la phrase, `source` s'en charge (sinon elle s'affiche deux fois dans la chaîne).
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // But : une phrase par variante, sans recopier la cause.
        todo!()
    }
}

impl Error for ErreurLecture {
    /// **Aide Rust** (ch00, sections 2, 11)
    /// - `source` prête la cause, il ne la copie pas : `Some(cause.clone())` donne
    ///   `E0308 … expected &dyn Error, found ParseIntError`. Dans `match self`, `cause` est déjà une référence :
    ///   `Some(cause)` suffit.
    /// - Toutes les variantes doivent avoir un bras :
    ///   `E0004 non-exhaustive patterns: &ErreurLecture::ChampManquant { .. } not covered`. Une variante sans cause
    ///   rend `None`.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // But : rendre l'erreur de la std quand il y en a une.
        todo!()
    }
}

impl fmt::Display for ErreurCommande {
    /// « ligne illisible », « produit vide », « quantité nulle ».
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // But : une phrase par variante.
        todo!()
    }
}

impl Error for ErreurCommande {
    /// **Aide Rust** (ch00, section 2)
    /// - Même forme que le `source` d'`ErreurLecture` : la variante `Lecture` prête l'erreur qu'elle enveloppe, les
    ///   autres rendent `None`.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // But : rendre l'erreur de lecture enveloppée.
        todo!()
    }
}

impl From<ParseIntError> for ErreurLecture {
    /// **Aide Rust** (ch00, section 1)
    /// - La variante garde l'erreur de la std telle quelle : `Self::PasUnEntier(cause.to_string())` donne
    ///   `E0308 … expected ParseIntError, found String`.
    fn from(cause: ParseIntError) -> Self {
        // But : permettre `texte.parse::<u32>()?` dans une fonction qui rend `ErreurLecture`.
        todo!()
    }
}

impl From<ErreurLecture> for ErreurCommande {
    fn from(cause: ErreurLecture) -> Self {
        // But : permettre `?` sur une `ErreurLecture` dans une fonction qui rend `ErreurCommande`.
        todo!()
    }
}

/// Le champ numéro `numero` (à partir de 1).
///
/// **Aide Rust** (ch00, sections 11, 13)
/// - `get` rend une `Option` (`E0308 … expected Result<&str, ErreurLecture>, found Option<&&str>`) : transformez le
///   `None` en erreur.
/// - Sur une tranche de `&str`, `get` donne un `&&str` : `E0308 … found Result<&&str, ErreurLecture>`. Retirez un
///   niveau de référence avec `.copied()`.
/// - Le `'a` dit que le champ rendu vit aussi longtemps que la ligne d'origine, pas que la tranche `champs` : vous
///   n'avez rien à écrire de plus. Attention, `numero - 1` panique si `numero` vaut 0.
pub fn champ<'a>(champs: &[&'a str], numero: usize) -> Result<&'a str, ErreurLecture> {
    // But : le champ, ou `ChampManquant`.
    todo!()
}

/// Un entier positif, blancs autour tolérés.
///
/// **Aide Rust** (ch00, section 12)
/// - `texte.trim().parse()` seul rend une `ParseIntError` :
///   `E0271 type mismatch resolving <u32 as FromStr>::Err == ErreurLecture`. Le `?` appelle votre `From` : il faut
///   donc un `?`, puis un `Ok(...)` autour.
pub fn entier(texte: &str) -> Result<u32, ErreurLecture> {
    // But : une ligne, grâce au `From` écrit plus haut.
    todo!()
}

/// Lit `produit;quantité;prix en centimes`, dans cet ordre de vérification : produit non vide, quantité
/// présente, lisible et non nulle, prix présent et lisible. Les champs en trop sont ignorés.
///
/// **Aide Rust** (ch00, sections 1, 12)
/// - `champ` attend une tranche : `champ(ligne.split(';'), 1)` donne
///   `E0308 … expected &[&str], found Split<'_, char>`. Collectez d'abord, en annotant :
///   `let champs: Vec<&str> = ...` (sans annotation, `E0277 a slice of type [&str] cannot be built`).
/// - `champ(...)` rend un `Result` : mettez son `?` avant de passer le texte à `entier` (sinon
///   `E0308 … expected &str, found Result<&str, ErreurLecture>`).
/// - Si vous parsez vous-même au lieu d'appeler `entier`, le `?` cherche un `From<ParseIntError> for ErreurCommande`
///   qui n'existe pas : `E0277 ? couldn't convert the error to ErreurCommande`.
pub fn lire_commande(ligne: &str) -> Result<LigneCommande, ErreurCommande> {
    // But : enchaîner `champ` et `entier` avec `?`, sans un seul `map_err`.
    todo!()
}

/// Toute la chaîne des causes, séparées par « : » : le message de l'erreur, puis celui de sa `source`, et ainsi
/// de suite jusqu'à la dernière.
///
/// **Aide Rust** (ch00, sections 6, 13)
/// - Le plus simple est la récursion (chapitre 6) : le message de l'erreur, puis, s'il y a une `source`, « : » et la
///   `chaine` de cette source.
/// - Avec `std::iter::successors(Some(erreur), |e| e.source())`, le compilateur dit
///   `lifetime may not live long enough` : la closure reçoit un `&&dyn Error`. Déstructurez-le avec `|&e|`.
/// - `collect()` suivi de `join` donne `E0282 type annotations needed` : `collect::<Vec<_>>()`.
pub fn chaine(erreur: &(dyn Error + 'static)) -> String {
    // But : suivre `source()` jusqu'au bout, sans boucle.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::style;

    #[test]
    fn cas_connus() {
        assert_eq!(
            lire_commande("café ; 2 ; 450"),
            Ok(LigneCommande {
                produit: "café".to_string(),
                quantite: 2,
                prix_centimes: 450
            })
        );
        assert_eq!(lire_commande(" ;2;450"), Err(ErreurCommande::ProduitVide));
        assert_eq!(
            lire_commande("thé;0;300"),
            Err(ErreurCommande::QuantiteNulle)
        );
        assert_eq!(
            lire_commande("thé;3"),
            Err(ErreurCommande::Lecture(ErreurLecture::ChampManquant {
                numero: 3
            }))
        );
        assert!(matches!(
            lire_commande("thé;deux;300"),
            Err(ErreurCommande::Lecture(ErreurLecture::PasUnEntier(_)))
        ));
    }

    #[test]
    fn la_conversion_marche_seule() {
        let erreur: ErreurCommande = ErreurLecture::ChampManquant { numero: 2 }.into();
        assert_eq!(
            erreur,
            ErreurCommande::Lecture(ErreurLecture::ChampManquant { numero: 2 })
        );
        assert!(matches!(entier("x"), Err(ErreurLecture::PasUnEntier(_))));
        assert_eq!(entier(" 42 "), Ok(42));
    }

    #[test]
    fn chaine_des_causes() {
        let erreur = lire_commande("thé;-1;300").unwrap_err();
        assert_eq!(erreur.to_string(), "ligne illisible");
        assert_eq!(
            chaine(&erreur),
            "ligne illisible : pas un entier positif : invalid digit found in string"
        );
        assert_eq!(chaine(&ErreurCommande::QuantiteNulle), "quantité nulle");
    }

    #[test]
    fn meme_resultat_que_le_naif() {
        [
            "café;2;450",
            "café;2;450;en trop",
            "",
            ";1;1",
            "pain",
            "pain;",
            "pain;1",
            "pain;1;",
            "pain;0;100",
            "pain;0",
            "pain;x;100",
            "pain;3;cent",
            "pain;99999999999;1",
            " croissant ; 12 ; 110 ",
        ]
        .iter()
        .for_each(|ligne| {
            assert_eq!(
                lire_commande(ligne).map_err(|e| chaine(&e)),
                lire_commande_naif(ligne),
                "ligne : {ligne:?}"
            )
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e03_commande.rs"),
            &["champ", "entier", "lire_commande", "chaine"],
        );
    }
}
