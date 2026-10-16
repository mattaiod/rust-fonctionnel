//! Exercice 4 : Arena : seule une soumission scorée entre au classement.
//!
//! Le cas du cours (C2, `Submission<Pending>` / `Submission<Scored>`), en petit. L'état `Scoree` porte le
//! score : c'est un vrai champ. `scorer` peut échouer (longueurs différentes) : la transition rend un `Result`
//! dont le `Ok` est dans le nouvel état. En cas d'échec, la soumission est perdue. Ce n'est pas gênant ici : une
//! soumission impossible à scorer ne sert plus à rien, l'équipe en déposera une autre (l'exercice 5 montre le
//! cas où il faut rendre l'objet dans l'erreur). Et `classement` ne prend que des `Soumission<Scoree>` : la
//! question « et si une soumission n'a pas de score ? » ne se pose plus, il n'y a ni `Option` ni `expect`.
//!
//! Le score est l'erreur absolue moyenne (MAE) : moyenne des `|prédiction - vérité|`, plus petit = meilleur.

use pf_outils::trier_par;

/// Pourquoi une soumission ne peut pas être scorée.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurScore {
    /// Aucune prédiction (et aucune vérité) : pas de moyenne.
    Vide,
    LongueursDifferentes {
        verite: usize,
        predictions: usize,
    },
}

/// Version naïve : le score est une `Option`, remplie par effet de bord. Ne pas modifier.
#[derive(Debug, Clone)]
pub struct SoumissionNaive {
    pub equipe: String,
    pub predictions: Vec<f64>,
    pub score: Option<f64>,
}

/// Version naïve : panique si les longueurs diffèrent. Ne pas modifier.
pub fn scorer_naif(soumission: &mut SoumissionNaive, verite: &[f64]) {
    if soumission.predictions.len() != verite.len() || verite.is_empty() {
        panic!("soumission de {} impossible à scorer", soumission.equipe);
    }
    let mut total = 0.0;
    for (prediction, vraie) in soumission.predictions.iter().zip(verite) {
        total += (prediction - vraie).abs();
    }
    soumission.score = Some(total / verite.len() as f64);
}

/// Version naïve : panique si l'une des soumissions n'a pas été scorée. Ne pas modifier.
pub fn classement_naif(soumissions: &[SoumissionNaive]) -> Vec<(String, f64)> {
    let mut lignes = Vec::new();
    for s in soumissions {
        lignes.push((s.equipe.clone(), s.score.expect("soumission non scorée")));
    }
    lignes.sort_by(|a, b| a.1.total_cmp(&b.1));
    lignes
}

/// État : déposée, pas encore scorée.
#[derive(Debug)]
pub struct EnAttente;

/// État : scorée, avec son score.
#[derive(Debug)]
pub struct Scoree {
    score: f64,
}

/// Une soumission d'équipe.
///
/// Ce qui compile :
///
/// ```no_run
/// use ch12_typestate::e04_soumission::{classement, Soumission};
/// let scoree = Soumission::nouvelle("lambda", vec![1.0, 2.0]).scorer(&[1.0, 3.0]).unwrap();
/// let podium = classement(&[scoree]);
/// ```
///
/// Classer une soumission en attente ne compile pas : `classement` veut des `Soumission<Scoree>`.
///
/// ```compile_fail,E0308
/// use ch12_typestate::e04_soumission::{classement, Soumission};
/// let podium = classement(&[Soumission::nouvelle("lambda", vec![1.0])]);
/// ```
///
/// Rescorer une soumission scorée non plus : `scorer` n'existe que sur `Soumission<EnAttente>`.
///
/// ```compile_fail,E0599
/// use ch12_typestate::e04_soumission::Soumission;
/// let scoree = Soumission::nouvelle("lambda", vec![1.0]).scorer(&[1.0]).unwrap();
/// let encore = scoree.scorer(&[2.0]);
/// ```
#[derive(Debug)]
pub struct Soumission<S> {
    equipe: String,
    predictions: Vec<f64>,
    etat: S,
}

impl<S> Soumission<S> {
    /// Le nom de l'équipe, dans tous les états.
    pub fn equipe(&self) -> &str {
        todo!()
    }
}

impl Soumission<EnAttente> {
    /// Une soumission déposée.
    ///
    /// **Aide Rust** (ch00, section 5)
    /// - `equipe` est une `&str`, le champ une `String` : `E0308 … expected String, found &str`.
    pub fn nouvelle(equipe: &str, predictions: Vec<f64>) -> Self {
        todo!()
    }

