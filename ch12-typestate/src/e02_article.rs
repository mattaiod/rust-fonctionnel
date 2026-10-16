//! Exercice 2 : un article qu'on ne publie qu'après relecture.
//!
//! Brouillon → en relecture → publié, et retour au brouillon si le relecteur rejette. Cette fois, deux états
//! portent une donnée (le nom du relecteur) : l'état devient un vrai champ `etat: E`, plus besoin de
//! `PhantomData`. Les méthodes communes à tous les états vont dans `impl<E> Article<E>`, les autres dans
//! l'`impl` de leur état. Une transition qui garde le type (`rediger`) prend `self` et rend `Self`.

/// Version naïve : un statut en `String` et des `Result<(), String>` partout. Ne pas modifier.
///
/// Elle contient un bug, laissé exprès : `valider` vérifie qu'il y a un relecteur, pas que l'article est en
/// relecture. Or `renvoyer_en_brouillon` oublie d'effacer le relecteur : un article rejeté peut être publié
/// sans nouvelle relecture (voir le test `le_naif_publie_un_brouillon`). Avec des types, ce bug n'a pas de place.
#[derive(Debug)]
pub struct ArticleNaif {
    pub titre: String,
    pub texte: String,
    pub statut: String,
    pub relecteur: Option<String>,
}

impl ArticleNaif {
    /// Ne pas modifier.
    pub fn creer(titre: &str) -> Self {
        ArticleNaif {
            titre: titre.to_string(),
            texte: String::new(),
            statut: "brouillon".to_string(),
            relecteur: None,
        }
    }

    /// Ne pas modifier.
    pub fn ecrire(&mut self, ajout: &str) -> Result<(), String> {
        if self.statut != "brouillon" {
            return Err(format!("impossible d'écrire : statut {}", self.statut));
        }
        self.texte.push_str(ajout);
        Ok(())
    }

    /// Ne pas modifier.
    pub fn envoyer_en_relecture(&mut self, relecteur: &str) -> Result<(), String> {
        if self.statut != "brouillon" {
            return Err(format!("déjà {}", self.statut));
        }
        self.statut = "relecture".to_string();
        self.relecteur = Some(relecteur.to_string());
        Ok(())
    }

    /// Ne pas modifier.
    pub fn renvoyer_en_brouillon(&mut self) -> Result<(), String> {
        if self.statut != "relecture" {
            return Err(format!("pas en relecture : {}", self.statut));
        }
        self.statut = "brouillon".to_string();
        Ok(())
    }

    /// Ne pas modifier (le bug est ici).
    pub fn valider(&mut self) -> Result<(), String> {
        if self.relecteur.is_none() {
            return Err("aucun relecteur".to_string());
        }
        self.statut = "publié".to_string();
        Ok(())
    }
}

/// État : brouillon, modifiable.
#[derive(Debug)]
pub struct Brouillon;

/// État : en relecture chez `relecteur`.
#[derive(Debug)]
pub struct EnRelecture {
    relecteur: String,
}

/// État : publié, approuvé par `relecteur`.
#[derive(Debug)]
pub struct Publie {
    relecteur: String,
}

/// L'article typé.
///
/// Ce qui compile :
///
/// ```no_run
/// use ch12_typestate::e02_article::Article;
/// let publie = Article::nouveau("Typestate").rediger("Un type par état.").soumettre("Léa").approuver();
/// println!("{}", publie.lien());
/// ```
///
/// Publier un brouillon ne compile pas : `approuver` n'existe que sur `Article<EnRelecture>`.
///
/// ```compile_fail,E0599
/// use ch12_typestate::e02_article::Article;
/// let publie = Article::nouveau("Typestate").approuver();
/// ```
///
/// Modifier un article en relecture non plus : `rediger` n'existe que sur `Article<Brouillon>`.
///
/// ```compile_fail,E0599
/// use ch12_typestate::e02_article::Article;
/// let article = Article::nouveau("Typestate").soumettre("Léa").rediger("Ajout en douce.");
/// ```
///
/// Ni demander le lien d'un article pas encore publié.
///
/// ```compile_fail,E0599
/// use ch12_typestate::e02_article::Article;
/// let lien = Article::nouveau("Typestate").soumettre("Léa").lien();
/// ```
#[derive(Debug)]
pub struct Article<E> {
    titre: String,
    texte: String,
    etat: E,
}

impl<E> Article<E> {
    /// Le titre, dans tous les états.
    ///
    /// **Aide Rust** (ch00, sections 2, 5)
    /// - `self.titre` est une `String` qui reste dans l'article : on la prête. `E0308 … expected &str, found String`
    ///   propose `&self.titre`. Idem pour `texte`.
    pub fn titre(&self) -> &str {
        todo!()
    }

    /// Le texte, dans tous les états.
    pub fn texte(&self) -> &str {
        todo!()
    }
}

impl Article<Brouillon> {
    /// Un brouillon vide.
    ///
    /// **Aide Rust** (ch00, section 5)
    /// - Le champ `titre` est une `String`, le paramètre une `&str` : `E0308 … expected String, found &str`.
    pub fn nouveau(titre: &str) -> Self {
        todo!()
    }

