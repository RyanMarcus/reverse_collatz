mod recursive;
mod triangular_expansion;
mod iterative_search;

#[cfg(test)]
mod tests {

    #[test]
    fn rec_small_numbers() {
        assert_eq!(super::recursive::calculate(6), 10);
        assert_eq!(super::recursive::calculate(45), 361);
        //assert_eq!(super::recursive::calculate(260), 18514);
    }

    #[test]
    fn tri_exp_small_numbers() {
        assert_eq!(super::triangular_expansion::calculate(6), 10);
        assert_eq!(super::triangular_expansion::calculate(45), 361);
        //assert_eq!(super::triangular_expansion::calculate(260), 18514);
    }

    #[test]
    fn iterative_search() {
        assert_eq!(super::iterative_search::calculate(6), 10);
        assert_eq!(super::iterative_search::calculate(45), 361);
        assert_eq!(super::iterative_search::calculate(260), 18514);
    }
}
