// use esercizi::{print_arr, transpose};

/*
fn main() {
    let fruits = vec!["apples", "oranges", "pears", "apricots", "kiwi", "lemons"];
    let mut str_fuits= vec![];

    for fruit in fruits {
        str_fuits.push(fruit.to_string());
    }

    // print_arr(&str_fuits);

    // print!("Shortest string length: {}\nNumber of fruits: {}\n", min(&str_fuits), &str_fuits.len());

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

use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

use esercizi::auction::{AuctionOutcome, AuctioneerMessage, PartecipantMessage, Product, auctioneer_routine, partecipant_routine};

// fn main() {
//     // 1. Parameter setup (you could ask the user for these here in the future)
//     let n_cells = 10;
//     let n_objects = 5;
//     let food_value = 5;
//     let poison_value = -10;
//     let initial_strength = 15;
//     let max_moves = 20;

//     // 2. Game initialization
//     let mut game = Configuration::new(n_cells, n_objects, food_value, poison_value, initial_strength);
//     let mut moves_made = 0;

//     // Start screen
//     print!("{}[2J{}[1;1H", 27 as char, 27 as char); // Clears the terminal
//     println!("=== GAME START ===");
//     println!("Press ENTER to start...");
//     let mut start = String::new();
//     io::stdin().read_line(&mut start).unwrap();

//     // 3. THE GAME LOOP
//     // The game continues while the player is alive (strength > 0) AND has moves left
//     while game.player.strength > 0 && moves_made < max_moves {
//         // Clears the screen every turn for an "animation" effect
//         print!("{}[2J{}[1;1H", 27 as char, 27 as char);

//         // Prints the user interface (UI)
//         println!("ROUND: {}/{}", moves_made + 1, max_moves);
//         println!("PLAYER STRENGTH: {}", game.player.strength);
//         println!("----------------------");

//         // Prints the map (requires you to implement Display for Configuration)
//         println!("{}", game);
//         println!("----------------------");

//         // Waits for user input
//         println!("Press ENTER for next round...");
//         let mut wait_enter = String::new();
//         io::stdin().read_line(&mut wait_enter).expect("Error while reading");

//         // Lets Configuration compute the turn logic
//         game.play_turn();

//         moves_made += 1;
//     }

//     // 4. FINAL SCREEN
//     print!("{}[2J{}[1;1H", 27 as char, 27 as char);
//     println!("{}", game);
//     println!("=== GAME ENDED ===");

//     if game.player.strength <= 0 {
//         println!("💀 YOU LOST! Poison killed you.");
//     } else {
//         println!("🏆 YOU WON! You survived for all {} rounds!", max_moves);
//         println!("Final strength: {}", game.player.strength);
//     }
// }

fn main() {
    let product = Product {
        start_price: 10,
        reserve_price: 15,
        description: "Bicchiere".to_string(),
    };

    let outcome = Arc::new(Mutex::new(AuctionOutcome {
        sold_product: product.description.clone(),
        sold_price_product: None,
        player_id: None,
    }));

    // Listen to all participants
    let (to_auctioneer, from_partecipant) = mpsc::channel::<PartecipantMessage>();

    let n = 5;
    let mut senders_to_partecipants: Vec<mpsc::Sender<AuctioneerMessage>> = Vec::new();

    for id in 0..n {
        // Auctioneer sends a message to every participant
        let (to_partecipant, from_auctioneer) = mpsc::channel();
        senders_to_partecipants.push(to_partecipant);

        let to_auctioneer_clone = to_auctioneer.clone();

        thread::spawn(move || {
            partecipant_routine(id, to_auctioneer_clone, from_auctioneer);
        });
    }

    drop(to_auctioneer);
    
    auctioneer_routine(n, product, from_partecipant, senders_to_partecipants, outcome);

    println!("Main: Program finished.");
}
