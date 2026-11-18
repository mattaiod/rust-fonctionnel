//! Exercice 5 : `NonVide<T>` : une liste qui a toujours un premier élément.
//!
//! `v[0]` panique sur une liste vide, `v.first()` rend une `Option` qu'il faut traiter partout. Si la liste ne peut
//! **pas** être vide, disons-le dans le type : un premier élément obligatoire, plus un reste (peut-être vide). Alors
//! `premier()` rend un `&T` et `maximum()` aussi, sans `Option` ni `unwrap`. La vérification « non vide » a lieu une
//! seule fois, à la frontière (`try_from` d'un `Vec`).
//!
//! Note sur `mut` : le paramètre `mut elements` de `depuis_iter` est une mutation **locale** d'une valeur possédée
//! (l'itérateur appartient à la fonction), invisible de l'appelant. C'est permis ; `let mut` dans le corps reste
//! interdit dans ces exercices. Tant que le corps est un `todo!()`, le compilateur signale ce `mut` inutilisé.

/// Version naïve : le premier élément. Panique sur une liste vide. Ne pas modifier.
pub fn premier_naif(valeurs: &[i64]) -> i64 {
    valeurs[0]
}

/// Version naïve : le maximum. Panique sur une liste vide. Ne pas modifier.
pub fn maximum_naif(valeurs: &[i64]) -> i64 {
    *valeurs.iter().max().unwrap()
}

/// Une liste d'au moins un élément. Champs privés : la seule façon d'en obtenir une vide serait... qu'il n'y en ait
/// pas.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonVide<T> {
    premier: T,
    reste: Vec<T>,
}

/// L'erreur : le `Vec` était vide.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VecVide;

impl<T> NonVide<T> {
    /// Une liste à partir de son premier élément et du reste : ne peut pas échouer.
    pub fn new(premier: T, reste: Vec<T>) -> Self {
        NonVide { premier, reste }
    }

    /// Les éléments d'un itérateur, `None` s'il est vide.
    ///
    /// **Aide Rust** (ch00, sections 3, 7)
    /// - Ne passez pas par un `Vec` puis `v[0]` : sortir un `T` d'un `Vec` par indice est interdit, et
    ///   `v[1..].to_vec()` exige `T: Clone` (`E0277 the trait bound T: Clone is not satisfied`). Un premier `next()`
    ///   prend le premier élément, `collect()` prend ce qui reste.
    pub fn depuis_iter(mut elements: impl Iterator<Item = T>) -> Option<Self> {
        // But : le premier `next()` décide de tout ; le reste se `collect`.
        // `next()` consomme le premier élément ; s'il manque, `map` rend `None` et la fonction s'arrête là.
        // Sinon `collect` prend la suite de l'itérateur, déjà avancé d'un cran. Aucun `Clone` nécessaire.
        elements.next().map(|premier| NonVide {
            premier,
            reste: elements.collect(),
        })
    }

    /// Le premier élément : jamais d'échec.
    ///
    /// **Aide Rust** (ch00, section 2)
    /// - `self.premier` est un `T` ; on rend un `&T` : `E0308 … expected &T, found type parameter T`. Prêtez-le.
    pub fn premier(&self) -> &T {
        // But : aucune `Option`.
        // Le champ `premier` existe toujours : c'est la forme de la `struct` qui garantit le non-vide.
        &self.premier
    }

    /// Le nombre d'éléments, au moins 1.
    pub fn taille(&self) -> usize {
        // But : une addition.
        1 + self.reste.len()
    }

    /// Tous les éléments, dans l'ordre.
    ///
    /// `Box<dyn Iterator>` plutôt que `impl Iterator` : un seul type de retour, quel que soit le corps écrit. Le
    /// `+ '_` dit que l'itérateur emprunte `self` : il ne doit pas lui survivre (ch00, section 13).
    ///
    /// **Aide Rust** (ch00, sections 3, 10)
    /// - Le type de retour est une `Box` : sans `Box::new(...)`,
    ///   `E0308 … expected Box<dyn Iterator<Item = &T>>, found Chain<…>`.
    /// - Tous les éléments doivent être des `&T` : `std::iter::once(self.premier)` donne des `T`, d'où
    ///   `E0271 type mismatch resolving … expected type parameter T, found &T`.
    pub fn iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        // But : le premier, puis le reste, en un seul itérateur.
        // `once(&self.premier)` est un itérateur d'un seul `&T` ; `chain` le colle devant `self.reste.iter()`.
        // Alternative : rendre `impl Iterator<Item = &T> + '_` éviterait l'allocation de la `Box`.
        Box::new(std::iter::once(&self.premier).chain(self.reste.iter()))
    }

    /// Applique `f` à chaque élément : une liste non vide reste non vide.
    ///
    /// **Aide Rust** (ch00, sections 3, 9)
    /// - `.map(f)` déplace `f` dans l'itérateur ; le réutiliser ensuite donne `E0382 borrow of moved value: f`.
    ///   Passez `&f` à `map` : une référence à un `Fn` est aussi un `Fn`.
    /// - `self` est possédé : prenez les éléments par valeur (`into_iter()`). Avec `iter()`, `f` recevrait des `&T` :
    ///   `E0277 expected a FnMut(&T) closure, found &impl Fn(T) -> U`.
    pub fn map<U>(self, f: impl Fn(T) -> U) -> NonVide<U> {
        // But : transformer le premier, puis le reste, avec la même closure.
        // `f` sert deux fois : appelée sur le premier, puis prêtée à `map`. Si `F: Fn`, alors `&F` est aussi un `Fn`.
        // Le résultat a encore un premier élément : non vide par construction, rien à revérifier.
        NonVide {
            premier: f(self.premier),
            reste: self.reste.into_iter().map(&f).collect(),
        }
    }

    /// Les éléments dans un `Vec` (jamais vide).
    ///
    /// **Aide Rust** (ch00, sections 3, 7)
    /// - `self.reste.insert(0, ...)` donne
    ///   `E0596 cannot borrow self.reste as mutable, as self is not declared as mutable`, et muter n'est pas le style
    ///   demandé. Fabriquez un nouvel itérateur, le premier puis le reste, et collectez-le.
    pub fn vers_vec(self) -> Vec<T> {
        // But : l'inverse de `new`.
        // `chain` accepte tout `IntoIterator` : on lui passe le `Vec` directement, par valeur.
        std::iter::once(self.premier).chain(self.reste).collect()
    }
}

