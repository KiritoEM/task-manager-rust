# Simple Task-Manager

Un petit outil CLI permettant de gérer des tâches via des fichiers JSON développé avec Rust. Ce gestionnaire permet d'ajouter, supprimer, mettre à jour et afficher des tâches tout en sauvegardant les opérations dans un fichier JSON.

## Fonctionnalités

- [x] Ajout de tâche
- [x] Affichage des tâches en board
- [x] Suppression de tâche 
- [ ] Modification du status 
- [x] Sauvegarde de toutes opérations dans un fichier JSON



## Installation

Pour installer l'outil, installer d'abord [Rust](https://www.rust-lang.org/learn/get-started) et utiliser les commandes:

```bash
git clone https://github.com/KiritoEM/task-manager-rust
cd task-manager-rust
cargo build --release
```

## Usage

```bash
task-manager
```

### Format du fichier JSON :
```bash
[
  {
    "name": "Task 1",
    "status": "INPROGRESS",
    "description": "Description 1"
  },
  {
    "name": "Task 2",
    "status": "TODO",
    "description": "Description 2"
  }
]

```

### Commandes disponibles :
- **add** : Ajoute une nouvelle tâche.
- **board** : Affiche la liste des tâches.
- **delete** : Supprimer une tâche
- **help** : Affiche ce message d'aide ou l'aide pour une sous-commande donnée

### Options :
- `-h`, `--help` : Affiche l'aide.
- `-V`, `--version` : Affiche la version de l'outil.
- 
## Exemple d'utilisation

- ### Ajouter une tâche 
 Le status est par défaut TODO (il n'existe que TODO, INPROGRESS, DONE).

```bash
task-manager add --name "Développer une authentification" --status "INPROGRESS" --description "Mettre en place le système d'authentification avec JWT et 0Auth" --file "example/tasks.json"
```

- ### Afficher le board de tâches

```bash
task-manager board --file "example/tasks.json"
```

- ### Supprimer une tâche

```bash
task-manager delete --name "Développer une authentification" --file "example/tasks.json"
```



