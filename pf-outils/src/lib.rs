//! Outils partagés par les chapitres : un tri pur, la vérification du style fonctionnel et des jeux de données
//! reproductibles pour les tests. Rien à modifier ici.

use std::cmp::Ordering;

/// Trie une collection et rend un nouveau `Vec`, comme `sorted` en Python.
///
/// `sort_by` demande un `Vec` mutable : la mutation reste enfermée ici. Vue de l'extérieur, la fonction est pure
/// (mêmes entrées, même sortie, rien de modifié chez l'appelant). C'est ainsi qu'on fait de la PF en Rust.
pub fn trier_par<T>(
    elements: impl IntoIterator<Item = T>,
    comparer: impl FnMut(&T, &T) -> Ordering,
) -> Vec<T> {
    let mut tries: Vec<T> = elements.into_iter().collect();
    tries.sort_by(comparer);
    tries
}

/// Vérification du style fonctionnel : relit le source d'un exercice et y cherche les mots interdits.
pub mod style {
    const INTERDITS: [&str; 4] = ["for", "while", "loop", "for_each"];

    /// Retire les commentaires `//` et le contenu des chaînes `"…"`.
    fn nettoyer(source: &str) -> String {
        source
            .lines()
            .map(|ligne| ligne.split("//").next().unwrap_or(""))
            .map(|ligne| {
                ligne
                    .split('"')
                    .enumerate()
                    .filter(|(i, _)| i % 2 == 0)
                    .map(|(_, morceau)| morceau)
                    .collect::<Vec<_>>()
                    .join("\"\"")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Corps (accolades comprises) de la fonction `nom` dans un source nettoyé.
    fn corps(source: &str, nom: &str) -> String {
        let signature = format!("fn {nom}(");
        let signature_generique = format!("fn {nom}<");
        let debut = source
            .find(&signature)
            .or_else(|| source.find(&signature_generique))
            .unwrap_or_else(|| panic!("fonction `{nom}` introuvable"));
        let ouverture = debut + source[debut..].find('{').expect("pas d'accolade ouvrante");
        let fin = source[ouverture..]
            .char_indices()
            .scan(0_i32, |profondeur, (i, c)| {
                *profondeur += match c {
                    '{' => 1,
                    '}' => -1,
                    _ => 0,
                };
                Some((i, *profondeur))
            })
            .find(|(_, profondeur)| *profondeur == 0)
            .map(|(i, _)| ouverture + i)
            .expect("accolades déséquilibrées");
        source[ouverture..=fin].to_string()
    }

    /// Mots interdits trouvés dans le corps de `nom`.
    pub fn violations(source: &str, nom: &str) -> Vec<String> {
        let corps = corps(&nettoyer(source), nom);
        let mots: Vec<&str> = corps
            .split(|c: char| !(c.is_alphanumeric() || c == '_'))
            .filter(|mot| !mot.is_empty())
            .collect();
        INTERDITS
            .iter()
            .filter(|interdit| mots.contains(interdit))
            .map(|interdit| format!("`{interdit}`"))
            .chain(
                mots.windows(2)
                    .any(|paire| paire == ["let", "mut"])
                    .then(|| "`let mut`".to_string()),
            )
            .collect()
    }

    /// Échoue si le corps de l'une des fonctions `noms` contient `let mut`, une boucle ou `for_each`.
    ///
    /// À appeler depuis un test avec `include_str!("le_fichier.rs")`.
    pub fn verifier(source: &str, noms: &[&str]) {
        let erreurs: Vec<String> = noms
            .iter()
            .map(|nom| (nom, violations(source, nom)))
            .filter(|(_, trouves)| !trouves.is_empty())
            .map(|(nom, trouves)| format!("`{nom}` contient {}", trouves.join(", ")))
            .collect();
        assert!(
            erreurs.is_empty(),
            "Pas encore fonctionnel : {}",
            erreurs.join(" ; ")
        );
    }
}

/// Jeux de données pseudo-aléatoires et reproductibles, pour comparer deux versions d'une même fonction.
pub mod jeux {
    /// Suite infinie de nombres pseudo-aléatoires (générateur congruentiel linéaire) à partir de `graine`.
    fn suite(graine: u64) -> impl Iterator<Item = u64> {
        std::iter::successors(Some(graine), |x| {
            Some(
                x.wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1_442_695_040_888_963_407),
            )
        })
        .skip(1)
        .map(|x| x >> 33)
    }

    /// `taille` entiers entre `min` et `max` inclus.
    pub fn entiers(graine: u64, taille: usize, min: i64, max: i64) -> Vec<i64> {
        let etendue = (max - min + 1) as u64;
        suite(graine)
            .take(taille)
            .map(|x| min + (x % etendue) as i64)
            .collect()
    }

    /// Vingt jeux d'entiers de tailles variées (0 à 49 éléments), dont le jeu vide.
    pub fn plusieurs(min: i64, max: i64) -> Vec<Vec<i64>> {
        (0..20_u64)
            .map(|graine| entiers(graine, (graine as usize * 7) % 50, min, max))
            .collect()
    }

    /// `taille` mots de 1 à 8 lettres minuscules tirées parmi les `alphabet` premières lettres.
    pub fn mots(graine: u64, taille: usize, alphabet: u8) -> Vec<String> {
        let nombres: Vec<u64> = suite(graine).take(taille * 9).collect();
        nombres
            .chunks(9)
            .map(|bloc| {
                let longueur = 1 + (bloc[0] % 8) as usize;
                bloc[1..=longueur]
                    .iter()
                    .map(|n| (b'a' + (n % alphabet as u64) as u8) as char)
                    .collect()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE: &str = r#"
pub fn imperatif(v: &[i64]) -> i64 {
    let mut s = 0; // for dans un commentaire : ignoré
    for x in v { s += x; }
    while false {}
    s
}
pub fn fonctionnel(v: &[i64]) -> i64 {
    // let mut dans un commentaire : ignoré
    let message = "for while loop";
    v.iter().fold(0, |mut acc, x| { acc += x; acc })
}
pub fn deguise(v: &[i64]) { v.iter().for_each(|x| println!("{x}")) }
pub fn generique<T: Clone>(v: &[T]) -> Vec<T> { { v.to_vec() } }
"#;

    #[test]
    fn repere_les_boucles_et_let_mut() {
        assert_eq!(
            style::violations(SOURCE, "imperatif"),
            vec!["`for`", "`while`", "`let mut`"]
        );
        assert_eq!(style::violations(SOURCE, "deguise"), vec!["`for_each`"]);
    }

    #[test]
    fn ignore_commentaires_chaines_et_mut_de_closure() {
        assert!(style::violations(SOURCE, "fonctionnel").is_empty());
        assert!(style::violations(SOURCE, "generique").is_empty());
    }

    #[test]
    fn trier_par_ne_touche_pas_l_original() {
        let original = vec![3, 1, 2];
        assert_eq!(trier_par(original.clone(), |a, b| b.cmp(a)), vec![3, 2, 1]);
        assert_eq!(original, vec![3, 1, 2]);
    }

    #[test]
    fn jeux_reproductibles() {
        assert_eq!(jeux::entiers(7, 10, -5, 5), jeux::entiers(7, 10, -5, 5));
        assert!(
            jeux::entiers(7, 100, -5, 5)
                .iter()
                .all(|n| (-5..=5).contains(n))
        );
        assert_eq!(jeux::mots(3, 4, 5).len(), 4);
    }
}