impl<T: Ord> NonVide<T> {
    /// Le plus grand élément (le premier rencontré en cas d'égalité) : jamais d'échec.
    ///
    /// **Aide Rust** (ch00, sections 2, 11)
    /// - L'accumulateur du `fold` est un `&T` : partir de `self.premier` donne
    ///   `E0308 … expected &T, found type parameter T`.
    /// - Piège qui compile : `self.reste.iter().max().unwrap_or(&self.premier)` ignore le premier dès que le reste
    ///   n'est pas vide (`[9, 1]` rendrait `1`).
    pub fn maximum(&self) -> &T {
        // But : un `fold` qui part du premier élément.
        // Partir de `&self.premier` remplace l'`Option` qu'aurait rendue un `max()` sur le reste.
        // `x > max` strict : en cas d'égalité, on garde l'élément déjà retenu, donc le premier rencontré.
        self.reste
            .iter()
            .fold(&self.premier, |max, x| if x > max { x } else { max })
    }
}

impl<T> TryFrom<Vec<T>> for NonVide<T> {
    type Error = VecVide;

    /// **Aide Rust** (ch00, sections 3, 11)
    /// - `depuis_iter` attend un itérateur : `depuis_iter(valeurs)` donne `E0277 Vec<T> is not an iterator`.
    /// - `depuis_iter` rend une `Option`, on veut un `Result`
    ///   (`E0308 … expected Result<NonVide<T>, VecVide>, found Option<NonVide<T>>`) : `Option` a une méthode qui
    ///   transforme `None` en une erreur donnée.
    fn try_from(valeurs: Vec<T>) -> Result<Self, Self::Error> {
        // But : réutiliser `depuis_iter`, nommer l'absence.
        // La seule vérification « non vide » du type a lieu ici. `ok_or` donne un nom, `VecVide`, au `None`.
        NonVide::depuis_iter(valeurs.into_iter()).ok_or(VecVide)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    #[test]
    fn construction() {
        assert_eq!(NonVide::<i64>::try_from(vec![]), Err(VecVide));
        assert_eq!(NonVide::try_from(vec![4, 2]), Ok(NonVide::new(4, vec![2])));
        assert_eq!(
            NonVide::depuis_iter("abc".chars()),
            Some(NonVide::new('a', vec!['b', 'c']))
        );
        assert_eq!(NonVide::depuis_iter(std::iter::empty::<u8>()), None);
    }

    #[test]
    fn lecture_sans_option() {
        let liste = NonVide::new(3, vec![9, 1, 9]);
        assert_eq!(*liste.premier(), 3);
        assert_eq!(*liste.maximum(), 9);
        assert_eq!(liste.taille(), 4);
        assert_eq!(liste.iter().copied().collect::<Vec<_>>(), vec![3, 9, 1, 9]);
        assert_eq!(*NonVide::new(5, vec![]).maximum(), 5);
    }

    #[test]
    fn map_et_retour_en_vec() {
        let mots = NonVide::new("un", vec!["deux", "trois"]);
        let longueurs = mots.map(str::len);
        assert_eq!(longueurs.vers_vec(), vec![2, 4, 5]);
    }

    #[test]
    fn le_naif_panique_sur_une_liste_vide() {
        assert!(std::panic::catch_unwind(|| premier_naif(&[])).is_err());
        assert!(std::panic::catch_unwind(|| maximum_naif(&[])).is_err());
    }

    #[test]
    fn meme_resultat_que_le_naif_sur_les_listes_non_vides() {
        jeux::plusieurs(-100, 100)
            .iter()
            .for_each(|jeu| match NonVide::try_from(jeu.clone()) {
                Ok(liste) => {
                    assert_eq!(*liste.premier(), premier_naif(jeu), "jeu : {jeu:?}");
                    assert_eq!(*liste.maximum(), maximum_naif(jeu), "jeu : {jeu:?}");
                    assert_eq!(liste.taille(), jeu.len());
                    assert_eq!(liste.vers_vec(), *jeu);
                }
                Err(VecVide) => assert!(jeu.is_empty()),
            });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e05_non_vide.rs"),
            &[
                "depuis_iter",
                "premier",
                "taille",
                "iter",
                "map",
                "vers_vec",
                "maximum",
                "try_from",
            ],
        );
    }
}
