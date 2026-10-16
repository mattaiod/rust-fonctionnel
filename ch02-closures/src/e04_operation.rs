//! Exercice 4 : choisir une opération à l'exécution.
//!
//! Chaque closure a son propre type, même si deux closures se ressemblent : un `match` qui rend une closure
//! différente par branche ne peut pas avoir pour type `impl Fn` (erreur `E0308`, « no two closures, even if
//! identical, have the same type »). La solution : `Box<dyn Fn(i64) -> i64>`, un pointeur vers « n'importe
//! quelle closure de ce type ». Une allocation et un appel indirect, en échange de la souplesse.
//! `Ajouter(k)` capture `k` : `move` encore.

/// Les opérations disponibles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Doubler,
    Opposer,
    Carre,
    Ajouter(i64),
}

/// Version impérative : un `match` dans une boucle. Ne pas modifier.
pub fn appliquer_toutes_imperatif(operations: &[Operation], x: i64) -> i64 {
    let mut valeur = x;
    for op in operations {
        valeur = match op {
            Operation::Doubler => valeur * 2,
            Operation::Opposer => -valeur,
            Operation::Carre => valeur * valeur,
            Operation::Ajouter(k) => valeur + k,
        };
    }
    valeur
}

/// La closure qui réalise `op`.
///
/// **Aide Rust** (ch00, sections 8, 10)
/// - Si le compilateur dit ``E0308: expected `Box<dyn Fn(i64) -> i64>`, found closure`` : chaque branche doit rendre
///   `Box::new(|x| …)`, pas la closure nue.
/// - Si le compilateur dit ``E0373: closure may outlive the current function, but it borrows `k` `` : la branche
///   `Ajouter(k)` a besoin de `Box::new(move |x| …)`.
pub fn operation(op: Operation) -> Box<dyn Fn(i64) -> i64> {
    // Un `match` dont chaque branche rend `Box::new(|x| …)` (ou `Box::new(move |x| …)`).
    todo!()
}

/// Version fonctionnelle : les opérations appliquées dans l'ordre, en partant de `x`.
///
/// **Aide Rust** (ch00, sections 3, 7)
/// - Si le compilateur dit ``E0308: expected `Operation`, found `&Operation` `` : passez `*op` (permis : `Operation`
///   est `Copy`).
/// - `operation(*op)` rend une closure : on l'appelle aussitôt avec une deuxième paire de parenthèses,
///   `operation(*op)(valeur)`, et le `fold` RENVOIE ce résultat.
pub fn appliquer_toutes(operations: &[Operation], x: i64) -> i64 {
    // Outils : `iter()`, `fold(x, |acc, op| …)`, `operation(*op)(acc)`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    #[test]
    fn cas_connus() {
        assert_eq!(operation(Operation::Doubler)(21), 42);
        assert_eq!(operation(Operation::Opposer)(5), -5);
        assert_eq!(operation(Operation::Carre)(-3), 9);
        assert_eq!(operation(Operation::Ajouter(10))(1), 11);
        let chaine = [Operation::Ajouter(1), Operation::Carre, Operation::Opposer];
        assert_eq!(appliquer_toutes(&chaine, 2), -9);
        assert_eq!(appliquer_toutes(&[], 7), 7);
    }

    #[test]
    fn des_closures_differentes_dans_un_meme_vec() {
        let pipeline: Vec<Box<dyn Fn(i64) -> i64>> = vec![
            operation(Operation::Doubler),
            operation(Operation::Ajouter(-1)),
        ];
        assert_eq!(
            pipeline.iter().map(|f| f(10)).collect::<Vec<_>>(),
            vec![20, 9]
        );
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        (0..20).for_each(|graine| {
            let ops: Vec<Operation> = jeux::entiers(graine, 8, 0, 6)
                .iter()
                .map(|n| match n {
                    0 => Operation::Doubler,
                    1 => Operation::Opposer,
                    2 => Operation::Carre,
                    k => Operation::Ajouter(k - 4),
                })
                .collect();
            jeux::entiers(graine + 100, 5, -9, 9).iter().for_each(|x| {
                assert_eq!(
                    appliquer_toutes(&ops, *x),
                    appliquer_toutes_imperatif(&ops, *x),
                    "ops : {ops:?}, x = {x}"
                )
            });
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e04_operation.rs"),
            &["operation", "appliquer_toutes"],
        );
    }
}
