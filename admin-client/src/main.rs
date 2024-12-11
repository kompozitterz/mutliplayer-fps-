use std::net::IpAddr; // Permet de travailler avec des adresses IP.

use admin_client::run_admin_client; // Import de la fonction pour exécuter le client administrateur.
use clap::Parser; // Bibliothèque pour parser les arguments de ligne de commande.
use common::defaults::{IP, PORT}; // Import des valeurs par défaut pour l'IP et le port.

/// Définition de la structure des arguments de ligne de commande.
/// Utilisation de `clap` pour gérer les options fournies par l'utilisateur.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // Métadonnées sur le programme.
struct Args {
    /// Port sur lequel le serveur sera hébergé.
    /// Utilisation d'un argument de ligne de commande court (-p) ou long (--port).
    #[arg(short, long, default_value_t = PORT)]
    port: u16,

    /// Adresse IP sur laquelle le serveur sera hébergé.
    /// Utilisation d'un argument de ligne de commande court (-i) ou long (--ip).
    #[arg(short, long, default_value_t = IP)]
    ip: IpAddr,
}

/// Point d'entrée principal du programme.
/// Utilisation de l'exécuteur async fourni par `tokio`.
#[tokio::main]
async fn main() -> Result<(), String> {
    // Parse les arguments de la ligne de commande pour obtenir les paramètres.
    let args = Args::parse();

    // Démarre le client administrateur avec l'adresse IP et le port spécifiés.
    // La fonction peut retourner une erreur, donc l'opérateur `?` est utilisé.
    run_admin_client(args.ip, args.port)?;

    // Retourne un résultat vide si tout s'est bien passé.
    Ok(())
}
