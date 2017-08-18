extern crate any_cache;

use any_cache::{Cache, CacheKey, HashCache};

#[derive(Clone, Eq, Hash, PartialEq)]
struct CustomKey(&'static str);

impl CacheKey for CustomKey {
  type Target = u32;
}

#[test]
fn new_hash_cache() {
  let _ = HashCache::new();
}
 
#[test]
fn new_hash_cache_custom_key() {
  let cache = HashCache::new();

  assert_eq!(cache.get(&CustomKey("a")), None);
}


#[test]
fn save() {
  let mut cache = HashCache::new();

  cache.save(CustomKey("a"), 1);
  cache.save(CustomKey("a"), 2);
  cache.save(CustomKey("b"), 3);
  cache.save(CustomKey("c"), 4);
}

#[test]
fn save_get() {
  let mut cache = HashCache::new();

  let key = CustomKey("key");
  cache.save(key.clone(), 42);

  assert_eq!(cache.get(&key), Some(&42));
}

#[test]
fn several_types_save_and_gotten() {
  let mut cache = HashCache::new();

  #[derive(Clone, Eq, Hash, PartialEq)]
  struct CustomKey2(u64);
  
  impl CacheKey for CustomKey2 {
    type Target = bool;
  }

  #[derive(Clone, Eq, Hash, PartialEq)]
  struct CustomKey3(String);
  
  impl CacheKey for CustomKey3 {
    type Target = Option<String>;
  }

  #[derive(Clone, Eq, Hash, PartialEq)]
  struct CustomKey4(u64);
  
  impl CacheKey for CustomKey4 {
    type Target = String;
  }

  let key1 = CustomKey("hey");
  let key2 = CustomKey2(12);
  let key3 = CustomKey3("foobar".to_owned());
  let key4 = CustomKey4(12);

  cache.save(key1.clone(), 42);
  cache.save(key2.clone(), false);
  cache.save(key3.clone(), None);
  cache.save(key4.clone(), "yay".to_owned());


  assert_eq!(cache.get(&key1), Some(&42));
  assert_eq!(cache.get(&key2), Some(&false));
  assert_eq!(cache.get(&key3), Some(&None));
  assert_eq!(cache.get(&key4), Some(&"yay".to_owned()));
}
