//! Exercice 1 : des températures qu'on ne mélange plus : `From` et `Into`.
//!
//! Un `f64` ne dit pas son unité : `plus_chaude_naif(30.0, 'C', 80.0, 'F')` repose sur deux `char` qu'on peut
//! inverser ou mal taper. Un newtype par unité, et `From` pour chaque conversion qui a un sens : les formules
//! vivent à un seul endroit. En implémentant `From<Fahrenheit> for Celsius`, on reçoit gratuitement
//! `Into<Celsius> for Fahrenheit` : une fonction qui prend `impl Into<Celsius>` accepte toutes les unités.

/// Version naïve : une conversion par fonction, des `char` pour dire l'unité. Ne pas modifier.
pub fn vers_celsius_naif(valeur: f64, unite: char) -> f64 {
    match unite {
        'C' => valeur,
        'F' => (valeur - 32.0) * 5.0 / 9.0,
        'K' => valeur - 273.15,
        autre => panic!("unité inconnue : {autre}"),
    }
}

/// Version naïve. Ne pas modifier.
pub fn celsius_vers_fahrenheit_naif(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

/// Version naïve : la plus chaude des deux, en degrés Celsius. Ne pas modifier.
pub fn plus_chaude_naif(a: f64, unite_a: char, b: f64, unite_b: char) -> f64 {
    let a = vers_celsius_naif(a, unite_a);
    let b = vers_celsius_naif(b, unite_b);
    if a >= b { a } else { b }
}

/// Degrés Celsius.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Celsius(pub f64);

/// Degrés Fahrenheit.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Fahrenheit(pub f64);

/// Kelvins.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Kelvin(pub f64);

impl From<Fahrenheit> for Celsius {
    /// **Aide Rust** (ch00, section 1)
    /// - `f` est un `Fahrenheit`, pas un nombre : `f - 32.0` donne `E0369 cannot subtract {float} from Fahrenheit`.
    ///   Le nombre est dans `f.0`.
    /// - On rend un `Celsius` : la formule seule donne `E0308 … expected Celsius, found f64`. Emballez-la dans
    ///   `Celsius(...)`. Même chose pour les deux autres `from`.
    fn from(f: Fahrenheit) -> Self {
        // But : la formule de la version naïve, à un seul endroit.
        todo!()
    }
}

impl From<Kelvin> for Celsius {
    fn from(k: Kelvin) -> Self {
        todo!()
    }
}

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        todo!()
    }
}

/// La plus chaude des deux températures, en Celsius, quelles que soient leurs unités. À égalité, `a`.
///
/// Ce qui compile :
///
/// ```no_run
/// use ch13_traits::e01_temperatures::{Fahrenheit, Kelvin, plus_chaude};
/// let c = plus_chaude(Fahrenheit(80.0), Kelvin(300.0));
/// ```
///
/// Un `f64` nu ne dit pas son unité : il n'existe pas de `From<f64> for Celsius`, donc pas de `Into<Celsius>`
/// pour un `f64`, et l'appel ne compile pas.
///
/// ```compile_fail,E0277
/// use ch13_traits::e01_temperatures::plus_chaude;
/// let c = plus_chaude(30.0, 80.0);
/// ```
///
/// **Aide Rust** (ch00, section 12)
/// - Comparer `a` et `b` tels quels donne `E0369 binary operation >= cannot be applied to type impl Into<Celsius>` :
///   convertissez d'abord avec `.into()`. Ici le type de retour suffit au compilateur pour savoir vers quoi
///   convertir ; ailleurs, annotez (`let a: Celsius = a.into();`).
/// - `max` n'existe pas :
///   `E0599 the method max exists for struct Celsius, but its trait bounds were not satisfied … Celsius: Ord`. Un
///   `f64` n'est que `PartialOrd` (ch. 10) : un `if` avec `>=` fait l'affaire.
pub fn plus_chaude(a: impl Into<Celsius>, b: impl Into<Celsius>) -> Celsius {
    // But : convertir les deux, comparer. Aucune formule ici.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    fn proches(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn conversions() {
        assert!(proches(Celsius::from(Fahrenheit(212.0)).0, 100.0));
        assert!(proches(Celsius::from(Kelvin(0.0)).0, -273.15));
        assert!(proches(Fahrenheit::from(Celsius(-40.0)).0, -40.0));
        let c: Celsius = Fahrenheit(32.0).into();
        assert!(proches(c.0, 0.0));
    }

    #[test]
    fn toutes_les_unites() {
        assert!(proches(
            plus_chaude(Celsius(30.0), Fahrenheit(80.0)).0,
            30.0
        ));
        assert!(proches(plus_chaude(Kelvin(300.0), Celsius(20.0)).0, 26.85));
        assert!(proches(
            plus_chaude(Fahrenheit(100.0), Kelvin(0.0)).0,
            37.77777777777778
        ));
    }

    #[test]
    fn meme_resultat_que_le_naif() {
        let valeurs = jeux::entiers(4, 30, -300, 400);
        valeurs.windows(2).for_each(|paire| {
            let (x, y) = (paire[0] as f64, paire[1] as f64);
            assert!(proches(
                plus_chaude(Fahrenheit(x), Kelvin(y + 300.0)).0,
                plus_chaude_naif(x, 'F', y + 300.0, 'K')
            ));
            assert!(proches(
                plus_chaude(Celsius(x), Fahrenheit(y)).0,
                plus_chaude_naif(x, 'C', y, 'F')
            ));
            assert!(proches(
                Fahrenheit::from(Celsius(x)).0,
                celsius_vers_fahrenheit_naif(x)
            ));
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(include_str!("e01_temperatures.rs"), &["plus_chaude"]);
    }
}
