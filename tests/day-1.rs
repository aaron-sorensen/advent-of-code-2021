#[cfg(test)]
mod day_1 {
    use advent::{get_increment_count, get_increment_window_count, get_input};

    const DAY: &str = "day-1";

    #[test]
    fn it_counts_increments() {
        let input = get_input(DAY);
        let result = get_increment_count(&input.test);
        assert_eq!(7, result);
    }

    #[test]
    fn it_counts_window_increments() {
        let input = get_input(DAY);
        let result = get_increment_window_count(&input.test, 3);
        assert_eq!(5, result);
    }

    #[test]
    fn it_maintains_correct_answers() {
        let input = get_input(DAY);
        let increments = get_increment_count(&input.real);
        let window_increments = get_increment_window_count(&input.real, 3);
        assert_eq!(1466, increments);
        assert_eq!(1491, window_increments);
    }
}
