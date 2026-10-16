//! Exercice 3 : les tags d'un blog, sans doublon et triés.
//!
//! Chaque article porte une liste de tags. On veut UNE liste : tous les tags de tous les articles. Un `map` donnerait
//! une liste de listes ; `flat_map` aplatit au passage (chaque article donne une séquence, on enchaîne les séquences).
//! Pour dédoublonner ET trier, inutile de trier puis de `dedup` : un `BTreeSet` est un ensemble trié, et `collect`
//! sait le remplir. Un second `collect` le ramène en `Vec`.

use std::collections::BTreeSet;

/// Un article de blog.
#[derive(Debug, Clone, PartialEq)]
pub struct Article {
    pub titre: String,
    pub tags: Vec<String>,
}

/// Version impérative : deux boucles imbriquées, un test d'appartenance, un tri final. Ne pas modifier.
pub fn tags_uniques_imperatif(articles: &[Article]) -> Vec<String> {
    let mut tags: Vec<String> = Vec::new();
    for article in articles {
        for tag in &article.tags {
            if !tags.contains(tag) {
                tags.push(tag.clone());
            }
        }
    }
    tags.sort();
    tags
}

/// Tous les tags des articles, une seule fois chacun, dans l'ordre alphabétique.
///
/// **Aide Rust** (ch00, sections 2, 6, 12)
/// - `article.tags.iter()` donne des `&String` (empruntées) ; un `BTreeSet<String>` veut des `String` possédées : il
///   faut copier chaque tag (`cloned()`).
/// - Si le compilateur dit `` error[E0277]: a value of type `BTreeSet<String>` cannot be built from an iterator over
///   elements of type `&String` `` : ajoutez `.cloned()` sur l'itérateur des tags.
/// - Si le compilateur dit `` error[E0507]: cannot move out of `a.tags` which is behind a shared reference `` : `|a|
///   a.tags` essaie de prendre le `Vec` d'un article seulement emprunté. Passez par `a.tags.iter()`.
/// - Si le compilateur dit `error[E0282]: type annotations needed` sur le premier `collect` : il ne sait pas quoi
///   fabriquer, dites-le : `collect::<BTreeSet<String>>()`.
pub fn tags_uniques(articles: &[Article]) -> Vec<String> {
    // Enchaîner les tags de tous les articles en une seule séquence, les ranger dans un ensemble trié, revenir en `Vec`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    fn article(titre: &str, tags: &[&str]) -> Article {
        Article {
            titre: titre.to_string(),
            tags: tags.iter().map(|t| t.to_string()).collect(),
        }
    }

    #[test]
    fn cas_connus() {
        let articles = [
            article("Les itérateurs", &["rust", "pf"]),
            article("Ownership", &["rust", "mémoire"]),
            article("Haskell", &["pf", "haskell"]),
        ];
        assert_eq!(
            tags_uniques(&articles),
            vec!["haskell", "mémoire", "pf", "rust"]
        );
    }

    #[test]
    fn sans_article_ou_sans_tag() {
        assert_eq!(tags_uniques(&[]), Vec::<String>::new());
        assert_eq!(
            tags_uniques(&[article("Brouillon", &[])]),
            Vec::<String>::new()
        );
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        (0..20).for_each(|graine| {
            let articles: Vec<Article> = (0..graine)
                .map(|n| Article {
                    titre: format!("article {n}"),
                    tags: jeux::mots(graine * 100 + n, (n % 5) as usize, 3),
                })
                .collect();
            assert_eq!(
                tags_uniques(&articles),
                tags_uniques_imperatif(&articles),
                "articles : {articles:?}"
            )
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(include_str!("e03_tags_uniques.rs"), &["tags_uniques"]);
    }
}
