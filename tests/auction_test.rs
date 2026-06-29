#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex, mpsc};
    use std::thread;

    use esercizi::auction::{
        AuctionOutcome, AuctioneerMessage, PartecipantMessage, Product, auctioneer_routine,
    };

    /// Builds the auctioneer side of the protocol and returns the handles the
    /// test needs to impersonate the participants:
    /// - the sender used to push `PartecipantMessage`s to the auctioneer,
    /// - one receiver per participant to read the auctioneer's broadcasts,
    /// - the join handle of the auctioneer thread,
    /// - the shared `AuctionOutcome`.
    fn setup_auction(
        n: u32,
        start_price: u32,
        reserve_price: u32,
    ) -> (
        mpsc::Sender<PartecipantMessage>,
        Vec<mpsc::Receiver<AuctioneerMessage>>,
        thread::JoinHandle<()>,
        Arc<Mutex<AuctionOutcome>>,
    ) {
        let product = Product {
            start_price,
            reserve_price,
            description: "TestItem".to_string(),
        };

        let outcome = Arc::new(Mutex::new(AuctionOutcome {
            sold_product: product.description.clone(),
            sold_price_product: None,
            player_id: None,
        }));

        let (to_auctioneer, from_partecipant) = mpsc::channel::<PartecipantMessage>();

        let mut senders_to_partecipants = Vec::new();
        let mut receivers = Vec::new();
        for _ in 0..n {
            let (to_partecipant, from_auctioneer) = mpsc::channel::<AuctioneerMessage>();
            senders_to_partecipants.push(to_partecipant);
            receivers.push(from_auctioneer);
        }

        let outcome_clone = outcome.clone();
        let handle = thread::spawn(move || {
            auctioneer_routine(
                n,
                product,
                from_partecipant,
                senders_to_partecipants,
                outcome_clone,
            );
        });

        (to_auctioneer, receivers, handle, outcome)
    }

    /// Single bid above the reserve: that bidder must win at the bid price.
    #[test]
    fn test_winner_above_reserve() {
        let (to_auctioneer, _receivers, handle, outcome) = setup_auction(2, 10, 15);

        to_auctioneer.send(PartecipantMessage::Ready(0)).unwrap();
        to_auctioneer.send(PartecipantMessage::Ready(1)).unwrap();

        // Player 0 bids 20 (> start price 10), player 1 walks away.
        to_auctioneer.send(PartecipantMessage::Bid(0, 20)).unwrap();
        to_auctioneer.send(PartecipantMessage::Exit(1)).unwrap();
        to_auctioneer.send(PartecipantMessage::Exit(0)).unwrap();

        handle.join().unwrap();

        let res = outcome.lock().unwrap();
        assert_eq!(res.player_id, Some(0));
        assert_eq!(res.sold_price_product, Some(20));
    }

    /// Highest bidder wins even when an earlier, lower bid was registered first.
    #[test]
    fn test_highest_bidder_wins() {
        let (to_auctioneer, _receivers, handle, outcome) = setup_auction(2, 10, 15);

        to_auctioneer.send(PartecipantMessage::Ready(0)).unwrap();
        to_auctioneer.send(PartecipantMessage::Ready(1)).unwrap();

        to_auctioneer.send(PartecipantMessage::Bid(0, 20)).unwrap();
        to_auctioneer.send(PartecipantMessage::Bid(1, 30)).unwrap();
        to_auctioneer.send(PartecipantMessage::Exit(0)).unwrap();
        to_auctioneer.send(PartecipantMessage::Exit(1)).unwrap();

        handle.join().unwrap();

        let res = outcome.lock().unwrap();
        assert_eq!(res.player_id, Some(1));
        assert_eq!(res.sold_price_product, Some(30));
    }

    /// Top bid stays below the reserve: the item is not sold.
    #[test]
    fn test_no_winner_below_reserve() {
        let (to_auctioneer, _receivers, handle, outcome) = setup_auction(1, 10, 100);

        to_auctioneer.send(PartecipantMessage::Ready(0)).unwrap();

        // 12 > start price 10, but still below the reserve of 100.
        to_auctioneer.send(PartecipantMessage::Bid(0, 12)).unwrap();
        to_auctioneer.send(PartecipantMessage::Exit(0)).unwrap();

        handle.join().unwrap();

        let res = outcome.lock().unwrap();
        assert_eq!(res.player_id, None);
        assert_eq!(res.sold_price_product, None);
    }

    /// Everyone leaves without bidding: no winner, no price.
    #[test]
    fn test_no_bids_no_winner() {
        let (to_auctioneer, _receivers, handle, outcome) = setup_auction(1, 10, 5);

        to_auctioneer.send(PartecipantMessage::Ready(0)).unwrap();
        to_auctioneer.send(PartecipantMessage::Exit(0)).unwrap();

        handle.join().unwrap();

        let res = outcome.lock().unwrap();
        assert_eq!(res.player_id, None);
        assert_eq!(res.sold_price_product, None);
    }

    /// The auctioneer must broadcast a `Start` then a `NewPrice` after a raise.
    #[test]
    fn test_broadcast_messages() {
        let (to_auctioneer, receivers, handle, _outcome) = setup_auction(1, 10, 15);

        to_auctioneer.send(PartecipantMessage::Ready(0)).unwrap();
        to_auctioneer.send(PartecipantMessage::Bid(0, 25)).unwrap();
        to_auctioneer.send(PartecipantMessage::Exit(0)).unwrap();

        // First message: the opening announcement with the start price.
        match receivers[0].recv().unwrap() {
            AuctioneerMessage::Start { min_price, .. } => assert_eq!(min_price, 10),
            _ => panic!("expected Start message first"),
        }

        // Second message: the new price after the accepted bid.
        match receivers[0].recv().unwrap() {
            AuctioneerMessage::NewPrice(price) => assert_eq!(price, 25),
            _ => panic!("expected NewPrice message after bid"),
        }

        handle.join().unwrap();
    }
}