    /// En attente → scorée, ou l'erreur. La vérification des longueurs passe avant celle du vide.
    ///
    /// **Aide Rust** (ch00, sections 1, 12)
    /// - Une variante à champs nommés se construit avec ses champs : `ErreurScore::LongueursDifferentes` seul donne
    ///   `E0533 expected value, found struct variant`.
    /// - `sum()` suivi d'une division donne `E0283 type annotations needed` : `sum::<f64>()`. Puis
    ///   `E0277 cannot divide f64 by usize` : convertissez la longueur avec `as f64`.
    /// - Le type change : `Soumission { etat: Scoree { score }, ..self }` donne
    ///   `E0308 … expected Soumission<Scoree>, found Soumission<EnAttente>`. Listez les champs.
    pub fn scorer(self, verite: &[f64]) -> Result<Soumission<Scoree>, ErreurScore> {
        // But : vérifier, calculer la MAE, rendre la soumission dans son nouvel état.
        todo!()
    }
}

impl Soumission<Scoree> {
    /// Le score : pas d'`Option`, il existe forcément.
    pub fn score(&self) -> f64 {
        todo!()
    }
}

/// `(équipe, score)` du meilleur (plus petit score) au moins bon ; à égalité, l'ordre d'arrivée est gardé.
///
/// **Aide Rust** (ch00, sections 2, 6)
/// - `iter()` prête les soumissions : `(s.equipe, ...)` donne
///   `E0507 cannot move out of s.equipe which is behind a shared reference`. Le résultat possède ses noms :
///   `s.equipe.clone()` (ou `s.equipe().to_string()`).
/// - Pour comparer deux `f64` dans `trier_par`, `partial_cmp` rend une `Option<Ordering>`
///   (`E0308 … expected Ordering, found Option<Ordering>`) : `total_cmp`.
pub fn classement(soumissions: &[Soumission<Scoree>]) -> Vec<(String, f64)> {
    // But : aucun `expect`, aucun `unwrap` : le type a déjà tout vérifié.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    #[test]
    fn cas_connus() {
        let s = Soumission::nouvelle("lambda", vec![1.0, 2.0, 3.0])
            .scorer(&[1.0, 4.0, 2.0])
            .unwrap();
        assert_eq!(s.equipe(), "lambda");
        assert_eq!(s.score(), 1.0);
    }

    #[test]
    fn erreurs_de_score() {
        assert_eq!(
            Soumission::nouvelle("a", vec![1.0])
                .scorer(&[1.0, 2.0])
                .unwrap_err(),
            ErreurScore::LongueursDifferentes {
                verite: 2,
                predictions: 1
            }
        );
        assert_eq!(
            Soumission::nouvelle("a", vec![]).scorer(&[]).unwrap_err(),
            ErreurScore::Vide
        );
    }

    #[test]
    fn classement_du_meilleur_au_moins_bon() {
        let verite = [0.0, 0.0];
        let scorees: Vec<Soumission<Scoree>> = [("a", 2.0), ("b", 1.0), ("c", 2.0), ("d", 0.5)]
            .iter()
            .map(|(e, p)| {
                Soumission::nouvelle(e, vec![*p, *p])
                    .scorer(&verite)
                    .unwrap()
            })
            .collect();
        assert_eq!(
            classement(&scorees),
            vec![
                ("d".to_string(), 0.5),
                ("b".to_string(), 1.0),
                ("a".to_string(), 2.0),
                ("c".to_string(), 2.0)
            ]
        );
        assert_eq!(classement(&[]), vec![]);
    }

    #[test]
    fn meme_resultat_que_le_naif() {
        (1..15_u64).for_each(|graine| {
            let verite: Vec<f64> = jeux::entiers(graine, 10, -50, 50)
                .into_iter()
                .map(|n| n as f64)
                .collect();
            let equipes: Vec<(String, Vec<f64>)> = (0..graine)
                .map(|i| {
                    let preds = jeux::entiers(graine * 100 + i, 10, -50, 50)
                        .into_iter()
                        .map(|n| n as f64)
                        .collect();
                    (format!("equipe{i}"), preds)
                })
                .collect();
            let naives: Vec<SoumissionNaive> = equipes
                .iter()
                .map(|(e, p)| {
                    let mut s = SoumissionNaive {
                        equipe: e.clone(),
                        predictions: p.clone(),
                        score: None,
                    };
                    scorer_naif(&mut s, &verite);
                    s
                })
                .collect();
            let typees: Vec<Soumission<Scoree>> = equipes
                .iter()
                .map(|(e, p)| Soumission::nouvelle(e, p.clone()).scorer(&verite).unwrap())
                .collect();
            assert_eq!(
                classement(&typees),
                classement_naif(&naives),
                "graine {graine}"
            );
        });
    }

    #[test]
    #[should_panic(expected = "non scorée")]
    fn le_naif_panique_sur_une_soumission_non_scoree() {
        classement_naif(&[SoumissionNaive {
            equipe: "a".to_string(),
            predictions: vec![1.0],
            score: None,
        }]);
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e04_soumission.rs"),
            &["equipe", "nouvelle", "scorer", "score", "classement"],
        );
    }
}
