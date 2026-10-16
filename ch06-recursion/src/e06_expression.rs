//! Exercice 6 : évaluer une expression arithmétique, sans version impérative : à vous.
//!
//! `(2 + 3) × -4` se représente par un arbre : `Produit(Somme(Nombre(2), Nombre(3)), Oppose(Nombre(4)))`. C'est la
//! forme qu'un compilateur ou un tableur donne à une formule après l'avoir lue. Évaluer, c'est descendre dans
//! l'arbre : une branche de `match` par variante, un appel récursif par sous-expression.
//! `afficher` suit exactement le même plan, avec des chaînes au lieu de nombres : quand on tient la forme d'une
//! fonction récursive sur un type, les suivantes s'écrivent toutes seules.

/// Une expression arithmétique sur les entiers.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Nombre(i64),
    Somme(Box<Expr>, Box<Expr>),
    Produit(Box<Expr>, Box<Expr>),
    Oppose(Box<Expr>),
}

/// La valeur de l'expression.
///
/// **Aide Rust** (ch00, section 2)
/// - `match expr` sur un `&Expr` lie `n: &i64` : `` error[E0308]: mismatched types … expected `i64`, found `&i64` `` se
///   corrige en `*n`.
/// - Les sous-expressions sont des `&Box<Expr>` : `evaluer(a)` les accepte telles quelles, sans étoile.
pub fn evaluer(expr: &Expr) -> i64 {
    todo!()
}

/// L'expression entièrement parenthésée : `((2 + 3) * -4)`. Un nombre s'écrit seul, l'opposé avec un `-` collé.
///
/// **Aide Rust** (ch00, section 5)
/// - `n.to_string()` pour un nombre, `format!("({} + {})", …)` pour assembler. `"(" + …` ne compile pas :
///   `` error[E0369]: cannot add `String` to `&str` ``.
pub fn afficher(expr: &Expr) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::style;

    fn n(valeur: i64) -> Expr {
        Expr::Nombre(valeur)
    }
    fn somme(a: Expr, b: Expr) -> Expr {
        Expr::Somme(Box::new(a), Box::new(b))
    }
    fn produit(a: Expr, b: Expr) -> Expr {
        Expr::Produit(Box::new(a), Box::new(b))
    }
    fn oppose(a: Expr) -> Expr {
        Expr::Oppose(Box::new(a))
    }

    #[test]
    fn un_nombre_seul() {
        assert_eq!(evaluer(&n(42)), 42);
        assert_eq!(afficher(&n(42)), "42");
    }

    #[test]
    fn cas_connus() {
        let expr = produit(somme(n(2), n(3)), oppose(n(4)));
        assert_eq!(evaluer(&expr), -20);
        assert_eq!(afficher(&expr), "((2 + 3) * -4)");

        let expr = somme(n(1), produit(n(2), somme(n(3), oppose(oppose(n(4))))));
        assert_eq!(evaluer(&expr), 15);
        assert_eq!(afficher(&expr), "(1 + (2 * (3 + --4)))");
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(include_str!("e06_expression.rs"), &["evaluer", "afficher"]);
    }
}
