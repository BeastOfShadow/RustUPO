#[cfg(test)]
mod tests {
    use esercizi::transpose::{min, transpose};

    #[test]
    fn test_min() {
        let fruits = vec!["mele", "arance", "pere", "albicocche", "kiwi", "limoni"];
        let mut str_fuits = vec![];

        for fruit in fruits {
            str_fuits.push(fruit.to_string());
        }

        let mut res = min(&str_fuits);
        assert_eq!(res, 4);
        assert_ne!(res, 7);
        assert_ne!(res, 5);
        str_fuits.remove(0);
        assert_eq!(res, 4);
        str_fuits.remove(1);
        str_fuits.remove(2);

        res = min(&str_fuits);
        assert_eq!(res, 6);
    }

    #[test]
    fn test_transpose_empty() {
        let v: Vec<String> = vec![];
        let res = transpose(&v);
        assert_eq!(res, Vec::<String>::new());
    }

    #[test]
    fn test_transpose_single_string() {
        let v = vec!["hello".to_string()];
        let res = transpose(&v);
        let expected = vec!["h", "e", "l", "l", "o"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();
        assert_eq!(res, expected);
    }

    #[test]
    fn test_transpose_all_same_length() {
        let v = vec!["abc".to_string(), "def".to_string(), "ghi".to_string()];
        let res = transpose(&v);
        let expected = vec!["adg", "beh", "cfi"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();
        assert_eq!(res, expected);
    }

    #[test]
    fn test_transpose_varying_lengths() {
        let v = vec!["a".to_string(), "bc".to_string(), "def".to_string()];
        // lunghezza minima = 1 → solo prima colonna
        let res = transpose(&v);
        let expected = vec!["abd".to_string()];
        assert_eq!(res, expected);
    }

    #[test]
    fn test_transpose_with_empty_strings() {
        let v = vec!["".to_string(), "abc".to_string(), "de".to_string()];
        // lunghezza minima = 0 → nessuna colonna
        let res = transpose(&v);
        let expected: Vec<String> = vec![];
        assert_eq!(res, expected);
    }

    #[test]
    fn test_transpose_single_empty_string() {
        let v = vec!["".to_string()];
        let res = transpose(&v);
        let expected: Vec<String> = vec![];
        assert_eq!(res, expected);
    }

    #[test]
    fn test_transpose_does_not_modify_original() {
        let original = vec!["apple".to_string(), "banana".to_string()];
        let original_copy = original.clone();
        let _ = transpose(&original);
        assert_eq!(original, original_copy);
    }
}