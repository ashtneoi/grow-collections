use GrowHashSet;
use GrowVec;
use std::ptr;

#[test]
fn test_simple() {
    let mut h = GrowHashSet::new();
    let mut v = GrowVec::new();

    let ha = h.insert("aaa".to_string());
    let va = v.push("aaa".to_string());
    let hb = h.insert("bbb".to_string());
    let vb = v.push("bbb".to_string());
    let hc = h.insert("ccc".to_string());
    let vc = v.push("ccc".to_string());
    assert_eq!(ha, "aaa");
    assert_eq!(va, "aaa");
    assert_eq!(hb, "bbb");
    assert_eq!(vb, "bbb");
    assert_eq!(hc, "ccc");
    assert_eq!(vc, "ccc");

    static COUNT: usize = 16384;
    let mut refs = Vec::with_capacity(COUNT);
    for i in 0..COUNT {
        let hx = h.insert(format!("{}", i));
        let vx = v.push(format!("{}", i));
        assert_eq!(hx, vx);
        refs.push(hx);
    }
    for i in 0..COUNT {
        assert_eq!(
            refs[i],
            &format!("{}", i),
        );
    }
}

#[test]
fn test_duplicate() {
    let mut h = GrowHashSet::new();

    let a = h.insert("aaa".to_string());
    let b = h.insert("bbb".to_string());
    let c = h.insert("ccc".to_string());
    let a2 = h.insert("aaa".to_string());
    let b2 = h.insert("bbb".to_string());
    let c2 = h.insert("ccc".to_string());

    assert!(ptr::eq(a, a2));
    assert!(ptr::eq(b, b2));
    assert!(ptr::eq(c, c2));
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
