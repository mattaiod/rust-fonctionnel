# Chapitre 18 : une API en couches avec Axum

> Cours : diapos « Une architecture en couches » à « Tester une route ». Tests : `cargo test -p ch18-couches`.

Une petite API de capteurs, une couche par fichier. Une requête traverse les couches de haut en bas, et chaque
couche ne connaît que celle du dessous :

```text
requête HTTP
  → e04_routes        quelle URL appelle quel handler
  → e03_controleur    HTTP : lire Path et Json, appeler le service, rendre un code et du JSON
  → e02_service       les règles : calculer les stats, ni HTTP ni stockage
  → e01_repository    le stockage, derrière le trait MesuresRepository
```

Faites-les **dans l'ordre, de bas en haut** : chaque couche utilise celle d'avant.

| # | Fichier | Ce que vous écrivez |
|---|---|---|
| 1 | `e01_repository.rs` | Le stockage en mémoire, qui implémente le trait |
| 2 | `e02_service.rs` | `stats_capteur`, générique sur `R: MesuresRepository` |
| 3 | `e03_controleur.rs` | Le handler `GET /capteurs/{id}/stats` (200 ou 404) |
| 4 | `e04_routes.rs` | Le `Router` qui branche le tout |

À la fin, lancez le vrai serveur et testez-le avec `curl` :

```bash
cargo run -p ch18-couches --example serveur
curl -s -X POST localhost:3000/capteurs/1/mesures -H 'Content-Type: application/json' -d '{"valeurs": [21.5, 22.0, 19.8]}'
curl -s localhost:3000/capteurs/1/stats
```

Loco range une application de la même façon, avec les mêmes couches (`controllers/`, `services/`,
`repositories/`), le stockage étant une base SQL.
