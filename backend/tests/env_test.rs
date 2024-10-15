use dotenv::dotenv;
use std::env;

/// This test verifies that the environment variable "TEST_KEY" is set correctly
/// and matches the expected value "TEST_VALUE".
///
/// The `.env` file should include:
/// ```
/// TEST_KEY=BACKEND_TEST_VALUE
/// ```
///
/// The test loads the `.env` file, retrieves the value of "TEST_KEY",
/// and compares it with the expected value "TEST_VALUE".

#[test]
fn test_value() {
    // Load the .env file, if it exists.
    dotenv().ok();

    // Retrieve the environment variable "TEST_KEY".
    // If the variable is not found, the test will panic with the given error message.
    let test_value = env::var("TEST_KEY").expect("TEST_KEY must be set in .env file");

    // Assert that the value of "TEST_KEY" is equal to "TEST_VALUE".
    // The test passes if the two values are equal.
    assert_eq!(test_value, String::from("BACKEND_TEST_VALUE"));
}
