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
    const INTERDITS: [&str; 5] = ["for", "while", "loop", "for_each", "try_for_each"];

    fn est_ident(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    /// Remplace les commentaires (`//`, `/* */` même imbriqués) par une espace, les chaînes (`"…"`, `b"…"`,
    /// `r#"…"#`) par `""` et les caractères (`'x'`, `'\n'`) par `' '`, sans toucher aux lifetimes (`'a`).
    ///
    /// Un analyseur caractère par caractère : ici, la boucle et l'état mutable sont le plus lisible.
    fn nettoyer(source: &str) -> String {
        let c: Vec<char> = source.chars().collect();
        let a = |k: usize| c.get(k).copied().unwrap_or('\0');
        let mut sortie = String::with_capacity(source.len());
        let mut i = 0;
        while i < c.len() {
            let debut_brut = a(i) == 'r'
                && (i == 0
                    || !est_ident(a(i - 1))
                    || (a(i - 1) == 'b' && (i < 2 || !est_ident(a(i - 2)))));
            let dieses = if debut_brut {
                c[i + 1..].iter().take_while(|&&x| x == '#').count()
            } else {
                0
            };
            if a(i) == '/' && a(i + 1) == '/' {
                while i < c.len() && a(i) != '\n' {
                    i += 1;
                }
            } else if a(i) == '/' && a(i + 1) == '*' {
                let mut profondeur = 0;
                while i < c.len() {
                    if a(i) == '/' && a(i + 1) == '*' {
                        profondeur += 1;
                        i += 2;
                    } else if a(i) == '*' && a(i + 1) == '/' {
                        profondeur -= 1;
                        i += 2;
                        if profondeur == 0 {
                            break;
                        }
                    } else {
                        i += 1;
                    }
                }
                sortie.push(' ');
            } else if debut_brut && a(i + 1 + dieses) == '"' {
                i += dieses + 2;
                while i < c.len() && !(a(i) == '"' && (1..=dieses).all(|k| a(i + k) == '#')) {
                    i += 1;
                }
                i += dieses + 1;
                sortie.push_str("\"\"");
            } else if a(i) == '"' {
                i += 1;
                while i < c.len() && a(i) != '"' {
                    i += if a(i) == '\\' { 2 } else { 1 };
                }
                i += 1;
                sortie.push_str("\"\"");
            } else if a(i) == '\'' && a(i + 1) == '\\' {
                i += 2;
                while i < c.len() && a(i) != '\'' {
                    i += 1;
                }
                i += 1;
                sortie.push_str("' '");
            } else if a(i) == '\'' && a(i + 2) == '\'' {
                i += 3;
                sortie.push_str("' '");
            } else {
                sortie.push(a(i));
                i += 1;
            }
        }
        sortie
    }

    /// Le source sans les modules de tests (`#[cfg(test)] mod … { … }`, où qu'ils soient dans le fichier). Un
    /// `#[cfg(test)]` posé sur autre chose qu'un module ne protège rien.
    fn sans_tests(source: &str) -> String {
        const ATTRIBUT: &str = "#[cfg(test)]";
        match source.find(ATTRIBUT) {
            None => source.to_string(),
            Some(debut) => {
                let suite = &source[debut + ATTRIBUT.len()..];
                if suite.trim_start().starts_with("mod ") {
                    let ouverture =
                        debut + ATTRIBUT.len() + suite.find('{').expect("module sans accolade");
                    let fin = ouverture + bloc(source, ouverture).len();
                    format!("{}{}", &source[..debut], sans_tests(&source[fin..]))
                } else {
                    format!("{}{}", &source[..debut + ATTRIBUT.len()], sans_tests(suite))
                }
            }
        }
    }

    /// Le bloc qui commence à l'accolade `ouverture`, accolades comprises.
    fn bloc(source: &str, ouverture: usize) -> &str {
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
        &source[ouverture..=fin]
    }

    /// Les fonctions et les `macro_rules!` du fichier, hors modules de tests, avec leur corps : `(nom, corps)`. Les
    /// déclarations sans corps (`fn nom(&self);` dans un `trait`) sont ignorées.
    fn fonctions(source: &str) -> Vec<(String, String)> {
        let code = sans_tests(source);
        let elements = |motif: &'static str, etiquette: fn(String) -> String| {
            code.match_indices(motif)
                .filter(|(i, _)| code[..*i].chars().last().is_none_or(|c| !est_ident(c)))
                .filter_map(|(i, _)| {
                    let apres = code[i + motif.len()..].trim_start();
                    let decalage = code.len() - apres.len();
                    let nom: String = apres.chars().take_while(|c| est_ident(*c)).collect();
                    let accolade = apres.find('{')?;
                    let point_virgule = apres.find(';').unwrap_or(usize::MAX);
                    (!nom.is_empty() && accolade < point_virgule)
                        .then(|| (etiquette(nom), bloc(&code, decalage + accolade).to_string()))
                })
                .collect::<Vec<_>>()
        };
        elements("fn ", |nom| nom)
            .into_iter()
            .chain(elements("macro_rules!", |nom| format!("{nom}!")))
            .collect()
    }

    /// Mots interdits trouvés dans un corps de fonction nettoyé.
    fn interdits_dans(corps: &str) -> Vec<String> {
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

    /// Corps (accolades comprises) de la fonction `nom` dans un source nettoyé.
    fn corps(source: &str, nom: &str) -> String {
        let signature = format!("fn {nom}(");
        let signature_generique = format!("fn {nom}<");
        let debut = source
            .find(&signature)
            .or_else(|| source.find(&signature_generique))
            .unwrap_or_else(|| panic!("fonction `{nom}` introuvable"));
        let ouverture = debut + source[debut..].find('{').expect("pas d'accolade ouvrante");
        bloc(source, ouverture).to_string()
    }

    /// Mots interdits trouvés dans le corps de `nom`.
    pub fn violations(source: &str, nom: &str) -> Vec<String> {
        interdits_dans(&corps(&sans_tests(&nettoyer(source)), nom))
    }

    /// Les fonctions d'aide (ni demandées, ni version de départ `_imperatif` / `_naif`) qui contiennent un mot
    /// interdit : `(nom, mots trouvés)`.
    pub fn violations_aides(source: &str, noms: &[&str]) -> Vec<(String, Vec<String>)> {
        fonctions(&nettoyer(source))
            .into_iter()
            .filter(|(nom, _)| {
                !noms.contains(&nom.as_str())
                    && !nom.ends_with("_imperatif")
                    && !nom.ends_with("_naif")
            })
            .map(|(nom, corps)| (nom, interdits_dans(&corps)))
            .filter(|(_, trouves)| !trouves.is_empty())
            .collect()
    }

    /// Échoue si le corps de l'une des fonctions `noms`, ou d'une fonction d'aide ajoutée dans le fichier, contient
    /// `let mut`, une boucle, `for_each` ou `try_for_each` (une `macro_rules!` compte comme une fonction d'aide).
    /// Seules les versions de départ (`_imperatif`, `_naif`) et les modules de tests y échappent.
    ///
    /// À appeler depuis un test avec `include_str!("le_fichier.rs")`.
    pub fn verifier(source: &str, noms: &[&str]) {
        let erreurs: Vec<String> = noms
            .iter()
            .map(|nom| (nom, violations(source, nom)))
            .filter(|(_, trouves)| !trouves.is_empty())
            .map(|(nom, trouves)| format!("`{nom}` contient {}", trouves.join(", ")))
            .chain(
                violations_aides(source, noms)
                    .into_iter()
                    .map(|(nom, trouves)| {
                        format!("la fonction d'aide `{nom}` contient {}", trouves.join(", "))
                    }),
            )
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

    const AVEC_AIDE: &str = r#"
pub trait Forme { fn aire(&self) -> f64; }
pub fn somme_imperatif(v: &[i64]) -> i64 { let mut s = 0; for x in v { s += x; } s }
fn aide(v: &[i64]) -> i64 { let mut s = 0; for x in v { s += x; } s }
fn propre(v: &[i64]) -> i64 { v.iter().sum() }
pub fn somme(v: &[i64]) -> i64 { aide(v) }
#[cfg(test)]
mod tests { fn compter() { let mut n = 0; loop { n += 1; } } }
"#;

    #[test]
    fn repere_les_boucles_cachees_dans_une_fonction_d_aide() {
        assert!(style::violations(AVEC_AIDE, "somme").is_empty());
        assert_eq!(
            style::violations_aides(AVEC_AIDE, &["somme"]),
            vec![(
                "aide".to_string(),
                vec!["`for`".to_string(), "`let mut`".to_string()]
            )]
        );
    }

    #[test]
    fn ne_se_trompe_pas_sur_caracteres_chaines_et_commentaires() {
        let source = "pub fn f<'a>(v: &'a [i64]) -> i64 {\n    let _a = '{';\n    let _b = '\"';\n    let _c = '\\'';\n    let _d = r#\"for \"x\" {\"#;\n    let _e = \"\\\" loop {\";\n    /* for /* while */ loop */\n    v.iter().sum()\n}\n";
        assert!(style::violations(source, "f").is_empty());
        assert!(style::violations_aides(source, &["f"]).is_empty());
    }

    #[test]
    fn repere_les_aides_apres_les_tests_les_macros_et_try_for_each() {
        let source = "pub fn f(v: &[i64]) -> i64 { v.iter().try_for_each(|_| Some(())); g(v) }\n#[cfg(test)]\nmod tests { fn t() { for _ in 0..1 {} } }\nfn g(v: &[i64]) -> i64 { let mut s = 0; for x in v { s += x } s }\nmacro_rules! m { () => { loop {} }; }\n#[cfg(test)]\nfn h() { while true {} }\n";
        assert_eq!(style::violations(source, "f"), vec!["`try_for_each`"]);
        let aides: Vec<String> = style::violations_aides(source, &["f"])
            .into_iter()
            .map(|(nom, _)| nom)
            .collect();
        assert_eq!(aides, vec!["g", "h", "m!"]);
    }

    #[test]
    #[should_panic(expected = "la fonction d'aide `aide` contient")]
    fn verifier_echoue_sur_une_fonction_d_aide_imperative() {
        style::verifier(AVEC_AIDE, &["somme"]);
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
