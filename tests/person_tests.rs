#[cfg(test)]
mod tests {
    use esercizi::person::People;

    fn setup_people() -> People {
        People::new() // Invece di People(Vec::new())
    }

    #[test]
    fn test_aggiunta_e_ricerca_persona() {
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
    fn test_legami_genitore_figlio() {
        let mut family = setup_people();
        
        family.add_person("Figlio").unwrap();
        family.add_person("Madre").unwrap();
        family.add_person("Padre").unwrap();

        family.set_mother("Figlio", "Madre");
        family.set_father("Figlio", "Padre");

        let figlio = family.search_person("Figlio").unwrap();
        let madre = family.search_person("Madre").unwrap();
        let padre = family.search_person("Padre").unwrap();

        assert_eq!(figlio.borrow().mother.as_ref().unwrap().borrow().name, "Madre");
        assert_eq!(figlio.borrow().father.as_ref().unwrap().borrow().name, "Padre");

        assert_eq!(madre.borrow().children.len(), 1);
        assert_eq!(madre.borrow().children[0].borrow().name, "Figlio");

        assert_eq!(padre.borrow().children.len(), 1);
        assert_eq!(padre.borrow().children[0].borrow().name, "Figlio");
    }

    #[test]
    fn test_discendenti_ricorsione_e_no_duplicati() {
        let mut family = setup_people();
        
        family.add_person("Nonno").unwrap();
        family.add_person("Zio").unwrap();
        family.add_person("Padre").unwrap();
        family.add_person("Figlio1").unwrap();
        family.add_person("Figlio2").unwrap();

        family.set_father("Padre", "Nonno");
        family.set_father("Zio", "Nonno");

        family.set_father("Figlio1", "Padre");
        family.set_father("Figlio2", "Padre");

        let discendenti_nonno = family.descendants("Nonno");
        
        assert_eq!(discendenti_nonno.len(), 4);
        
        assert!(discendenti_nonno.contains(&"Padre".to_string()));
        assert!(discendenti_nonno.contains(&"Zio".to_string()));
        assert!(discendenti_nonno.contains(&"Figlio1".to_string()));
        assert!(discendenti_nonno.contains(&"Figlio2".to_string()));
    }
}