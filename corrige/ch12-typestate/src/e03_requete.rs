//! Exercice 3 : un builder qui refuse d'envoyer une requête sans URL.
//!
//! Un builder est un objet qu'on règle méthode après méthode (`.methode(…).url(…).entete(…)`) avant de
//! produire la valeur finale. Le builder classique remplit des `Option` et vérifie tout dans `construire` :
//! l'oubli de l'URL est un `panic!` (ou un `Result` à traiter) à l'exécution. Ici le paramètre de type dit si
//! l'URL est fournie : `RequeteBuilder<SansUrl>` puis `RequeteBuilder<AvecUrl>`, et `envoyer` n'existe que sur
//! le second. Les réglages facultatifs (méthode, en-têtes, corps) sont dans `impl<U> RequeteBuilder<U>` :
//! disponibles avant comme après l'URL, ils rendent `Self` sans changer d'état.

/// Méthode HTTP.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Methode {
    #[default]
    Get,
    Post,
    Put,
    Delete,
}

/// La requête prête à partir (ici on ne l'envoie pas vraiment : on la rend).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Requete {
    pub methode: Methode,
    pub url: String,
    pub entetes: Vec<(String, String)>,
    pub corps: Option<String>,
}

/// Version naïve : tout est facultatif, l'URL aussi, jusqu'au `panic!` de `construire`. Ne pas modifier.
#[derive(Debug, Default)]
pub struct ConstructeurNaif {
    methode: Methode,
    url: Option<String>,
    entetes: Vec<(String, String)>,
    corps: Option<String>,
}

impl ConstructeurNaif {
    /// Ne pas modifier.
    pub fn avec_methode(&mut self, methode: Methode) -> &mut Self {
        self.methode = methode;
        self
    }

    /// Ne pas modifier.
    pub fn avec_url(&mut self, url: &str) -> &mut Self {
        self.url = Some(url.to_string());
        self
    }

    /// Ne pas modifier.
    pub fn avec_entete(&mut self, nom: &str, valeur: &str) -> &mut Self {
        self.entetes.push((nom.to_string(), valeur.to_string()));
        self
    }

    /// Ne pas modifier.
    pub fn avec_corps(&mut self, corps: &str) -> &mut Self {
        self.corps = Some(corps.to_string());
        self
    }

    /// Panique si l'URL a été oubliée. Ne pas modifier.
    pub fn construire(&self) -> Requete {
        Requete {
            methode: self.methode,
            url: self.url.clone().expect("url manquante"),
            entetes: self.entetes.clone(),
            corps: self.corps.clone(),
        }
    }
}

/// État : pas encore d'URL.
#[derive(Debug)]
pub struct SansUrl;

/// État : l'URL est connue.
#[derive(Debug)]
pub struct AvecUrl(String);

/// Le builder typé. Par défaut : `GET`, aucun en-tête, pas de corps.
///
/// Ce qui compile :
///
/// ```no_run
/// use ch12_typestate::e03_requete::{Methode, RequeteBuilder};
/// let requete = RequeteBuilder::nouveau()
///     .methode(Methode::Post)
///     .url("https://arena.example/api/soumissions")
///     .entete("Content-Type", "text/csv")
///     .envoyer();
/// ```
///
/// Envoyer sans URL ne compile pas : `envoyer` n'existe que sur `RequeteBuilder<AvecUrl>`.
///
/// ```compile_fail,E0599
/// use ch12_typestate::e03_requete::RequeteBuilder;
/// let requete = RequeteBuilder::nouveau().entete("Accept", "text/csv").envoyer();
/// ```
///
/// Donner l'URL deux fois non plus : `url` n'existe que sur `RequeteBuilder<SansUrl>`.
///
/// ```compile_fail,E0599
/// use ch12_typestate::e03_requete::RequeteBuilder;
/// let builder = RequeteBuilder::nouveau().url("https://a.example").url("https://b.example");
/// ```
#[derive(Debug)]
pub struct RequeteBuilder<U> {
    methode: Methode,
    entetes: Vec<(String, String)>,
    corps: Option<String>,
    url: U,
}

impl RequeteBuilder<SansUrl> {
    /// Un builder vide : `GET`, sans URL.
    ///
    /// **Aide Rust** (ch00, section 1)
    /// - `Methode::default()` vaut `Methode::Get` : c'est la variante marquée `#[default]` dans
    ///   `#[derive(Default)]` (le trait `Default` est détaillé au chapitre 13). Écrire `Methode::Get` revient au même.
    /// - Le champ `url` est un état, pas une `Option` : `url: None` donne
    ///   `E0308 … expected SansUrl, found Option<_>`.
    pub fn nouveau() -> Self {
        // L'absence d'URL est la valeur `SansUrl`, pas un `None` : elle est dans le type, le compilateur la voit.
        RequeteBuilder {
            methode: Methode::default(),
            entetes: Vec::new(),
            corps: None,
            url: SansUrl,
        }
    }

