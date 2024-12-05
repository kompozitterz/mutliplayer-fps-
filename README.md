***Start server***

```sh
/client/ cargo run --release --bin server 
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