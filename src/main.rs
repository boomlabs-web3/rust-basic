fn main() {
    use std::mem;

    let mut v = vec![65, 122];
    // `v`의 내용으로 Sting `s`를 생성
    let s = unsafe { String::from_raw_parts(v.as_mut_ptr(), v.len(), v.capacity()) };
    println!("{:?}", v);
    // `v`의 메모리는 `s`에 의해 관리되기 때문에 leak.
    mem::forget(v); // 이 부분을 주석처리하면 double-free error가 생김.
    assert_eq!(s, "Az");
    mem::drop(s); // `s` 는 drop되고, 메모리는 dealloc됨.
}
