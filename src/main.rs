// use esercizi::{print_arr, transpose};

/*
fn main() {
    let fruits = vec!["mele", "arance", "pere", "albicocche", "kiwi", "limoni"];
    let mut str_fuits= vec![];

    for fruit in fruits {
        str_fuits.push(fruit.to_string());
    }

    // print_arr(&str_fuits);

    // print!("Lunghezza stringa più corta: {}\nNumero di frutti: {}\n", min(&str_fuits), &str_fuits.len());

    print_arr(&transpose(&str_fuits));
}
*/

// use esercizi::Rational;

// fn main() {
//     let rat = Rational::new(-7, 6);
//     let rat1 = Rational::new(-3, 4);
//     let mut rat2 = rat.product(&rat1);
//     println!("{}", rat.to_string());
//     println!("{}", rat1.to_string());
//     println!("Prodotto: {}", rat2.to_string());

//     rat2 = rat.sum(&rat1);
//     println!("Somma: {}", rat2.to_string());
//     println!("Inverso: {}", rat2.inverse().to_string());
//     // print!("{:?}{}/{}", rat.sign, rat.num, rat.den);
// }

use esercizi::game::Configuration;
use std::io;

fn main() {
    // 1. Setup dei parametri (in futuro potresti chiederli all'utente qui)
    let n_celle = 10;
    let n_oggetti = 5;
    let valore_cibo = 5;
    let valore_veleno = -10;
    let forza_iniziale = 15;
    let mosse_massime = 20;

    // 2. Inizializzazione del gioco
    let mut game = Configuration::new(n_celle, n_oggetti, valore_cibo, valore_veleno, forza_iniziale);
    let mut mosse_fatte = 0;

    // Schermata di avvio
    print!("{}[2J{}[1;1H", 27 as char, 27 as char); // Pulisce il terminale
    println!("=== GAME START ===");
    println!("Press ENETER to start...");
    let mut start = String::new();
    io::stdin().read_line(&mut start).unwrap();

    // 3. IL GAME LOOP
    // Il gioco continua finché il giocatore è vivo (forza > 0) E non ha finito le mosse
    while game.player.strength > 0 && mosse_fatte < mosse_massime {
        // Pulisce lo schermo a ogni turno per un effetto "animazione"
        print!("{}[2J{}[1;1H", 27 as char, 27 as char);

        // Stampa l'interfaccia utente (UI)
        println!("ROUND: {}/{}", mosse_fatte + 1, mosse_massime);
        println!("PLAYER STRENGTH: {}", game.player.strength);
        println!("----------------------");
        
        // Stampa la mappa (richiede che tu implementi Display per Configuration)
        println!("{}", game); 
        println!("----------------------");

        // Aspetta l'input dell'utente
        println!("Press ENETER for next round...");
        let mut aspetta_invio = String::new();
        io::stdin().read_line(&mut aspetta_invio).expect("Error while reading");

        // Fai calcolare la logica del turno a Configuration
        game.play_turn();
        
        mosse_fatte += 1;
    }

    // 4. SCHERMATA FINALE
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);
    println!("{}", game);
    println!("=== GAME ENDED ===");

    if game.player.strength <= 0 {
        println!("💀 YOU LOST! Poison killed you.");
    } else {
        println!("🏆 YOU WON! You survived for all {} rounds!", mosse_massime);
        println!("Final strength: {}", game.player.strength);
    }
}
