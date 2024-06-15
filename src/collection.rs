
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
