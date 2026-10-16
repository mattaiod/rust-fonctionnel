//! Exercice 4 : des opérations décrites par des données, transformées en une seule fonction.
//!
//! Un `enum Operation` décrit un calcul sans l'exécuter : c'est une donnée, qu'on peut lire depuis un fichier,
//! afficher, tester. La version impérative l'**interprète** à chaque appel : une boucle, un `match` par opération.
//! La version fonctionnelle la **traduit** une fois pour toutes : chaque opération devient une fonction, puis la
//! liste devient une seule fonction composée, qu'on appelle autant de fois qu'on veut.
//! Pourquoi `Box<dyn Fn>` en retour de `vers_fonction` ? Les trois branches du `match` rendent trois closures de
//! types différents ; `impl Fn` exige un seul type concret, `Box<dyn Fn>` accepte les trois.

/// Une opération sur un entier.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Ajouter(i32),
    Multiplier(i32),
    Opposer,
}

/// Version impérative : un interprète qui relit la liste à chaque appel. Ne pas modifier.
pub fn appliquer_imperatif(operations: &[Operation], depart: i32) -> i32 {
    let mut valeur = depart;
    for operation in operations {
        match operation {
            Operation::Ajouter(n) => valeur += n,
            Operation::Multiplier(n) => valeur *= n,
            Operation::Opposer => valeur = -valeur,
        }
    }
    valeur
}

/// La fonction qui réalise une opération.
///
/// **Aide Rust** (ch00, sections 8, 10, 13)
/// - Trois bras, trois closures de types différents : un `Box::new(…)` par bras. Une closure rendue sans `Box` donne
///   `` error[E0308]: mismatched types … expected `Box<dyn Fn(i32) -> i32>`, found closure ``. (Avec `-> impl Fn` en
///   retour, ce serait `` `match` arms have incompatible types … no two closures, even if identical, have the same
///   type ``.)
/// - `match operation` sur un `&Operation` lie `n: &i32` : la closure garderait une référence vers `operation`
///   (`` error: lifetime may not live long enough … returning this value requires that `'1` must outlive `'static` ``).
///   `match *operation` donne un `i32` (`Operation` est `Copy`).
/// - Si le compilateur dit `` error[E0373]: closure may outlive the current function, but it borrows `n` `` : il manque
///   `move` devant la closure.
pub fn vers_fonction(operation: &Operation) -> Box<dyn Fn(i32) -> i32> {
    // Une branche par variante, chacune rend une closure en boîte ; la closure doit posséder le nombre qu'elle utilise.
    todo!()
}

/// Une seule fonction qui applique toutes les opérations dans l'ordre.
///
/// **Aide Rust** (ch00, sections 9, 10)
/// - Premier trou : `vers_fonction` se passe par son nom à `map`, et `collect` remplit le `Vec<Box<dyn Fn…>>` annoté.
/// - La closure rendue est une `Fn` : elle sera appelée plusieurs fois, elle ne peut pas consommer `etapes`. Si le
///   compilateur dit `` error[E0507]: cannot move out of `etapes`, a captured variable in an `Fn` closure `` :
///   `etapes.iter()`, pas `into_iter()`.
pub fn composer_operations(operations: &[Operation]) -> impl Fn(i32) -> i32 {
    // Traduire une seule fois, au moment de composer, et pas à chaque appel.
    let etapes: Vec<Box<dyn Fn(i32) -> i32>> = {
        // Chaque opération devient sa fonction.
        todo!()
    };
    move |depart| {
        // Faire passer la valeur dans toutes les étapes, dans l'ordre, en la transportant d'une étape à la suivante.
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Operation::{Ajouter, Multiplier, Opposer};
    use pf_outils::{jeux, style};

    #[test]
    fn vers_fonction_cas_connus() {
        assert_eq!(vers_fonction(&Ajouter(3))(4), 7);
        assert_eq!(vers_fonction(&Multiplier(-2))(4), -8);
        assert_eq!(vers_fonction(&Opposer)(4), -4);
    }

    #[test]
    fn cas_connus() {
        let calcul = composer_operations(&[Ajouter(1), Multiplier(10), Opposer]);
        assert_eq!(calcul(2), -30);
        assert_eq!(calcul(0), -10);
        assert_eq!(composer_operations(&[])(42), 42);
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        jeux::plusieurs(-10, 10).iter().for_each(|jeu| {
            // Multiplier seulement par -1, 0 ou 1 : pas de dépassement de capacité.
            let operations: Vec<Operation> = jeu
                .iter()
                .map(|&n| match n.rem_euclid(3) {
                    0 => Ajouter(n as i32),
                    1 => Multiplier(n.signum() as i32),
                    _ => Opposer,
                })
                .collect();
            let calcul = composer_operations(&operations);
            (-5..=5).for_each(|x| {
                assert_eq!(
                    calcul(x),
                    appliquer_imperatif(&operations, x),
                    "opérations : {operations:?}, x = {x}"
                )
            });
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e04_operations.rs"),
            &["vers_fonction", "composer_operations"],
        );
    }
}
