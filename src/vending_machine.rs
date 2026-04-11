use std::collections::HashMap;

use rand::Rng;

pub trait MachineStatus {
    fn insert_card(self: Box<Self>, machine: &mut VendingMachine) -> Box<dyn MachineStatus>;
    fn select_product(self: Box<Self>, machine: &mut VendingMachine, prod: String) -> Box<dyn MachineStatus>;
    fn confirm_selection(self: Box<Self>, machine: &mut VendingMachine) -> Box<dyn MachineStatus>;
    fn cancel(self: Box<Self>, machine: &mut VendingMachine) -> Box<dyn MachineStatus>;
    fn product_retrieve(self: Box<Self>, machine: &mut VendingMachine) -> Box<dyn MachineStatus>;
}

pub struct WaitingCard;
pub struct AcceptedCard;
pub struct SelectedProduct;
pub struct OutOfStockProduct;
pub struct AvailableProduct;

pub struct VendingMachine {
    pub product: HashMap<String, i32>,
    pub product_erogation: Option<String>,
    pub status: Box<dyn MachineStatus>
}

impl MachineStatus for WaitingCard {
    fn insert_card(self: Box<Self>, _machine: &mut VendingMachine) -> Box<dyn MachineStatus> {
        match rand::thread_rng().gen_range(0..2) {
            0=> {
                println!("Accepted card!");
                Box::new(AcceptedCard)
            },
            1 => {
                println!("Card not accepted!");
                self
            },
            _ => {
                println!("Invalid operation.");
                self
            }
        }
    }

    fn select_product(self: Box<Self>, _machine: &mut VendingMachine, _prod: String) -> Box<dyn MachineStatus> {
        println!("Invalid operation.");
        self
    }

    fn confirm_selection(self: Box<Self>, _machine: &mut VendingMachine) -> Box<dyn MachineStatus> {
        println!("Invalid operation.");
        self
    }

    fn cancel(self: Box<Self>, _machine: &mut VendingMachine) -> Box<dyn MachineStatus> {
        println!("Invalid operation.");
        self
    }

    fn product_retrieve(self: Box<Self>, _machine: &mut VendingMachine) -> Box<dyn MachineStatus> {
        println!("Invalid operation.");
        self
    }
}

impl MachineStatus for AcceptedCard {
    fn insert_card(self: Box<Self>, _machine: &mut VendingMachine) -> Box<dyn MachineStatus> {
        println!("Invalid operation.");
        self
    }

    fn select_product(self: Box<Self>, machine: &mut VendingMachine, prod: String) -> Box<dyn MachineStatus> {
        if machine.product.contains_key(&prod) {
            println!("The product {} is still avilable!", prod);
            machine.product_erogation = Some(prod);
            Box::new(SelectedProduct)
        } else {
            println!("Choose another product...");
            self
        }
    }

    fn confirm_selection(self: Box<Self>, _machine: &mut VendingMachine) -> Box<dyn MachineStatus> {
        println!("Invalid operation.");
        self
    }

    fn cancel(self: Box<Self>, _machine: &mut VendingMachine) -> Box<dyn MachineStatus> {
        println!("Cancelled operation.");
        Box::new(WaitingCard)
    }

    fn product_retrieve(self: Box<Self>, _machine: &mut VendingMachine) -> Box<dyn MachineStatus> {
        println!("Invalid operation.");
        self
    }
}

impl MachineStatus for SelectedProduct {
    fn insert_card(self: Box<Self>, _machine: &mut VendingMachine) -> Box<dyn MachineStatus> {
        println!("Invalid operation.");
        self
    }

    fn select_product(self: Box<Self>, _machine: &mut VendingMachine, _prod: String) -> Box<dyn MachineStatus> {
        println!("Invalid operation.");
        self
    }

