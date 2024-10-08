use backend::Error;

#[test]
fn test_custom_error() {
    let error_message = "Something went wrong";
    let error = Error::custom(error_message);

    // Check that the error is of type `Error::Custom` and contains the correct message
    let Error::Custom(msg) = error;
    assert_eq!(msg, error_message);
}

#[test]
fn test_error_from_str() {
    let error_message = "Error from &str";
    let error: Error = error_message.into(); // This uses `From<&str>` for `Error`

    // Check that the error is of type `Error::Custom` and contains the correct message
    let Error::Custom(msg) = error;
    assert_eq!(msg, error_message);
}