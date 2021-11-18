pub fn add_one(x: &i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn add_one() {
        assert_eq!(3, super::add_one(&2));
    }
}
