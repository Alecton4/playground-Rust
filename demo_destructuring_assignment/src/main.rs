// REF: https://course.rs/basic/variable.html#%E8%A7%A3%E6%9E%84%E5%BC%8F%E8%B5%8B%E5%80%BC

fn main() {
    let (a, b, c, d, e);

    (a, b) = (1, 2);
    // "_" means match a value, but we do not care about the value. Thus we do not give it meaningful name.
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}

struct Struct {
    e: i32,
}
