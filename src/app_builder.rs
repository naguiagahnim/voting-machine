use std::{io, vec};

use crate::configuration::Configuration;

struct Score {
    nom: String,
    score: i32
}


pub async fn run_app(configuration : Configuration) -> anyhow::Result<()> {
    println!("Bienvenue sur le serveur de vote !");
    println!("Les commandes valides sont : voter, votants ou score");
    let mut votants = vec![String::from("Brice Binouze")];
    let mut scores: Vec<Score> = configuration.candidates
        .iter()
        .map(|name| Score {
            nom: name.to_string(),
            score: 0,
        })
        .collect();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "voter" => {
                println!("Quel votant êtes-vous ?");
                let mut votant = String::new();
                io::stdin().read_line(&mut votant)?;
                votant = votant.trim().to_string();
                if votants.contains(&votant) {
                    println!("Vous avez déjà voté !");
                } else {
                    println!("Pour qui voulez-vous voter ?");
                    let mut nom = String::new();
                    io::stdin().read_line(&mut nom)?;
                    let nom = nom.trim();
                    
                    if nom.is_empty() {
                        // Vote blanc
                        if let Some(score) = scores.iter_mut().find(|s| s.nom == "Blanc") {
                            score.score += 1;
                            println!("Vote blanc enregistré");
                        }
                    } else if let Some(score) = scores.iter_mut().find(|s| s.nom == nom) {
                        // Vote pour un candidat existant
                        score.score += 1;
                        println!("Vote enregistré pour {}", nom);
                    } else {
                        // Vote nul (candidat non trouvé)
                        if let Some(score) = scores.iter_mut().find(|s| s.nom == "Nul") {
                            score.score += 1;
                            println!("Vote nul enregistré (candidat non trouvé)");
                        }
                    }
                    
                    votants.push(votant.trim().to_string());
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