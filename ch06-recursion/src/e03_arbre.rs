//! Exercice 3 : somme et profondeur d'un arbre binaire.
//!
//! Un type qui se contient lui-même n'a pas de taille connue : `Noeud(Arbre, i64, Arbre)` ne compile pas. `Box`
//! met le sous-arbre sur le tas et ne garde qu'un pointeur de taille fixe : `Noeud(Box<Arbre>, i64, Box<Arbre>)`.
//! Sur un type récursif, la fonction récursive suit la forme du type : une branche par variante, un appel par
//! sous-arbre. Quand on filtre une référence (`match arbre` avec `arbre: &Arbre`), les champs sont liés par
//! référence : `Arbre::Noeud(gauche, valeur, droite)` lie `gauche: &Box<Arbre>` et `valeur: &i64`. On passe `gauche`
//! tel quel à une fonction qui attend un `&Arbre` : le compilateur convertit seul un `&Box<Arbre>` en `&Arbre`.
//! La version impérative doit gérer elle-même une pile de nœuds à visiter : la récursion utilise la pile d'appels.

/// Un arbre binaire d'entiers.
#[derive(Debug, Clone, PartialEq)]
pub enum Arbre {
    Feuille,
    Noeud(Box<Arbre>, i64, Box<Arbre>),
}

/// Version impérative : une pile explicite de nœuds à visiter. Ne pas modifier.
pub fn somme_arbre_imperatif(arbre: &Arbre) -> i64 {
    let mut total = 0;
    let mut pile = vec![arbre];
    while let Some(courant) = pile.pop() {
        if let Arbre::Noeud(gauche, valeur, droite) = courant {
            total += valeur;
            pile.push(gauche);
            pile.push(droite);
        }
    }
    total
}

/// Version impérative : la pile garde aussi la profondeur de chaque nœud. Ne pas modifier.
pub fn profondeur_imperatif(arbre: &Arbre) -> usize {
    let mut maximum = 0;
    let mut pile = vec![(arbre, 0)];
    while let Some((courant, niveau)) = pile.pop() {
        if let Arbre::Noeud(gauche, _, droite) = courant {
            maximum = maximum.max(niveau + 1);
            pile.push((gauche, niveau + 1));
            pile.push((droite, niveau + 1));
        }
    }
    maximum
}

/// Somme de toutes les valeurs de l'arbre (0 pour une feuille).
///
/// **Aide Rust** (ch00, section 2)
/// - `match arbre` sur un `&Arbre` lie `gauche: &Box<Arbre>` : passez-le tel quel, `somme_arbre(gauche)` (la `Box` se
///   déréférence toute seule).
/// - Si le compilateur dit `` error[E0308]: mismatched types … expected `&Arbre`, found `Box<Arbre>` `` : vous avez
///   écrit `somme_arbre(*gauche)`, retirez l'étoile.
/// - Si le compilateur dit `` error[E0507]: cannot move out of `arbre` as enum variant `Noeud` which is behind a shared
///   reference `` : vous avez écrit `match *arbre`. On ne sort pas une `Box` d'un arbre emprunté : `match arbre`.
pub fn somme_arbre(arbre: &Arbre) -> i64 {
    // Une feuille vaut 0 ; un nœud vaut sa valeur plus la somme de ses deux sous-arbres.
    todo!()
}

/// Nombre de nœuds sur le plus long chemin de la racine à une feuille (0 pour une feuille seule).
pub fn profondeur(arbre: &Arbre) -> usize {
    // Une feuille a une profondeur nulle ; un nœud, un de plus que le plus profond de ses sous-arbres.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    fn noeud(gauche: Arbre, valeur: i64, droite: Arbre) -> Arbre {
        Arbre::Noeud(Box::new(gauche), valeur, Box::new(droite))
    }

    fn seul(valeur: i64) -> Arbre {
        noeud(Arbre::Feuille, valeur, Arbre::Feuille)
    }

    /// Arbre binaire de recherche : on insère les valeurs une à une.
    fn inserer(arbre: Arbre, x: i64) -> Arbre {
        match arbre {
            Arbre::Feuille => seul(x),
            Arbre::Noeud(g, v, d) if x < v => noeud(inserer(*g, x), v, *d),
            Arbre::Noeud(g, v, d) => noeud(*g, v, inserer(*d, x)),
        }
    }

    #[test]
    fn cas_connus() {
        //       5
        //      / \
        //     3   8
        //        /
        //       7
        let arbre = noeud(seul(3), 5, noeud(seul(7), 8, Arbre::Feuille));
        assert_eq!(somme_arbre(&arbre), 23);
        assert_eq!(profondeur(&arbre), 3);
    }

    #[test]
    fn feuille_seule() {
        assert_eq!(somme_arbre(&Arbre::Feuille), 0);
        assert_eq!(profondeur(&Arbre::Feuille), 0);
        assert_eq!(profondeur(&seul(-4)), 1);
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        jeux::plusieurs(-100, 100).iter().for_each(|jeu| {
            let arbre = jeu.iter().fold(Arbre::Feuille, |a, &x| inserer(a, x));
            assert_eq!(
                somme_arbre(&arbre),
                somme_arbre_imperatif(&arbre),
                "jeu : {jeu:?}"
            );
            assert_eq!(
                profondeur(&arbre),
                profondeur_imperatif(&arbre),
                "jeu : {jeu:?}"
            );
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(include_str!("e03_arbre.rs"), &["somme_arbre", "profondeur"]);
    }
}
