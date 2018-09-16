use GrowVec;

#[test]
fn test_simple() {
    let mut v = GrowVec::new();
    let a = v.push("aaa".to_string());
    let b = v.push("bbb".to_string());
    let c = v.push("ccc".to_string());
    assert_eq!(a, "aaa");
    assert_eq!(b, "bbb");
    assert_eq!(c, "ccc");

    static COUNT: usize = 16384;
    let mut refs = Vec::with_capacity(COUNT);
    for i in 0..COUNT {
        let x = v.push(format!("{}", i));
        refs.push(x);
    }
    for i in 0..COUNT {
        assert_eq!(
            refs[i],
            &format!("{}", i),
        );
    }
}

#[test]
fn test_with_capacity() {
    let mut v = GrowVec::with_capacity(1024);

    static COUNT: usize = 16384;
    let mut refs = Vec::with_capacity(COUNT);
    for i in 0..COUNT {
        let x = v.push(format!("{}", i));
        refs.push(x);
    }
    for i in 0..COUNT {
        assert_eq!(
            refs[i],
            &format!("{}", i),
        );
    }
}
