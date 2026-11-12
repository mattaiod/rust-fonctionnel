//! # Exercice 1 : une fonction générique
//!
//! **À faire** : écrire `plus_grand`, qui rend le plus grand élément d'une liste, `None` si elle est vide. Elle doit
//! marcher pour des `i32`, des `f64`, des `char`… : c'est le rôle du `<T>`.
//!
//! **Ce qu'il faut savoir**
//! - `T: PartialOrd` est le **bound** : il permet d'écrire `a > b` sur deux `T`.
//! - `T: Copy` permet de rendre une copie de l'élément plutôt qu'une référence.
//! - Outil : `liste.iter().copied().reduce(|a, b| if a > b { a } else { b })`.
//!
//! Test : `cargo test -p ch15-traits e01`

/// Le plus grand élément, `None` si la liste est vide.
pub fn plus_grand<T: PartialOrd + Copy>(liste: &[T]) -> Option<T> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plusieurs_types() {
        assert_eq!(plus_grand(&[3, 9, 2]), Some(9));
        assert_eq!(plus_grand(&[21.5, 19.8, 22.0]), Some(22.0));
        assert_eq!(plus_grand(&['a', 'z', 'm']), Some('z'));
    }

    #[test]
    fn liste_vide() {
        assert_eq!(plus_grand::<i32>(&[]), None);
    }
}
