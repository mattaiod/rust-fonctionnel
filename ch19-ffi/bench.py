"""Benchmark : Python pur contre Rust (capteurs) contre NumPy, sur 1 million de valeurs. Lancé par ./lancer.sh.

La conversion list -> Vec<f64> coûte à chaque appel, et NumPy travaille sur un tableau déjà converti. Les fonctions
natives de Python (sum, min, max) sont écrites en C.
"""

import random
import timeit

import capteurs

try:
    import numpy as np
except ImportError:  # NumPy est facultatif
    np = None

N = 1_000_000
REPETITIONS = 5


def stats_python(valeurs: list[float]) -> dict[str, float]:
    """Une boucle Python écrite à la main, un seul passage."""
    total, plus_petit, plus_grand = 0.0, valeurs[0], valeurs[0]
    for valeur in valeurs:
        total += valeur
        if valeur < plus_petit:
            plus_petit = valeur
        if valeur > plus_grand:
            plus_grand = valeur
    return {"moyenne": total / len(valeurs), "min": plus_petit, "max": plus_grand}


def stats_natives(valeurs: list[float]) -> dict[str, float]:
    """Les fonctions natives de Python : trois passages, mais en C."""
    return {"moyenne": sum(valeurs) / len(valeurs), "min": min(valeurs), "max": max(valeurs)}


def mesurer(nom: str, fonction) -> None:
    meilleur = min(timeit.repeat(fonction, number=1, repeat=REPETITIONS))
    print(f"  {nom:<34} {meilleur * 1000:8.1f} ms")


def main() -> None:
    rng = random.Random(2026)
    valeurs = [rng.uniform(-20.0, 40.0) for _ in range(N)]

    attendu = stats_python(valeurs)
    obtenu = capteurs.stats(valeurs)
    assert all(abs(attendu[nom] - obtenu[nom]) < 1e-6 for nom in attendu), (attendu, obtenu)

    print(f"stats (moyenne, min, max), {N:_} valeurs, meilleur de {REPETITIONS}")
    mesurer("Python pur (boucle)", lambda: stats_python(valeurs))
    mesurer("Python natif (sum, min, max)", lambda: stats_natives(valeurs))
    mesurer("Rust (capteurs.stats)", lambda: capteurs.stats(valeurs))
    if np is not None:
        tableau = np.array(valeurs)
        mesurer("NumPy (tableau prêt)", lambda: (tableau.mean(), tableau.min(), tableau.max()))
        mesurer("NumPy (avec conversion)", lambda: (lambda t: (t.mean(), t.min(), t.max()))(np.array(valeurs)))
    else:
        print("  NumPy absent : pip install numpy pour la comparaison")


if __name__ == "__main__":
    main()
