mod recursive;

#[cfg(test)]
mod tests {

    #[test]
    fn small_numbers() {
        assert_eq!(super::recursive::calculate(6), 10);
        assert_eq!(super::recursive::calculate(45), 361);
        //assert_eq!(super::recursive::calculate(260), 18514);
    }
}
