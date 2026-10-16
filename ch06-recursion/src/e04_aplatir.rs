//! Exercice 4 : aplatir une liste imbriquée.
//!
//! `[1, [2, [3, 4]], 5]` → `[1, 2, 3, 4, 5]`. Une `Imbrique` est soit une valeur, soit une liste d'`Imbrique` : le
//! type est récursif, la fonction le sera aussi. Ici, pas besoin de `Box` : le `Vec` met déjà ses éléments sur le tas.
//! Le cas récursif combine les deux outils du cours : aplatir chaque élément de la liste (récursion), puis enchaîner
//! les résultats en une seule liste (le combinator qui aplatit, chapitre 5).
//! La version impérative montre ce que la récursion vous épargne : une pile, et l'ordre à remettre à l'endroit.

/// Une valeur, ou une liste de valeurs imbriquées.
#[derive(Debug, Clone, PartialEq)]
pub enum Imbrique {
    Valeur(i64),
    Liste(Vec<Imbrique>),
}

/// Version impérative : une pile, et les enfants empilés à l'envers pour garder l'ordre. Ne pas modifier.
pub fn aplatir_imperatif(donnee: &Imbrique) -> Vec<i64> {
    let mut resultat = Vec::new();
    let mut pile = vec![donnee];
    while let Some(courant) = pile.pop() {
        match courant {
            Imbrique::Valeur(v) => resultat.push(*v),
            Imbrique::Liste(enfants) => {
                for enfant in enfants.iter().rev() {
                    pile.push(enfant);
                }
            }
        }
    }
    resultat
}

/// Toutes les valeurs, dans l'ordre de lecture, sans les niveaux d'imbrication.
///
/// **Aide Rust** (ch00, sections 2, 12)
/// - `Imbrique::Valeur(v)` sur un `&Imbrique` lie `v: &i64` : `` expected `i64`, found `&i64` `` dans `vec![v]` se
///   corrige en `vec![*v]`.
/// - Si le compilateur dit `` error[E0277]: a value of type `Vec<i64>` cannot be built from an iterator over elements
///   of type `Vec<i64>` `` : `map(aplatir)` donne une liste de listes. Prenez le combinator qui aplatit au passage
///   (chapitre 5).
/// - `aplatir` se passe par son nom : elle prend un `&Imbrique`, exactement ce que donne `enfants.iter()`.
pub fn aplatir(donnee: &Imbrique) -> Vec<i64> {
    // Une valeur donne une liste d'un élément ; une liste donne la concaténation de ses éléments aplatis.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use Imbrique::{Liste, Valeur};
    use pf_outils::{jeux, style};

    /// Construit une imbrication pseudo-aléatoire : un nombre pair ouvre une sous-liste de 2 éléments.
    fn construire(nombres: &[i64], niveau: usize) -> Imbrique {
        let morceaux: Vec<Imbrique> = nombres
            .chunks(3)
            .map(|bloc| match bloc {
                [x, reste @ ..] if x % 2 == 0 && niveau < 4 => {
                    Liste(vec![Valeur(*x), construire(reste, niveau + 1)])
                }
                _ => Liste(bloc.iter().copied().map(Valeur).collect()),
            })
            .collect();
        Liste(morceaux)
    }

    #[test]
    fn cas_connus() {
        let donnee = Liste(vec![
            Valeur(1),
            Liste(vec![Valeur(2), Liste(vec![Valeur(3), Valeur(4)])]),
            Valeur(5),
        ]);
        assert_eq!(aplatir(&donnee), vec![1, 2, 3, 4, 5]);
        assert_eq!(aplatir(&Valeur(7)), vec![7]);
    }

    #[test]
    fn listes_vides() {
        assert_eq!(aplatir(&Liste(vec![])), Vec::<i64>::new());
        assert_eq!(
            aplatir(&Liste(vec![Liste(vec![]), Liste(vec![Liste(vec![])])])),
            Vec::<i64>::new()
        );
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        jeux::plusieurs(-20, 20).iter().for_each(|jeu| {
            let donnee = construire(jeu, 0);
            assert_eq!(
                aplatir(&donnee),
                aplatir_imperatif(&donnee),
                "donnée : {donnee:?}"
            );
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(include_str!("e04_aplatir.rs"), &["aplatir"]);
    }
}
