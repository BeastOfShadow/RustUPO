use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub mother: Option<Rc<RefCell<Person>>>,
    pub father: Option<Rc<RefCell<Person>>>,
    pub children: Vec<Rc<RefCell<Person>>>,
}

#[derive(Debug)]
pub struct People(Vec<Rc<RefCell<Person>>>);

impl Person {
    pub fn new(name: &str) -> Rc<RefCell<Person>> {
        Rc::new(RefCell::new(Person {
            name: name.to_string(),
            mother: None,
            father: None,
            children: Vec::new(),
        }))
    }
}

impl People {
    pub fn new() -> Self {
        People(Vec::new())
    }

    pub fn add_person(&mut self, name: &str) -> Result<Rc<RefCell<Person>>, String> {
        for person in &self.0 {
            if person.borrow().name == name {
                return Err(format!("Person with name {} already exists.", name));
            }
        }

        let new_person = Person::new(name);
        self.0.push(new_person.clone());

        Ok(new_person)
    }

    pub fn search_person(&self, name: &str) -> Option<Rc<RefCell<Person>>> {
        for person in &self.0 {
            if person.borrow().name == name {
                return Some(person.clone());
            }
        }

        None
    }

    pub fn set_mother(&self, child: &str, mother: &str) {
        if let (Some(exist_mother), Some(exist_child)) =
            (self.search_person(mother), self.search_person(child))
        {
            exist_child.borrow_mut().mother = Some(exist_mother.clone());
            exist_mother.borrow_mut().children.push(exist_child.clone());
        }
    }

    pub fn set_father(&self, child: &str, father: &str) {
        if let (Some(exist_father), Some(exist_child)) =
            (self.search_person(father), self.search_person(child))
        {
            exist_child.borrow_mut().father = Some(exist_father.clone());
            exist_father.borrow_mut().children.push(exist_child.clone());
        }
    }

    pub fn print(&self, name: &str) {
        if let Some(searched_person) = self.search_person(name) {
            let person = searched_person.borrow();
            println!("Person: {}", person.name);

            if let Some(searched_father) = &person.father {
                println!("Father: {}", searched_father.borrow().name);
            } else {
                println!("Father: unknown");
            }

            if let Some(searched_mother) = &person.mother {
                println!("Mother: {}", searched_mother.borrow().name);
            } else {
                println!("Mother: unknown");
            }
        } else {
            println!("Person not found.");
        }
    }

    pub fn descendants(&self, name: &str) -> Vec<String> {
        if let Some(person) = self.search_person(name) {
            let mut result = Vec::new();
            Self::rec_descendants(person, &mut result);

            result
        } else {
            Vec::new()
        }
    }

    fn rec_descendants(person: Rc<RefCell<Person>>, list: &mut Vec<String>) {
        for child in &person.borrow().children {
            let child_name = child.borrow().name.clone();

            // If the name is not already in the vector, add it
            if !list.contains(&child_name) {
                list.push(child_name);
                // Recursive call for this child's children
                Self::rec_descendants(child.clone(), list);
            }
        }
    }
}
