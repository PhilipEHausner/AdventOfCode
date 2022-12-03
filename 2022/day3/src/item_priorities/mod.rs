pub fn get_item_priority(c: char) -> u64 {
    match c {
        _ if c.is_lowercase() => c as u64 - 96,
        _ if c.is_uppercase() => c as u64 - 64 + 26,
        _ => panic!("Invalid item in backpack."),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_item_priority() {
        assert_eq!(get_item_priority('a'), 1);
        assert_eq!(get_item_priority('A'), 27);
        assert_eq!(get_item_priority('d'), 4);
        assert_eq!(get_item_priority('g'), 7);
        assert_eq!(get_item_priority('Z'), 52);
        assert_eq!(get_item_priority('B'), 28);
    }
}
