//! Exercice 1 : une erreur qu'on peut trier : de `String` à `enum`.
//!
//! Une erreur de type `String` ne sert qu'à être affichée. Dès que l'appelant veut réagir selon la cause
//! (« une note vide vaut 0, le reste est une vraie erreur »), il doit comparer des phrases : la moindre
//! retouche du message casse son code, sans que le compilateur ne dise rien. Un `enum` d'erreur donne une
//! variante par cause, avec ses données, et l'appelant fait un `match` que le compilateur vérifie.

/// Version naïve : l'erreur est une phrase. Ne pas modifier.
pub fn parser_note_naif(texte: &str) -> Result<u8, String> {
    let texte = texte.trim();
    if texte.is_empty() {
        return Err("note vide".to_string());
    }
    let note: u32 = match texte.parse() {
        Ok(n) => n,
        Err(_) => return Err(format!("« {texte} » n'est pas un nombre")),
    };
    if note > 20 {
        return Err(format!("{note} dépasse 20"));
    }
    Ok(note as u8)
}

/// Version naïve : pour distinguer la note vide, on compare le message. Si quelqu'un écrit un jour
/// « Note vide » avec une majuscule, cette fonction change de comportement en silence. Ne pas modifier.
pub fn note_ou_zero_naif(texte: &str) -> Result<u8, String> {
    match parser_note_naif(texte) {
        Err(message) if message == "note vide" => Ok(0),
        autre => autre,
    }
}

/// Les causes d'échec de la lecture d'une note sur 20.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurNote {
    /// Rien à lire (chaîne vide ou blanche).
    Vide,
    /// Le texte (sans les blancs autour) n'est pas un entier positif.
    PasUnNombre(String),
    /// Un entier, mais au-dessus de 20.
    Depasse20(u32),
}

/// Lit une note entière sur 20 (blancs autour tolérés).
///
/// **Aide Rust** (ch00, sections 1, 5, 12)
/// - `parse` échoue avec une `ParseIntError`, pas une `ErreurNote` : un `?` direct donne
///   `E0271 type mismatch resolving <u32 as FromStr>::Err == ErreurNote`. Traduisez l'erreur avec `map_err(|_| ...)`
///   avant le `?`.
/// - `ErreurNote::PasUnNombre(texte)` donne `E0308 … expected String, found &str` : la variante possède son texte
///   (`to_string()`).
/// - La note lue est un `u32`, la fonction rend un `u8` : `Ok(note)` donne `E0308 … expected u8, found u32`. Après le
///   contrôle `<= 20`, la conversion ne perd rien.
pub fn parser_note(texte: &str) -> Result<u8, ErreurNote> {
    // But : les mêmes règles que `parser_note_naif`, une variante d'`ErreurNote` par cause.
    todo!()
}

/// Comme `parser_note`, mais une note vide vaut 0. Les autres erreurs remontent telles quelles.
///
/// **Aide Rust** (ch00, section 1)
/// - Un `match` doit couvrir tous les cas : avec seulement `Err(ErreurNote::Vide)` et `Ok(n)`, le compilateur dit
///   `E0004 non-exhaustive patterns: Err(ErreurNote::PasUnNombre(_)) and Err(ErreurNote::Depasse20(_)) not covered`.
///   Un dernier bras qui attrape tout le reste et le rend tel quel suffit.
/// - Tous les bras rendent le même type, un `Result` : `=> 0` donne
///   `E0308 … expected Result<u8, ErreurNote>, found integer`.
pub fn note_ou_zero(texte: &str) -> Result<u8, ErreurNote> {
    // But : réagir à UNE variante, laisser passer les autres. Aucune comparaison de chaînes.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    #[test]
    fn cas_connus() {
        assert_eq!(parser_note("12"), Ok(12));
        assert_eq!(parser_note("  20 "), Ok(20));
        assert_eq!(parser_note("0"), Ok(0));
        assert_eq!(parser_note(""), Err(ErreurNote::Vide));
        assert_eq!(parser_note("   "), Err(ErreurNote::Vide));
        assert_eq!(
            parser_note(" douze "),
            Err(ErreurNote::PasUnNombre("douze".to_string()))
        );
        assert_eq!(
            parser_note("-3"),
            Err(ErreurNote::PasUnNombre("-3".to_string()))
        );
        assert_eq!(parser_note("21"), Err(ErreurNote::Depasse20(21)));
        assert_eq!(parser_note("300"), Err(ErreurNote::Depasse20(300)));
    }

    #[test]
    fn note_vide_vaut_zero() {
        assert_eq!(note_ou_zero(""), Ok(0));
        assert_eq!(note_ou_zero("15"), Ok(15));
        assert_eq!(note_ou_zero("25"), Err(ErreurNote::Depasse20(25)));
    }

    #[test]
    fn meme_resultat_que_le_naif() {
        let textes: Vec<String> = (-5..=40)
            .map(|n| n.to_string())
            .chain(jeux::mots(1, 20, 4))
            .chain(["", " ", " 7 ", "1e1", "99999999999"].map(String::from))
            .collect();
        textes.iter().for_each(|t| {
            assert_eq!(
                parser_note(t).ok(),
                parser_note_naif(t).ok(),
                "texte : {t:?}"
            );
            assert_eq!(
                note_ou_zero(t).ok(),
                note_ou_zero_naif(t).ok(),
                "texte : {t:?}"
            );
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e01_note.rs"),
            &["parser_note", "note_ou_zero"],
        );
    }
}
