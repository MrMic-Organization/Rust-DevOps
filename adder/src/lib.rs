pub fn add_two(a: i32) -> i32 {
    internal_addler(a, 2)
}

fn internal_addler(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_addler(2, 2));
    }
}
