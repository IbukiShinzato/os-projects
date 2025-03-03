#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_record_keys() {
        // cargo test -- --nocapture
        println!("\nPlease input test\n");

        let simulated_input = "test";

        let result = record_keys().expect("Recording keys failed");

        assert_eq!(result, simulated_input);
    }

    #[test]
    // for getlab-ci
    fn test_for_ci() {
        let simulated_input = "test";

        let result = record_keys_with_input(simulated_input).expect("Recording keys failed");

        assert_eq!(result, simulated_input);
    }
}
