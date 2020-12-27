#[cfg(test)]
mod test {
    use crate::{solution_part_1, solution_part_2};

    #[test]
    fn test_solution_part_1() {
        assert_eq!(306, solution_part_1("testData.txt"));
    }

    #[test]
    fn test_solution_part_2() {
        assert_eq!(291, solution_part_2("testData.txt"));
    }
}