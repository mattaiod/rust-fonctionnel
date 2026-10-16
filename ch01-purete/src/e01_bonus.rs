//! Exercice 1 : ajouter un bonus aux notes, sans toucher aux notes d'origine.
//!
//! La version impérative reçoit `&mut [i64]` et écrase les notes de l'appelant : après l'appel, les notes brutes
//! sont perdues. La version pure reçoit `&[i64]` (lecture seule, le `borrow checker` le garantit) et rend un
//! NOUVEAU `Vec`. La signature suffit à savoir qui modifie quoi. Une note ne dépasse jamais 20 : `min(20)`.

/// Version impérative : modifie les notes en place. Ne pas modifier.
pub fn ajouter_bonus_imperatif(notes: &mut [i64], bonus: i64) {
    for note in notes.iter_mut() {
        *note = (*note + bonus).min(20);
    }
}

/// Version pure : rend les nouvelles notes, plafonnées à 20, et laisse `notes` intact.
///
/// **Aide Rust** (ch00, sections 2, 3, 12)
/// - `notes.iter()` donne des `&i64` : des références en lecture seule. On ne les modifie pas, on calcule une
///   nouvelle valeur dans le `map` (`&i64 + i64` marche et donne un `i64`).
/// - Si le compilateur dit ``E0594: cannot assign to `*note`, which is behind a `&` reference`` : vous essayez
///   d'écrire dans la note d'origine. Rendez la nouvelle note au lieu de l'écrire.
/// - Si le compilateur dit ``E0308: expected `&i64`, found integer`` sur `note.min(20)` : `note` est une référence.
///   Faites d'abord `(note + bonus)`, qui est un `i64`, puis `.min(20)`.
/// - Si le compilateur dit ``E0308: expected `Vec<i64>`, found `Map<…>` `` : il manque `.collect()` au bout ; un
///   `map` seul ne calcule rien.
pub fn ajouter_bonus(notes: &[i64], bonus: i64) -> Vec<i64> {
    // Outils : `iter()`, `map(|note| …)`, `min(20)`, `collect()`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    #[test]
    fn cas_connus() {
        assert_eq!(ajouter_bonus(&[10, 15, 19], 2), vec![12, 17, 20]);
        assert_eq!(ajouter_bonus(&[20], 0), vec![20]);
        assert_eq!(ajouter_bonus(&[], 3), Vec::<i64>::new());
    }

    #[test]
    fn ne_touche_pas_l_original() {
        let notes = vec![8, 12];
        let avec_bonus = ajouter_bonus(&notes, 1);
        assert_eq!(notes, vec![8, 12]);
        assert_eq!(avec_bonus, vec![9, 13]);
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        jeux::plusieurs(0, 20).iter().for_each(|jeu| {
            (0..4).for_each(|bonus| {
                let copie = jeu.clone();
                let modifiee = {
                    let mut notes = jeu.clone();
                    ajouter_bonus_imperatif(&mut notes, bonus);
                    notes
                };
                assert_eq!(ajouter_bonus(&copie, bonus), modifiee, "jeu : {jeu:?}");
            })
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(include_str!("e01_bonus.rs"), &["ajouter_bonus"]);
    }
}