    /// Fixe l'URL : change d'état.
    ///
    /// **Aide Rust** (ch00, sections 2, 5)
    /// - Le type change : `..self` donne `E0308 … expected RequeteBuilder<AvecUrl>, found RequeteBuilder<SansUrl>`.
    ///   Listez les champs.
    /// - `AvecUrl` possède une `String` : `AvecUrl(url)` donne `E0308 … expected String, found &str`.
    pub fn url(self, url: &str) -> RequeteBuilder<AvecUrl> {
        // But : tout garder, sauf l'état.
        // Transition : `self` consommé, builder reconstruit champ par champ, seul `url` change de type.
        // `url` n'existe que sur `RequeteBuilder<SansUrl>` : la donner deux fois ne compile pas.
        RequeteBuilder {
            methode: self.methode,
            entetes: self.entetes,
            corps: self.corps,
            url: AvecUrl(url.to_string()),
        }
    }
}

impl<U> RequeteBuilder<U> {
    /// Remplace la méthode.
    ///
    /// **Aide Rust** (ch00, section 7)
    /// - `self.methode = methode;` donne `E0594 cannot assign to self.methode, as self is not declared as mutable`.
    ///   Le type ne change pas : un nouveau `RequeteBuilder { methode, ..self }` fait l'affaire (`..self` toujours en
    ///   dernier, sans virgule après : `cannot use a comma after the base struct`).
    pub fn methode(self, methode: Methode) -> Self {
        // Dans `impl<U>`, le réglage marche avant comme après l'URL et rend le même état.
        // Le type ne change pas, donc `..self` est permis.
        RequeteBuilder { methode, ..self }
    }

    /// Ajoute un en-tête à la fin de la liste.
    ///
    /// **Aide Rust** (ch00, sections 3, 7)
    /// - Sans `push` : le nouveau `Vec` est l'ancien suivi d'un élément. `std::iter::once(x)` est un itérateur d'un
    ///   seul élément, `a.chain(b)` parcourt `a` puis `b` (chapitre 6).
    /// - `self.entetes.iter()` donne des `&(String, String)`, le nouvel en-tête est un `(String, String)` :
    ///   `E0271 type mismatch resolving … expected &(String, String), found (String, String)`. `self` vous
    ///   appartient : `into_iter()`.
    pub fn entete(self, nom: &str, valeur: &str) -> Self {
        // But : un nouveau builder, sans `push` sur l'ancien.
        // `into_iter` déplace les en-têtes (aucun clone), `chain(once(..))` ajoute le nouveau à la fin.
        // `..self` ne reprend que les champs non cités : `entetes` est déjà déplacé, le reste suit.
        RequeteBuilder {
            entetes: self
                .entetes
                .into_iter()
                .chain(std::iter::once((nom.to_string(), valeur.to_string())))
                .collect(),
            ..self
        }
    }

    /// Remplace le corps.
    ///
    /// **Aide Rust** (ch00, section 5)
    /// - `Some(corps)` donne `E0308 … expected String, found &str` : le champ est une `Option<String>`.
    pub fn corps(self, corps: &str) -> Self {
        // Même schéma que `methode`.
        RequeteBuilder {
            corps: Some(corps.to_string()),
            ..self
        }
    }
}

impl RequeteBuilder<AvecUrl> {
    /// Rend la requête. Ne peut pas échouer : l'URL est là, le type le garantit.
    ///
    /// **Aide Rust** (ch00, section 2)
    /// - `url: self.url` donne `E0308 … expected String, found AvecUrl` : la `String` est dans le champ `.0` du tuple
    ///   struct `AvecUrl`.
    pub fn envoyer(self) -> Requete {
        // Aucun `expect` : sur `RequeteBuilder<AvecUrl>`, l'URL est forcément là. `.0` sort la `String` de `AvecUrl`.
        Requete {
            methode: self.methode,
            url: self.url.0,
            entetes: self.entetes,
            corps: self.corps,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::style;

    #[test]
    fn valeurs_par_defaut() {
        assert_eq!(
            RequeteBuilder::nouveau().url("https://a.example").envoyer(),
            Requete {
                methode: Methode::Get,
                url: "https://a.example".to_string(),
                entetes: vec![],
                corps: None
            }
        );
    }

    #[test]
    fn reglages_avant_et_apres_l_url() {
        let requete = RequeteBuilder::nouveau()
            .entete("Accept", "text/csv")
            .methode(Methode::Put)
            .url("https://a.example/x")
            .entete("X-Equipe", "lambda")
            .corps("id,label\n1,0")
            .methode(Methode::Post)
            .envoyer();
        assert_eq!(requete.methode, Methode::Post);
        assert_eq!(
            requete.entetes,
            vec![
                ("Accept".to_string(), "text/csv".to_string()),
                ("X-Equipe".to_string(), "lambda".to_string())
            ]
        );
        assert_eq!(requete.corps.as_deref(), Some("id,label\n1,0"));
    }

    #[test]
    fn meme_resultat_que_le_naif() {
        let naive = ConstructeurNaif::default()
            .avec_methode(Methode::Delete)
            .avec_url("https://a.example/1")
            .avec_entete("Authorization", "jeton")
            .construire();
        let typee = RequeteBuilder::nouveau()
            .methode(Methode::Delete)
            .url("https://a.example/1")
            .entete("Authorization", "jeton")
            .envoyer();
        assert_eq!(typee, naive);
    }

    #[test]
    #[should_panic(expected = "url manquante")]
    fn le_naif_panique_sans_url() {
        ConstructeurNaif::default().avec_corps("{}").construire();
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e03_requete.rs"),
            &["nouveau", "url", "methode", "entete", "corps", "envoyer"],
        );
    }
}
