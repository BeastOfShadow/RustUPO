use std::sync::{Arc, Mutex, mpsc};

use rand::Rng;

pub struct Product {
    pub start_price: u32,
    pub reserve_price: u32,
    pub description: String,
}

pub struct AuctionOutcome {
    pub sold_product: String,
    pub sold_price_product: Option<u32>,
    pub player_id: Option<u32>,
}

pub struct Partecipant {
    pub id: u32,
}

pub enum PartecipantMessage {
    Ready(u32),
    Bid(u32, u32),
    Exit(u32),
}

pub enum AuctioneerMessage {
    Start {
        product_desc: String,
        min_price: u32,
    },
    NewPrice(u32),
    Winner(Option<u32>),
}

pub fn partecipant_routine(
    id: u32,
    to_auctioneer: mpsc::Sender<PartecipantMessage>,
    from_auctioneer: mpsc::Receiver<AuctioneerMessage>,
) {
    to_auctioneer.send(PartecipantMessage::Ready(id)).unwrap();

    while let Ok(message) = from_auctioneer.recv() {
        match message {
            AuctioneerMessage::Start {
                product_desc,
                min_price,
            } => {
                println!(
                    "Player {} see: {}, starting price {}",
                    id, product_desc, min_price
                );

                let mut rnd = rand::thread_rng();
                let action = rnd.gen_range(0..100);
                let min_bet = min_price + 1;

                if action >= 10 {
                    let bid = PartecipantMessage::Bid(id, rnd.gen_range(min_bet..min_price + 50));
                    to_auctioneer.send(bid).unwrap();
                } else {
                    to_auctioneer.send(PartecipantMessage::Exit(id)).unwrap();
                    break;
                }
            }
            AuctioneerMessage::NewPrice(price) => {
                let mut rnd = rand::thread_rng();
                let action = rnd.gen_range(0..100);
                let min_bet = price + 1;

                if action >= 10 {
                    let bid = PartecipantMessage::Bid(id, rnd.gen_range(min_bet..price + 50));
                    to_auctioneer.send(bid).unwrap();
                } else {
                    to_auctioneer.send(PartecipantMessage::Exit(id)).unwrap();
                    break;
                }
            }
            AuctioneerMessage::Winner(Some(winner_id)) => {
                if winner_id == id {
                    println!("Player {}: i won!", id);
                } else {
                    println!("Player {}: i lost (winner {}).", id, winner_id);
                }
                break;
            }
            AuctioneerMessage::Winner(None) => {
                println!("Player {}: there are no winners...", id);
                break;
            }
        }
    }
}

pub fn auctioneer_routine(
    n: u32,
    product: Product,
    from_partecipant: mpsc::Receiver<PartecipantMessage>,
    senders_to_partecipants: Vec<mpsc::Sender<AuctioneerMessage>>,
    outcome: Arc<Mutex<AuctionOutcome>>,
) {
    println!("Banditore: in attesa di {} partecipanti...", n);

    for _ in 0..n {
        match from_partecipant.recv() {
            Ok(PartecipantMessage::Ready(id)) => {
                println!("Banditore: partecipante {} pronto!", id);
            }
            _ => {
                println!("L'asta non può partire per cause di forza maggiore.");
                break;
            }
        }
    }

    println!("Banditore: tutti pronti! Invio descrizione prodotto...");

    for sender in &senders_to_partecipants {
        sender
            .send(AuctioneerMessage::Start {
                product_desc: product.description.clone(),
                min_price: product.start_price,
            })
            .unwrap();
    }

    let mut active_partecipants = n;
    let mut actual_price = product.start_price;
    let mut current_winner: Option<u32> = None;

    while active_partecipants != 0 {
        if let Ok(msg) = from_partecipant.recv() {
            match msg {
                PartecipantMessage::Bid(partecipant_id, bid) => {
                    if bid > actual_price {
                        actual_price = bid;
                        current_winner = Some(partecipant_id);

                        println!(
                            "Banditore: Il Player {} offre {}!",
                            partecipant_id, actual_price
                        );

                        for s in &senders_to_partecipants {
                            let _ = s.send(AuctioneerMessage::NewPrice(actual_price));
                        }
                    }
                }
                PartecipantMessage::Exit(_partecipant_id) => {
                    active_partecipants -= 1;
                }
                _ => {}
            }
        }
    }

    println!("Banditore: Asta conclusa.");

    let final_winner = if actual_price >= product.reserve_price {
        println!("Player {} is the winner!", current_winner.unwrap());
        current_winner
    } else {
        println!("There is no winner..");
        None
    };

    let mut res = outcome.lock().unwrap();
    res.player_id = final_winner;
    res.sold_price_product = if final_winner.is_some() {
        Some(actual_price)
    } else {
        None
    };

    for s in &senders_to_partecipants {
        let _ = s.send(AuctioneerMessage::Winner(final_winner));
    }
}
