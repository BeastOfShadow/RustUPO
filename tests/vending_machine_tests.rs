#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use esercizi::vending_machine::{VendingMachine, WaitingCard};

    // Helper function to quickly create a machine ready for the tests
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
    fn test_purchase_completed_successfully() {
        let mut machine = setup_machine("Coffee", 5);

        loop {
            machine.insert_card();
            machine.select_product("Coffee".to_string());
            if machine.product_erogation.is_some() {
                break;
            }
        }

        assert_eq!(machine.product_erogation, Some("Coffee".to_string()));
        machine.confirm_selection();
        machine.product_retrieve();

        assert_eq!(*machine.product.get("Coffee").unwrap(), 4);
        assert_eq!(machine.product_erogation, None);
    }

    #[test]
    fn test_product_out_of_stock() {
        let mut machine = setup_machine("Water", 0);

        loop {
            machine.insert_card();
            machine.select_product("Water".to_string());
            if machine.product_erogation.is_some() {
                break;
            }
        }

        machine.confirm_selection();
        machine.cancel();

        assert_eq!(*machine.product.get("Water").unwrap(), 0);
    }

    #[test]
    fn test_invalid_operations_ignored() {
        let mut machine = setup_machine("Snack", 2);

        machine.confirm_selection();

        assert_eq!(machine.product_erogation, None);
        assert_eq!(*machine.product.get("Snack").unwrap(), 2);
    }

    #[test]
    fn test_cancel_from_card_accepted() {
        let mut machine = setup_machine("Juice", 1);

        loop {
            machine.insert_card();
            machine.cancel();
            break;
        }
        
        assert_eq!(machine.product_erogation, None);
    }
}