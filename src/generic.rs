fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[test]
fn generic_for_one_type() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    assert_eq!(*result, 100);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    assert!(*result == 6000);
}