extern crate cowsay_rs as cowsay;

#[test]
fn test_basic_multiline_message() {
    let input = "One\nTwo\nThree";
    let output = " _______\n/ One   \\\n| Two   |\n\\ Three /\n -------".to_string();
    assert_eq!(cowsay::message_box(input, 1, false, false), output);
}

#[test]
fn test_basic_singleline_message() {
    let input = "This is a line.";
    let output = " _________________\n< This is a line. >\n -----------------".to_string();
    assert_eq!(cowsay::message_box(input, 1, false, false), output);
}
