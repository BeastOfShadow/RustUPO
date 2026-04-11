#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use esercizi::vending_machine::{VendingMachine, WaitingCard};

    // Funzione di supporto per creare rapidamente una macchina pronta per i test
    fn setup_machine(product_name: &str, quantity: i32) -> VendingMachine {
        let mut product = HashMap::new();
        product.insert(product_name.to_string(), quantity);
        
        VendingMachine {
            product,
            product_erogation: None,
            status: Box::new(WaitingCard),
        }
    }

    #[test]
    fn test_acquisto_completato_con_successo() {
        let mut machine = setup_machine("Caffe", 5);
        
        loop {
            machine.insert_card();
            machine.select_product("Caffe".to_string());
            if machine.product_erogation.is_some() {
                break;
            }
        }
        
        assert_eq!(machine.product_erogation, Some("Caffe".to_string()));
        machine.confirm_selection();
        machine.product_retrieve();
        
        assert_eq!(*machine.product.get("Caffe").unwrap(), 4);
        assert_eq!(machine.product_erogation, None);
    }

    #[test]
    fn test_prodotto_esaurito() {
        let mut machine = setup_machine("Acqua", 0);
        
        loop {
            machine.insert_card();
            machine.select_product("Acqua".to_string());
            if machine.product_erogation.is_some() {
                break;
            }
        }

        machine.confirm_selection();
        machine.cancel();
        
        assert_eq!(*machine.product.get("Acqua").unwrap(), 0);
    }

    #[test]
    fn test_operazioni_non_valide_ignorate() {
        let mut machine = setup_machine("Snack", 2);
        
        machine.confirm_selection();
        
        assert_eq!(machine.product_erogation, None);
        assert_eq!(*machine.product.get("Snack").unwrap(), 2);
    }

    #[test]
    fn test_operazione_cancel_da_carta_accettata() {
        let mut machine = setup_machine("Succo", 1);
        
        loop {
            machine.insert_card();
            machine.cancel();
            break;
        }
        
        assert_eq!(machine.product_erogation, None);
    }
}