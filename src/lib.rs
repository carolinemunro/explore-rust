/// Processes a number using a sequence of transformations.
/// 
/// If the number is even, divides it by 3.
/// If the number is odd, multiplies it by 6.
/// Continues until the number reaches 0.
///
/// # Arguments
/// * `mut number` - The starting number to process
///
/// # Returns
/// A vector of all intermediate values during the process
///
/// # Example
/// ```
/// let result = explore_rust::process_number(9);
/// assert!(!result.is_empty());
/// ```
pub fn process_number(mut number: i32) -> Vec<i32> {
    let mut sequence = vec![];
    
    while number != 0 {
        if number % 2 == 0 {
            number = number / 3;
        } else {
            number = number * 6;
        }
        sequence.push(number);
    }
    
    sequence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_number_with_odd_starting_value() {
        let result = process_number(9);
        // 9 is odd -> 9 * 6 = 54
        // 54 is even -> 54 / 3 = 18
        // 18 is even -> 18 / 3 = 6
        // 6 is even -> 6 / 3 = 2
        // 2 is even -> 2 / 3 = 0
        let expected = vec![54, 18, 6, 2, 0];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_process_number_with_even_starting_value() {
        let result = process_number(6);
        // 6 is even -> 6 / 3 = 2
        // 2 is even -> 2 / 3 = 0
        let expected = vec![2, 0];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_process_number_with_one() {
        let result = process_number(1);
        // 1 is odd -> 1 * 6 = 6
        // 6 is even -> 6 / 3 = 2
        // 2 is even -> 2 / 3 = 0
        let expected = vec![6, 2, 0];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_process_number_returns_vector() {
        let result = process_number(4);
        assert!(!result.is_empty());
        assert_eq!(result.last(), Some(&0));
    }

    #[test]
    fn test_process_number_ends_at_zero() {
        let result = process_number(10);
        assert_eq!(result.last(), Some(&0), "Sequence should always end at 0");
    }
}
