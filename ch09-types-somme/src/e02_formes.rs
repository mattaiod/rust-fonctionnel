//! Exercice 2 : des formes et leur aire.
//!
//! La version naïve code la forme dans une `String` et ses dimensions dans deux `f64` dont le sens change selon la
//! forme (`b` ne sert à rien pour un cercle). Une faute de frappe (`"cercel"`) compile et plante à l'exécution. Le
//! type somme met dans chaque variante exactement les données qu'elle demande, avec leur nom : un cercle n'a qu'un
//! rayon. On appelle ça « somme de produits » : un `enum` (OU) dont chaque variante est un `struct` (ET).

use std::f64::consts::PI;

/// Version naïve : la forme dans une chaîne, deux nombres au sens variable. Ne pas modifier.
#[derive(Debug, Clone, PartialEq)]
pub struct FormeNaive {
    pub genre: String,
    pub a: f64,
    pub b: f64,
}

/// Version naïve de l'aire : panique sur un genre inconnu. Ne pas modifier.
pub fn aire_naif(forme: &FormeNaive) -> f64 {
    if forme.genre == "cercle" {
        PI * forme.a * forme.a
    } else if forme.genre == "rectangle" {
        forme.a * forme.b
    } else if forme.genre == "triangle" {
        forme.a * forme.b / 2.0
    } else {
        panic!("forme inconnue : {}", forme.genre)
    }
}

/// Une forme, avec les seules dimensions qui la décrivent.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Forme {
    Cercle { rayon: f64 },
    Rectangle { largeur: f64, hauteur: f64 },
    Triangle { base: f64, hauteur: f64 },
}

impl Forme {
    /// L'aire de la forme.
    ///
    /// **Aide Rust** (ch00, section 2)
    /// - Les variantes ont des champs **nommés** : `Forme::Cercle { rayon }`, pas `Forme::Cercle(rayon)`
    ///   (`` error[E0164]: expected tuple struct or tuple variant, found struct variant `Forme::Cercle` ``).
    /// - Si le compilateur dit `` error[E0027]: pattern does not mention field `hauteur` `` : nommez tous les champs,
    ///   ou ignorez les autres avec `..`.
    /// - `match self` (un `&Forme`) lie des `&f64`, `match *self` les copie en `f64` (`Forme` est `Copy`) : les deux
    ///   compilent ici.
    pub fn aire(&self) -> f64 {
        // But : un `match` qui déstructure chaque variante.
        todo!()
    }

    /// Le nom de la forme, en français et au singulier.
    pub fn nom(&self) -> &'static str {
        // But : un `match` qui ignore les données (`{ .. }`).
        todo!()
    }
}

/// La somme des aires.
pub fn aire_totale(formes: &[Forme]) -> f64 {
    // But : chapitre 3, avec la méthode écrite plus haut.
    todo!()
}

/// Les carrés (rectangles de côtés égaux), dans l'ordre.
///
/// **Aide Rust** (ch00, sections 3, 12)
/// - Si le compilateur dit `` error[E0277]: a value of type `Vec<Forme>` cannot be built from an iterator over elements
///   of type `&Forme` `` : `.copied()` avant `collect` (`Forme` est `Copy`).
/// - `matches!(valeur, Motif { … } if condition)` rend un `bool`, parfait pour un `filter`.
pub fn carres(formes: &[Forme]) -> Vec<Forme> {
    // But : un motif avec garde dans un `filter` (pensez à `matches!`).
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::style;

    fn proche(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn cas_connus() {
        assert!(proche(Forme::Cercle { rayon: 1.0 }.aire(), PI));
        assert!(proche(
            Forme::Rectangle {
                largeur: 2.0,
                hauteur: 3.0
            }
            .aire(),
            6.0
        ));
        assert!(proche(
            Forme::Triangle {
                base: 4.0,
                hauteur: 3.0
            }
            .aire(),
            6.0
        ));
        assert_eq!(
            Forme::Triangle {
                base: 1.0,
                hauteur: 1.0
            }
            .nom(),
            "triangle"
        );
        assert!(proche(aire_totale(&[]), 0.0));
    }

    #[test]
    fn meme_aire_que_le_naif() {
        let paires = [
            (Forme::Cercle { rayon: 2.5 }, ("cercle", 2.5, 0.0)),
            (
                Forme::Rectangle {
                    largeur: 2.0,
                    hauteur: 7.0,
                },
                ("rectangle", 2.0, 7.0),
            ),
            (
                Forme::Triangle {
                    base: 5.0,
                    hauteur: 3.0,
                },
                ("triangle", 5.0, 3.0),
            ),
        ];
        paires.iter().for_each(|(forme, (genre, a, b))| {
            let naive = FormeNaive {
                genre: genre.to_string(),
                a: *a,
                b: *b,
            };
            assert!(proche(forme.aire(), aire_naif(&naive)), "{forme:?}");
            assert_eq!(forme.nom(), *genre);
        });
        let formes: Vec<Forme> = paires.iter().map(|(f, _)| *f).collect();
        assert!(proche(aire_totale(&formes), PI * 6.25 + 14.0 + 7.5));
    }

    #[test]
    #[should_panic(expected = "forme inconnue")]
    fn le_naif_compile_une_faute_de_frappe() {
        aire_naif(&FormeNaive {
            genre: "cercel".to_string(),
            a: 1.0,
            b: 0.0,
        });
    }

    #[test]
    fn garder_les_carres() {
        let formes = [
            Forme::Rectangle {
                largeur: 2.0,
                hauteur: 2.0,
            },
            Forme::Cercle { rayon: 2.0 },
            Forme::Rectangle {
                largeur: 2.0,
                hauteur: 3.0,
            },
            Forme::Triangle {
                base: 2.0,
                hauteur: 2.0,
            },
            Forme::Rectangle {
                largeur: 5.0,
                hauteur: 5.0,
            },
        ];
        assert_eq!(carres(&formes), vec![formes[0], formes[4]]);
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e02_formes.rs"),
            &["aire", "nom", "aire_totale", "carres"],
        );
    }
}
