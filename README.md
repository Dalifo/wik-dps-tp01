# wik-dps-tp01
# Serveur HTTP Rust

Ce projet contient un serveur HTTP simple en Rust qui répond aux requêtes GET sur le chemin "/ping". Le serveur peut être exécuté avec ou sans l'utilisation de la librairie Actix.


## Branches

- La branche `main` est une branche sans dépendances supplémentaires. Elle utilise les fonctionnalités standard de Rust pour créer un serveur HTTP minimaliste.

- La branche `withlibrary` utilise la librairie Actix Web pour gérer le serveur HTTP.


## Prérequis

Avant de pouvoir exécuter ce projet, assurez-vous d'avoir installé Cargo et Rust sur votre système. Voici comment procéder :

### Installation de Cargo et Rust

1. Assurez-vous que vous avez un compilateur C installé sur votre système. Vous pouvez installer GCC sur Linux en utilisant la commande suivante :

`udo apt-get install build-essential`

Sur Windows, vous pouvez utiliser le compilateur inclus avec Visual Studio ou installer MinGW.

2. Rendez-vous sur le site officiel de Rust à l'adresse https://www.rust-lang.org/tools/install.

3. Suivez les instructions pour télécharger et installer Rust et Cargo sur votre système.

4. Vérifiez que l'installation a réussi en exécutant les commandes suivantes dans votre terminal :

`rustc --version`

`cargo --version`


## Exécution du Code

Pour exécuter le code, suivez ces étapes :

1. Clonez ce dépôt GitHub sur votre système en utilisant la commande suivante :

`git clone https://github.com/Dalifo/wik-dps-tp01.git`

2. Naviguez dans le répertoire du projet en utilisant la commande `cd` :

`cd wik-dps-tp01`

3. Sélectionnez la branche que vous souhaitez exécuter en utilisant la commande `git checkout` :

`git checkout nom-de-la-branche`

Par exemple, pour la branche `main`, utilisez :

`git checkout main`

Pour la branche `withlibrary`, utilisez :

`git checkout withlibrary`

4. Enfin, exécutez le code en utilisant Cargo avec la commande suivante :

`cargo run`

Le serveur sera alors lancé et écoutera sur le port par défaut (8080) ou sur le port spécifié dans la variable d'environnement `PING_LISTEN_PORT`.

Assurez-vous de tester le serveur en accédant à l'URL `http://localhost:8080/ping` (ou remplacez le port en conséquence) dans votre navigateur ou en utilisant un outil de requête HTTP comme cURL ou Postman.
