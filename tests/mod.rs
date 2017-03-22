extern crate any_cache;

use any_cache::{Cache, HashCache};

#[test]
fn new_hash_cache_std_key() {
  let _ = HashCache::<()>::new();
  let _ = HashCache::<&'static str>::new();
  let _ = HashCache::<String>::new();
}

#[test]
fn new_hash_cache_custom_key() {
  #[derive(Eq, Hash, PartialEq)]
  struct CustomKey;

  let _ = HashCache::<CustomKey>::new();
}

#[test]
fn save() {
  let mut cache = HashCache::new();

  cache.save("a", ());
  cache.save("a", ());
  cache.save("b", ());
  cache.save("c", ());
}

#[test]
fn save_get() {
  let mut cache = HashCache::new();

  cache.save("key", "hey!");
  assert_eq!(cache.get(&"key"), Some(&"hey!"));
}

#[test]
fn several_types_save_and_gotten() {
  let mut cache = HashCache::new();

  cache.save("a", "hey!");
  cache.save("b", 3);
  cache.save("c", false);


  assert_eq!(cache.get(&"a"), Some(&"hey!"));
  assert_eq!(cache.get(&"b"), Some(&3));
  assert_eq!(cache.get(&"c"), Some(&false));
}
