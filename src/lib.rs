pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_maybe() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another(){
        panic!("Make this fail");

        assert!(true);
    }
}
