use easy_assert::bool_assertions::BooleanAssert;
use rust_algorithms::dynamic_connectivity::quick_find::QuickFind;
use rust_algorithms::dynamic_connectivity::quick_union::QuickUnion;
use rust_algorithms::dynamic_connectivity::union_find::UnionFind;

#[test]
pub fn simple_client() {
    let values = create_values();
    let mut union_find = QuickUnion::new(10);
    println!("====================");
    println!("{}", union_find);
    for (key, value) in &values {
        if !union_find.connected(key, value).expect("should be ok") {
            let _ = union_find.union(key, value);
            println!("p={:?}, q={:?} ", key, value);
        }
    }
    println!("====================");
    println!("{}", union_find);
}

#[test]
pub fn quick_find() {
    let values = create_values();
    let mut union_find = QuickFind::new(10);
    for (key, value) in &values {
        let _ = union_find.union(key, value);
    }

    validate_result(union_find);
}

#[test]
pub fn quick_union() {
    let values = create_values();
    let mut union_find = QuickUnion::new(10);
    for (key, value) in &values {
        let _ = union_find.union(key, value);
    }

    validate_result(union_find);
}

fn validate_result(union_find: (impl UnionFind + Sized)) {
    BooleanAssert::assert_that(union_find.connected(&4, &3).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&4, &4).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&4, &9).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&4, &8).expect("should be Ok")).is_true();

    BooleanAssert::assert_that(union_find.connected(&3, &3).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&3, &4).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&3, &8).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&3, &9).expect("should be Ok")).is_true();

    BooleanAssert::assert_that(union_find.connected(&8, &3).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&8, &4).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&8, &8).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&8, &9).expect("should be Ok")).is_true();

    BooleanAssert::assert_that(union_find.connected(&9, &3).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&9, &4).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&9, &8).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&9, &9).expect("should be Ok")).is_true();

    BooleanAssert::assert_that(union_find.connected(&6, &4).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&6, &3).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&6, &8).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&6, &9).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&3, &6).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&4, &6).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&8, &6).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&9, &6).expect("should be Ok")).is_false();

    BooleanAssert::assert_that(union_find.connected(&7, &4).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&7, &3).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&7, &8).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&7, &9).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&3, &7).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&4, &7).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&8, &7).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&9, &7).expect("should be Ok")).is_false();

    BooleanAssert::assert_that(union_find.connected(&2, &4).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&2, &3).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&2, &8).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&2, &9).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&3, &2).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&4, &2).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&8, &2).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&9, &2).expect("should be Ok")).is_false();

    BooleanAssert::assert_that(union_find.connected(&1, &4).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&1, &3).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&1, &8).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&1, &9).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&3, &1).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&4, &1).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&8, &1).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&9, &1).expect("should be Ok")).is_false();

    BooleanAssert::assert_that(union_find.connected(&5, &4).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&5, &3).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&5, &8).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&5, &9).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&3, &5).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&4, &5).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&8, &5).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&9, &5).expect("should be Ok")).is_false();

    BooleanAssert::assert_that(union_find.connected(&0, &4).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&0, &3).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&0, &8).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&0, &9).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&3, &0).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&4, &0).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&8, &0).expect("should be Ok")).is_false();
    BooleanAssert::assert_that(union_find.connected(&9, &0).expect("should be Ok")).is_false();

    BooleanAssert::assert_that(union_find.connected(&7, &2).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&7, &1).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&7, &6).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&7, &5).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&7, &0).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&7, &7).expect("should be Ok")).is_true();

    BooleanAssert::assert_that(union_find.connected(&2, &2).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&2, &1).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&2, &6).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&2, &5).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&2, &0).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&2, &7).expect("should be Ok")).is_true();

    BooleanAssert::assert_that(union_find.connected(&1, &2).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&1, &1).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&1, &6).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&1, &5).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&1, &0).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&1, &7).expect("should be Ok")).is_true();

    BooleanAssert::assert_that(union_find.connected(&6, &2).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&6, &1).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&6, &6).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&6, &5).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&6, &0).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&6, &7).expect("should be Ok")).is_true();

    BooleanAssert::assert_that(union_find.connected(&5, &2).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&5, &1).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&5, &6).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&5, &5).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&5, &0).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&5, &7).expect("should be Ok")).is_true();

    BooleanAssert::assert_that(union_find.connected(&0, &2).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&0, &1).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&0, &6).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&0, &5).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&0, &0).expect("should be Ok")).is_true();
    BooleanAssert::assert_that(union_find.connected(&0, &7).expect("should be Ok")).is_true();
}

fn create_values() -> Vec<(usize, usize)> {
    let mut values: Vec<(usize, usize)> = Vec::new();
    values.push((4, 3)); //connect
    values.push((3, 8)); //connect
    values.push((6, 5)); //connect
    values.push((9, 4)); //connect
    values.push((2, 1)); //connect
    values.push((8, 9)); //already connected
    values.push((5, 0)); //print
    values.push((7, 2)); //print
    values.push((6, 1)); //print
    values.push((1, 0)); //already connected
    values.push((6, 7)); //already connected
    return values;
}
