"""Tests du module Python `capteurs`. Lancés par ./lancer.sh."""

import unittest

import capteurs

SERIE = [12.0, 15.5, 9.5, 20.0, 13.0]


class Statistiques(unittest.TestCase):
    def test_moyenne_min_max(self) -> None:
        self.assertAlmostEqual(capteurs.moyenne(SERIE), 14.0)
        self.assertEqual(capteurs.min(SERIE), 9.5)
        self.assertEqual(capteurs.max(SERIE), 20.0)

    def test_une_liste_vide_donne_none(self) -> None:
        self.assertIsNone(capteurs.moyenne([]))
        self.assertIsNone(capteurs.min([]))
        self.assertIsNone(capteurs.max([]))

    def test_les_entiers_python_sont_acceptes(self) -> None:
        self.assertAlmostEqual(capteurs.moyenne([1, 2, 3, 4]), 2.5)

    def test_stats_rend_un_dictionnaire(self) -> None:
        self.assertEqual(capteurs.stats([1.0, 3.0]), {"moyenne": 2.0, "min": 1.0, "max": 3.0})
        self.assertEqual(capteurs.stats([]), {"moyenne": None, "min": None, "max": None})


if __name__ == "__main__":
    unittest.main()
