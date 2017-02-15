use std::any::Any;
use std::collections::HashMap;
use std::hash::Hash;

pub trait Cache<K> {
  fn save<T>(&mut self, key: K, value: T) where T: Any + 'static;
  fn get<T>(&self, key: &K) -> Option<&T> where T: Any + 'static;
  fn remove<T>(&mut self, key: &K) -> Option<T> where T: Any + 'static;
  fn clear(&mut self);
}

pub struct HashCache<K> {
  items: HashMap<K, Box<Any>>
}

impl<K> HashCache<K> where K: Eq + Hash {
  pub fn new() -> Self {
    HashCache {
      items: HashMap::new()
    }
  }
}

impl<K> Default for HashCache<K> where K: Eq + Hash {
  fn default() -> Self {
    Self::new()
  }
}

impl<K> Cache<K> for HashCache<K> where K: Eq + Hash {
  fn save<T>(&mut self, key: K, value: T) where T: Any + 'static {
    self.items.insert(key, Box::new(value));
  }

  fn get<T>(&self, key: &K) -> Option<&T> where T: Any + 'static {
    self.items.get(key).and_then(|a| { a.downcast_ref::<T>() })
  }

  fn remove<T>(&mut self, key: &K) -> Option<T> where T: Any + 'static {
    self.items.remove(key).and_then(|any| any.downcast()
  }

  fn clear(&mut self) {
    self.items.clear();
  }
}

pub struct DummyCache;

impl DummyCache {
  pub fn new() -> Self {
    DummyCache
  }
}

impl Default for DummyCache {
  fn default() -> Self {
    DummyCache
  }
}

impl<K> Cache<K> for DummyCache {
  fn save<T>(&mut self, _: K, _: T) where T: Any + 'static {
  }

  fn get<T>(&self, _: &K) -> Option<&T> where T: Any + 'static {
    None
  }

  fn remove<T>(&mut self, _: &K) -> Option<T> where T: Any + 'static {
    false
  }

  fn clear(&mut self) {
  }
}