    fn confirm_selection(self: Box<Self>, machine: &mut VendingMachine) -> Box<dyn MachineStatus> {
        if let Some(product_name) = &machine.product_erogation {
            if let Some(&quantity) = machine.product.get(product_name) {
                if quantity > 0 {
                    println!("Erogated product: {}", product_name);
                    Box::new(AvailableProduct)
                } else {
                    println!("Product {} out of stock", product_name);
                    Box::new(OutOfStockProduct)
                }
            } else {
                println!("Invalid operation");
                self
            }
        } else {
            println!("Invalid operation");
            self
        }
    }

    fn cancel(self: Box<Self>, _machine: &mut VendingMachine) -> Box<dyn MachineStatus> {
        println!("Cancelled operation.");
        Box::new(WaitingCard)
    }

    fn product_retrieve(self: Box<Self>, _machine: &mut VendingMachine) -> Box<dyn MachineStatus> {
        println!("Invalid operation.");
        self
    }
}

impl MachineStatus for OutOfStockProduct {
    fn insert_card(self: Box<Self>, _machine: &mut VendingMachine) -> Box<dyn MachineStatus> {
        println!("Invalid operation.");
        self
    }

    fn select_product(self: Box<Self>, _machine: &mut VendingMachine, _prod: String) -> Box<dyn MachineStatus> {
        println!("Invalid operation.");
        self
    }

    fn confirm_selection(self: Box<Self>, _machine: &mut VendingMachine) -> Box<dyn MachineStatus> {
        println!("Invalid operation.");
        self
    }

    fn cancel(self: Box<Self>, _machine: &mut VendingMachine) -> Box<dyn MachineStatus> {
        println!("Cancelled operation.");
        Box::new(WaitingCard)
    }

    fn product_retrieve(self: Box<Self>, _machine: &mut VendingMachine) -> Box<dyn MachineStatus> {
        println!("Invalid operation.");
        self
    }
}

impl MachineStatus for AvailableProduct {
    fn insert_card(self: Box<Self>, _machine: &mut VendingMachine) -> Box<dyn MachineStatus> {
        println!("Invalid operation.");
        self
    }

    fn select_product(self: Box<Self>, _machine: &mut VendingMachine, _prod: String) -> Box<dyn MachineStatus> {
        println!("Invalid operation.");
        self
    }

    fn confirm_selection(self: Box<Self>, _machine: &mut VendingMachine) -> Box<dyn MachineStatus> {
        println!("Invalid operation.");
        self
    }

    fn cancel(self: Box<Self>, _machine: &mut VendingMachine) -> Box<dyn MachineStatus> {
        println!("Invalid operation.");
        self
    }

    fn product_retrieve(self: Box<Self>, machine: &mut VendingMachine) -> Box<dyn MachineStatus> {
        if let Some(product_name) = &machine.product_erogation.take() {
            if let Some(quantity) = machine.product.get_mut(product_name) {
                *quantity -= 1;
                println!("Erogated product {}", product_name);
                Box::new(WaitingCard)
            } else {
                println!("Invalid operation");
                self
            }
        } else {
            println!("Invalid operation");
            self
        }
    }
}

impl VendingMachine {
    pub fn insert_card(&mut self) {
        let current_status = std::mem::replace(&mut self.status, Box::new(WaitingCard));
        self.status = current_status.insert_card(self);
    }

    pub fn select_product(&mut self, prod: String) {
        let current_status = std::mem::replace(&mut self.status, Box::new(WaitingCard));
        self.status = current_status.select_product(self, prod);
    }

    pub fn confirm_selection(&mut self) {
        let current_status = std::mem::replace(&mut self.status, Box::new(WaitingCard));
        self.status = current_status.confirm_selection(self);
    }

    pub fn cancel(&mut self) {
        let current_status = std::mem::replace(&mut self.status, Box::new(WaitingCard));
        self.status = current_status.cancel(self);
    }

    pub fn product_retrieve(&mut self) {
        let current_status = std::mem::replace(&mut self.status, Box::new(WaitingCard));
        self.status = current_status.product_retrieve(self);
    }
}