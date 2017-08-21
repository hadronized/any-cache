use std::any::{Any, TypeId};
use std::collections::hash_map::{DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};

/// A cache that can store abitrary values and namespace them by key types.
pub trait Cache {
  fn save<K>(&mut self, key: K, value: K::Target) where K::Target: Any + 'static, K: CacheKey;
  fn get<K>(&self, key: &K) -> Option<&K::Target> where K::Target: Any + 'static, K: CacheKey;
  fn remove<K>(&mut self, key: &K) -> Option<K::Target> where K::Target: Any + 'static, K: CacheKey;
  fn clear(&mut self);
}

/// A key that is usable in a cache.
///
/// Cache keys are required to declare the type of values they reference. This is needed to
/// implement type-level namespacing.
pub trait CacheKey: 'static + Hash {
  type Target;
}

/// An implementation of a cache with a `HashMap`.
pub struct HashCache {
  items: HashMap<u64, Box<Any>>
}

impl HashCache {
  pub fn new() -> Self {
    HashCache {
      items: HashMap::new()
    }
  }
}

impl Default for HashCache {
  fn default() -> Self {
    Self::new()
  }
}

impl Cache for HashCache {
  fn save<K>(&mut self, key: K, value: K::Target) where K::Target: Any + 'static, K: CacheKey {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    TypeId::of::<K>().hash(&mut hasher);
    self.items.insert(hasher.finish(), Box::new(value));
  }

  fn get<K>(&self, key: &K) -> Option<&K::Target> where K::Target: Any + 'static, K: CacheKey {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    TypeId::of::<K>().hash(&mut hasher);
    self.items.get(&hasher.finish()).and_then(|a| { a.downcast_ref::<K::Target>() })
  }

  fn remove<K>(&mut self, key: &K) -> Option<K::Target> where K::Target: Any + 'static, K: CacheKey {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    TypeId::of::<K>().hash(&mut hasher);
    self.items.remove(&hasher.finish()).and_then(|anybox| anybox.downcast().ok()).map(|b| *b)
  }

  fn clear(&mut self) {
    self.items.clear();
  }
}

/// An implementation of a cache that actually doesn’t cache at all.
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

impl Cache for DummyCache {
  fn save<K>(&mut self, _: K, _: K::Target) where K::Target: Any + 'static, K: CacheKey {
  }

  fn get<K>(&self, _: &K) -> Option<&K::Target> where K::Target: Any + 'static, K: CacheKey {
    None
  }

  fn remove<K>(&mut self, _: &K) -> Option<K::Target> where K::Target: Any + 'static, K: CacheKey {
    None
  }

  fn clear(&mut self) {
  }
}
