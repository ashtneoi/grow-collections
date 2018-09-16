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

    for i in 0..16384 {
        let s = format!("{}", i);
        let x = v.push(s.clone());
        assert_eq!(x, &s);
    }
}
