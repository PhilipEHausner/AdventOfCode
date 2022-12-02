#[macro_export]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
        let mut map = ::std::collections::HashMap::new();
        $( map.insert($key, $val); )*
        map
    }}
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hashmap_macro() {
        let caesar = hashmap!['A' => 'G', 'C' => 'A', 'G' => 'T', 'T' => 'C'];

        assert_eq!(caesar[&'A'], 'G');
        assert_eq!(caesar[&'C'], 'A');
        assert_eq!(caesar[&'G'], 'T');
        assert_eq!(caesar[&'T'], 'C');
    }
}