    /// Ajoute `ajout` à la fin du texte.
    ///
    /// **Aide Rust** (ch00, sections 5, 7)
    /// - `self.texte.push_str(ajout)` donne
    ///   `E0596 cannot borrow self.texte as mutable, as self is not declared as mutable`. Fabriquez le nouveau texte
    ///   (`format!`, ou `String + &str`) dans un nouvel `Article`.
    /// - Ici le type ne change pas, donc `..self` est permis pour recopier les autres champs (toujours en dernier
    ///   dans les accolades).
    pub fn rediger(self, ajout: &str) -> Self {
        // But : le même brouillon, texte allongé.
        todo!()
    }

    /// Brouillon → en relecture chez `relecteur`.
    ///
    /// **Aide Rust** (ch00, sections 2, 5)
    /// - Le type change (`Brouillon` vers `EnRelecture`) : `..self` donne
    ///   `E0308 … expected Article<EnRelecture>, found Article<Brouillon>`. Listez les trois champs, en déplaçant
    ///   `self.titre` et `self.texte`.
    pub fn soumettre(self, relecteur: &str) -> Article<EnRelecture> {
        todo!()
    }
}

impl Article<EnRelecture> {
    /// Qui relit.
    ///
    /// **Aide Rust** (ch00, section 2)
    /// - Le relecteur est dans l'état : `self.etat.relecteur`. `self.relecteur` désigne la méthode elle-même :
    ///   `E0615 attempted to take value of method relecteur`.
    pub fn relecteur(&self) -> &str {
        todo!()
    }

    /// En relecture → brouillon (le relecteur est oublié : il faudra resoumettre).
    pub fn rejeter(self) -> Article<Brouillon> {
        todo!()
    }

    /// En relecture → publié.
    ///
    /// **Aide Rust** (ch00, sections 2, 6)
    /// - Une fois `self.titre` ou `self.texte` déplacé, `self` n'est plus entier : appeler `self.relecteur()` ensuite
    ///   donne `E0382 borrow of partially moved value: self`. Déplacez directement le champ `self.etat.relecteur`,
    ///   sans `to_string()` ni `clone()`.
    pub fn approuver(self) -> Article<Publie> {
        todo!()
    }
}

impl Article<Publie> {
    /// Qui a approuvé la publication.
    pub fn approuve_par(&self) -> &str {
        todo!()
    }

    /// `/articles/` suivi du titre en minuscules, mots (suites de caractères alphanumériques) joints par `-`.
    /// « Le typestate, en Rust ! » → `/articles/le-typestate-en-rust`.
    ///
    /// **Aide Rust** (ch00, sections 4, 12, 13)
    /// - `split(|c| ...)` donne `E0282 type annotations needed` : annotez le paramètre, `|c: char|`.
    /// - `join` n'existe pas sur un itérateur (`E0599 no method named join found for struct Filter`) : collectez
    ///   d'abord dans un `Vec`.
    /// - `let mots: Vec<&str> = self.titre.to_lowercase().split(...)...` donne
    ///   `E0716 temporary value dropped while borrowed` : les `&str` pointent dans la `String` temporaire. Gardez-la
    ///   dans un `let` (sans `mut`) d'abord.
    /// - Dans `filter(|m| ...)`, `m` est un `&&str` : `m != ""` donne `E0277 can't compare &str with str`.
    ///   `!m.is_empty()` évite la question.
    pub fn lien(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::style;

    #[test]
    fn parcours_complet() {
        let brouillon = Article::nouveau("Le typestate, en Rust !")
            .rediger("Un type ")
            .rediger("par état.");
        assert_eq!(brouillon.texte(), "Un type par état.");
        let relu = brouillon.soumettre("Léa");
        assert_eq!(relu.relecteur(), "Léa");
        let publie = relu.approuver();
        assert_eq!(publie.titre(), "Le typestate, en Rust !");
        assert_eq!(publie.approuve_par(), "Léa");
        assert_eq!(publie.lien(), "/articles/le-typestate-en-rust");
    }

    #[test]
    fn rejet_puis_nouvelle_relecture() {
        let publie = Article::nouveau("Élève modèle")
            .soumettre("Léa")
            .rejeter()
            .rediger("Corrigé.")
            .soumettre("Marc")
            .approuver();
        assert_eq!(publie.approuve_par(), "Marc");
        assert_eq!(publie.texte(), "Corrigé.");
        assert_eq!(publie.lien(), "/articles/élève-modèle");
    }

    #[test]
    fn le_naif_publie_un_brouillon() {
        let mut naif = ArticleNaif::creer("Bug");
        naif.envoyer_en_relecture("Léa").unwrap();
        naif.renvoyer_en_brouillon().unwrap();
        naif.ecrire("Texte jamais relu.").unwrap();
        assert!(naif.valider().is_ok(), "le bug de la version naïve");
        assert_eq!(naif.statut, "publié");
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e02_article.rs"),
            &[
                "titre",
                "texte",
                "nouveau",
                "rediger",
                "soumettre",
                "relecteur",
                "rejeter",
                "approuver",
                "approuve_par",
                "lien",
            ],
        );
    }
}
