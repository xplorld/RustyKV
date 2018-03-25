
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn basic_put_get() {
    let mut kv = ::KV::new();
    kv.put(String::from("a"), String::from("123")); //have to copy the key and value in put
    let key = "a";
    assert_eq!(kv.get(&key), Some("123"));
}