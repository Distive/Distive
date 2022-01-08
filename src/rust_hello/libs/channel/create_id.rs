use nanoid::nanoid;

pub fn create_id() -> String {
    let alphabet: [char; 36] = [
        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    nanoid!(5, &alphabet)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_id() {
        let id = create_id();
        assert_eq!(id.len(), 5);
    }

    #[test]
    fn not_contain_delimiter() {
        let id = create_id();
        assert!(!id.contains('_'));
    }
}
