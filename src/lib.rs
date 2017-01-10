use std::any::Any;
use std::collections::HashMap;

pub trait Cache {
  fn save<T>(&mut self, key: &str, value: T) where T: Any + 'static;
  fn get<T>(&self, key: &str) -> Option<&T> where T: Any + 'static;
  fn remove(&mut self, key: &str) -> bool;
  fn clear(&mut self);
}

pub struct HashCache {
  items: HashMap<String, Box<Any>>
}

impl Default for HashCache {
  fn default() -> Self {
    HashCache {
      items: HashMap::new()
    }
  }
}

impl Cache for HashCache {
  fn save<T>(&mut self, key: &str, value: T) where T: Any + 'static {
    self.items.insert(key.to_owned(), Box::new(value));
  }

  fn get<T>(&self, key: &str) -> Option<&T> where T: Any + 'static {
    self.items.get(key).and_then(|a| { a.downcast_ref::<T>() })
  }

  fn remove(&mut self, key: &str) -> bool {
    self.items.remove(key).is_some()
  }

  fn clear(&mut self) {
    self.items.clear();
  }
}

pub struct DummyCache;

impl Default for DummyCache {
  fn default() -> Self {
    DummyCache
  }
}

impl Cache for DummyCache {
  fn save<T>(&mut self, _: &str, _: T) where T: Any + 'static {
  }

  fn get<T>(&self, _: &str) -> Option<&T> where T: Any + 'static {
    None
  }

  fn remove(&mut self, _: &str) -> bool {
    false
  }

  fn clear(&mut self) {
  }
}
