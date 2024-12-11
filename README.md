***Start server***

<video width="640" height="360" controls>
  <source src="assets/capture_1.mov" type="video/mp4">
  Votre navigateur ne supporte pas la balise vidéo.
</video>

```sh
/server/ cargo run --release --bin server 
```

***Recupération de l adresse ip***

```sh
ipconfig
```
Une fois l'adresse ip récupéré, la rentrée comme adresse référente lors de la connexion au server.

***Rejoindre le server***

```sh
/client/ cargo run --release --bin client 
```