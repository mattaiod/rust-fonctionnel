# Chapitre 19 : FFI, Rust depuis Python

> Cours : diapos « FFI : Rust depuis Python ». Prérequis : Python 3.9 ou plus récent.

Un seul exercice, dans `src/lib.rs` : les fonctions du module Python `capteurs`. Ce chapitre ne se teste pas avec
`cargo test` : c'est Python qui appelle votre code.

```bash
cd ch19-ffi
./lancer.sh
```

`lancer.sh` crée un environnement Python (`.venv`), installe maturin, compile le module, lance les tests Python
(`tests/test_capteurs.py`) puis `bench.py` : Python pur, votre Rust et NumPy sur 1 million de valeurs. Qu'en
concluez-vous ?

Sous Windows sans bash, faites les étapes du script à la main : `python -m venv .venv`, activer l'environnement,
`pip install maturin numpy`, `maturin develop --release`, `python -m unittest discover -s tests -v`, `python bench.py`.
