use app;

#[test]
fn greet_name_correct() {
    assert_eq!(app::greet("danny"), "Hello, danny!");
}

#[test]
fn greet_empty_name_correct() {
    assert_eq!(app::greet(""), "please input your name");
}

#[test]
fn greet_single_space_name_correct() {
    assert_eq!(app::greet(" "), "please input your name");
}

#[test]
fn greet_spaces_name_correct() {
    assert_eq!(app::greet("    "), "please input your name");
}