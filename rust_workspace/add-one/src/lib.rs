pub fn add_one_num(x: i32) -> i32 {
    x + 1
}

pub mod random {
    use rand;

    pub fn add_random(x: i32) -> (i32, i32) {
        let random = rand::random();
        let result = x + random;
        (random, result)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
