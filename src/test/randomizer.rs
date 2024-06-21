use crate::create_password;

#[test]
fn test_create_passsword() {
    let pw_length: usize = 10;
    let actual = create_password(pw_length);
    let actual_size: usize = actual.chars().count();

    assert_eq!(pw_length, actual_size);
}
