#[cfg(test)]
mod tests {
    use esercizi::person::People;

    fn setup_people() -> People {
        People::new() // Instead of People(Vec::new())
    }

    #[test]
    fn test_add_and_search_person() {
        let mut family = setup_people();

        let result = family.add_person("Mario");
        assert!(result.is_ok());

        let duplicate_result = family.add_person("Mario");
        assert!(duplicate_result.is_err());

        let search_result = family.search_person("Mario");
        assert!(search_result.is_some());
        assert_eq!(search_result.unwrap().borrow().name, "Mario");

        assert!(family.search_person("Luigi").is_none());
    }

    #[test]
    fn test_parent_child_links() {
        let mut family = setup_people();

        family.add_person("Child").unwrap();
        family.add_person("Mother").unwrap();
        family.add_person("Father").unwrap();

        family.set_mother("Child", "Mother");
        family.set_father("Child", "Father");

        let child = family.search_person("Child").unwrap();
        let mother = family.search_person("Mother").unwrap();
        let father = family.search_person("Father").unwrap();

        assert_eq!(child.borrow().mother.as_ref().unwrap().borrow().name, "Mother");
        assert_eq!(child.borrow().father.as_ref().unwrap().borrow().name, "Father");

        assert_eq!(mother.borrow().children.len(), 1);
        assert_eq!(mother.borrow().children[0].borrow().name, "Child");

        assert_eq!(father.borrow().children.len(), 1);
        assert_eq!(father.borrow().children[0].borrow().name, "Child");
    }

    #[test]
    fn test_descendants_recursion_and_no_duplicates() {
        let mut family = setup_people();

        family.add_person("Grandfather").unwrap();
        family.add_person("Uncle").unwrap();
        family.add_person("Father").unwrap();
        family.add_person("Child1").unwrap();
        family.add_person("Child2").unwrap();

        family.set_father("Father", "Grandfather");
        family.set_father("Uncle", "Grandfather");

        family.set_father("Child1", "Father");
        family.set_father("Child2", "Father");

        let grandfather_descendants = family.descendants("Grandfather");

        assert_eq!(grandfather_descendants.len(), 4);

        assert!(grandfather_descendants.contains(&"Father".to_string()));
        assert!(grandfather_descendants.contains(&"Uncle".to_string()));
        assert!(grandfather_descendants.contains(&"Child1".to_string()));
        assert!(grandfather_descendants.contains(&"Child2".to_string()));
    }
}
