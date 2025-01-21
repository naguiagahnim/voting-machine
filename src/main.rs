use std::{io, vec};

struct score {
    nom: String,
    score: i32
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Bienvenue sur le serveur de vote !");
    println!("Les commandes valides sont : voter, votants ou score");
    let mut votants = vec!["Jean Terouak", "Franky Vincent", "Jean-Pierre Pernaut", "Brice Binouze", "Brigitte Bibine"];
    let mut scores = vec![
        score { nom: "Grahargul le destructeur de mondes".to_string(), score: 4 },
        score { nom: "Titi le gentil".to_string(), score: 0 },
        score { nom: "Jacky Mono".to_string(), score: 1 }
    ];
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "voter" => {
                println!("Pour qui voulez-vous voter ?");
                let mut nom = String::new();
                io::stdin().read_line(&mut nom)?;
                let nom = nom.trim();

                if let Some(score) = scores.iter_mut().find(|s| s.nom == nom) {
                    score.score += 1;
                    println!("Vote enregistré pour {}", nom);
                } else {
                    println!("Candidat non trouvé");
                }
            },
            "votants" => {
                println!("Voici la liste des votants :");
                for votant in &votants {
                    println!("• {}", votant);
                }
            },
            "score" => {
                println!("Scores actuels :");
                for score in &scores {
                    println!("• {} : {}", score.nom, score.score);
                }
            },
            "" => println!("Erreur : Saisissez une commande parmi voter, votants ou score"),
            _ => println!("Erreur : Commande invalide ! Les commandes valides sont : voter, votants ou score"),
        }
    }
    Ok(())
}