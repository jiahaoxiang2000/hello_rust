#[test]
fn test_add() {
    assert_eq!(4, 4);
}

#[test]
fn vector_basic() {
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(6);
    assert_eq!(v, [1, 2, 3, 4, 5, 6]);

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    assert_eq!(v, [150, 82, 107]);
}

#[test]
fn string_basic() {
    let data = "initial contents";
    let s = data.to_string();
    let b = "initial contents".to_string();
    assert_eq!(s, b);

    let c = format!("{s}-{b}");
    assert_eq!(c, "initial contents-initial contents");

    // the String cant be indexed, by the UTF-8 encoding. the char not stored in 1 byte, is encoding the 1 or 2 bytes.
}
