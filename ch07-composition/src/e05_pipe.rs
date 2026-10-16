//! Exercice 5 : une fonction `pipe` pour du texte, sans version impérative : à vous.
//!
//! En F#, en Elixir ou avec la proposition `|>` en JavaScript, on écrit `texte |> f |> g |> h`. En Rust, la chaîne
//! de méthodes joue ce rôle quand les étapes sont des méthodes. Quand les étapes sont des **valeurs** (une liste
//! choisie à l'exécution, une configuration), on écrit `pipe` soi-même : faire passer un texte dans une liste de
//! transformations, dans l'ordre. Le type `&[&dyn Fn(String) -> String]` accepte aussi bien des fonctions nommées
//! que des closures qui capturent quelque chose. `&dyn Fn` joue le même rôle que `Box<dyn Fn>` (un type commun à
//! des fonctions de types différents), mais emprunté : pas d'allocation, les fonctions restent à l'appelant.
//! Écrivez aussi deux transformations nommées et une fabrique de transformation (application partielle, exercice 2).

/// Fait passer `depart` dans chaque étape, dans l'ordre ; sans étape, rend `depart` tel quel.
///
/// **Aide Rust** (ch00, sections 9, 12)
/// - Chaque étape prend la `String` **par valeur** et en rend une nouvelle : le `fold` la transporte d'une étape à la
///   suivante.
/// - Si le compilateur dit `` error[E0308]: mismatched types … expected `String`, found `&String` `` : passez `texte` à
///   l'étape, pas `&texte`.
pub fn pipe(depart: String, etapes: &[&dyn Fn(String) -> String]) -> String {
    todo!()
}

/// Le texte en majuscules.
pub fn majuscules(texte: String) -> String {
    todo!()
}

/// Les blancs du début et de la fin retirés, et chaque suite de blancs intérieure remplacée par une seule espace.
///
/// **Aide Rust** (ch00, sections 5, 12)
/// - Si le compilateur dit `` expected `String`, found `&str` `` : `trim()` et les morceaux de `split_whitespace()`
///   sont des `&str` ; il faut fabriquer une `String` à la fin.
/// - Si le compilateur dit `error[E0282]: type annotations needed` sur un `collect()` suivi de `.join(…)` : précisez
///   `collect::<Vec<_>>()`. (`collect::<String>()` collerait les mots sans espace.)
pub fn espaces_simples(texte: String) -> String {
    todo!()
}

/// La transformation « entourer le texte de `bord` des deux côtés ».
pub fn encadrer(bord: char) -> impl Fn(String) -> String {
    move |texte| todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::style;

    #[test]
    fn transformations_seules() {
        assert_eq!(majuscules("élève".to_string()), "ÉLÈVE");
        assert_eq!(espaces_simples("  a \t b\n\nc ".to_string()), "a b c");
        assert_eq!(encadrer('*')("ok".to_string()), "*ok*");
    }

    #[test]
    fn sans_etape() {
        assert_eq!(pipe("tel quel".to_string(), &[]), "tel quel");
    }

    #[test]
    fn cas_connus() {
        let texte = "  bonjour   le monde ".to_string();
        assert_eq!(
            pipe(
                texte.clone(),
                &[&espaces_simples, &majuscules, &encadrer('*')]
            ),
            "*BONJOUR LE MONDE*"
        );
        // L'ordre compte : encadrer d'abord, et les bords restent séparés du texte par une espace.
        assert_eq!(
            pipe(texte, &[&encadrer('|'), &espaces_simples]),
            "| bonjour le monde |"
        );
    }

    #[test]
    fn accepte_une_closure_qui_capture() {
        let prefixe = String::from("> ");
        let citer = |texte: String| format!("{prefixe}{texte}");
        assert_eq!(pipe("salut".to_string(), &[&citer, &majuscules]), "> SALUT");
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e05_pipe.rs"),
            &["pipe", "majuscules", "espaces_simples", "encadrer"],
        );
    }
}
