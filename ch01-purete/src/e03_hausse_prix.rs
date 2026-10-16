//! Exercice 3 : augmenter les prix d'un panier, avec une nouvelle `struct` plutôt qu'une `struct` modifiée.
//!
//! La syntaxe `Article { prix_centimes: nouveau, ..ancien }` (*struct update syntax*) construit une nouvelle
//! valeur : on écrit le champ qui change, `..ancien` reprend tous les autres. Comme `article` et `panier` sont
//! reçus par valeur, leurs champs (le `String` du nom, le `Vec` des articles) sont DÉPLACÉS dans la nouvelle
//! valeur : aucune copie, et l'ancien panier n'est plus utilisable, donc personne ne peut le voir à moitié modifié.
//! Les prix sont en centimes (des entiers) : `prix × (100 + pourcent) / 100`, division entière.

/// Une ligne du panier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Article {
    pub nom: String,
    pub prix_centimes: u64,
    pub quantite: u32,
}

/// Le panier d'un client.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Panier {
    pub client: String,
    pub articles: Vec<Article>,
    pub code_promo: Option<String>,
}

/// Version impérative : modifie le panier de l'appelant. Ne pas modifier.
pub fn appliquer_hausse_imperatif(panier: &mut Panier, pourcent: u64) {
    for article in panier.articles.iter_mut() {
        article.prix_centimes = article.prix_centimes * (100 + pourcent) / 100;
    }
}

/// Le même article, prix augmenté de `pourcent` %.
///
/// **Aide Rust** (ch00, sections 2, 7)
/// - Si le compilateur dit
///   ``E0594: cannot assign to `article.prix_centimes`, as `article` is not declared as mutable`` : ne modifiez pas
///   `article`, construisez `Article { prix_centimes: …, ..article }` (le `..article` en dernier).
pub fn augmenter(article: Article, pourcent: u64) -> Article {
    // Outil : `Article { prix_centimes: …, ..article }`.
    todo!()
}

/// Le même panier, chaque article augmenté de `pourcent` %.
///
/// **Aide Rust** (ch00, sections 2, 3, 6)
/// - `iter()` PRÊTE les articles (`&Article`) alors que `augmenter` veut un `Article` à lui. Le panier vous
///   appartient : `panier.articles.into_iter()` vous DONNE ses articles.
/// - Si le compilateur dit ``E0308: expected `Article`, found `&Article` `` puis, avec une `*`,
///   ``E0507: cannot move out of `*a` which is behind a shared reference`` : c'est `into_iter()` qu'il faut, ni `*`
///   ni `clone()`.
/// - Après avoir pris `panier.articles`, `..panier` reste permis : il ne recopie que les autres champs.
pub fn appliquer_hausse(panier: Panier, pourcent: u64) -> Panier {
    // Outils : `into_iter()`, `map(|article| augmenter(…))`, `collect()`, puis `..panier`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    fn article(nom: &str, prix_centimes: u64) -> Article {
        Article {
            nom: nom.to_string(),
            prix_centimes,
            quantite: 2,
        }
    }

    fn panier(prix: &[i64]) -> Panier {
        Panier {
            client: "Ada".to_string(),
            articles: prix
                .iter()
                .enumerate()
                .map(|(i, p)| article(&format!("article {i}"), *p as u64))
                .collect(),
            code_promo: Some("RENTREE".to_string()),
        }
    }

    #[test]
    fn cas_connus() {
        assert_eq!(augmenter(article("stylo", 200), 10), article("stylo", 220));
        assert_eq!(augmenter(article("gomme", 99), 5), article("gomme", 103));
        let apres = appliquer_hausse(panier(&[1000, 250]), 20);
        assert_eq!(apres, panier(&[1200, 300]));
    }

    #[test]
    fn garde_les_autres_champs() {
        let apres = appliquer_hausse(panier(&[100]), 50);
        assert_eq!(apres.client, "Ada");
        assert_eq!(apres.code_promo, Some("RENTREE".to_string()));
        assert_eq!(apres.articles[0].nom, "article 0");
        assert_eq!(apres.articles[0].quantite, 2);
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        jeux::plusieurs(1, 100_000).iter().for_each(|prix| {
            (0..30).step_by(7).for_each(|pourcent| {
                let modifie = {
                    let mut p = panier(prix);
                    appliquer_hausse_imperatif(&mut p, pourcent);
                    p
                };
                assert_eq!(
                    appliquer_hausse(panier(prix), pourcent),
                    modifie,
                    "prix : {prix:?}"
                );
            })
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e03_hausse_prix.rs"),
            &["augmenter", "appliquer_hausse"],
        );
    }
}
