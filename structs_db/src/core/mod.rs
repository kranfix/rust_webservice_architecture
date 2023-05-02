use std::{collections::HashMap, hash::Hash};

pub struct Collection<T: Record> {
  len: usize,
  data: HashMap<<T as Record>::Id, T>,
}

impl<T: Record> Default for Collection<T> {
  fn default() -> Self {
    Self {
      len: Default::default(),
      data: Default::default(),
    }
  }
}

pub trait Record: Sized + Clone {
  type Id: Clone + PartialEq + Eq + Hash + std::fmt::Debug;

  fn create_next_id(collection: &mut Collection<Self>) -> Self::Id;

  fn id(&self) -> &Self::Id;
}

impl<T: Record> Collection<T> {
  pub fn len(&self) -> usize {
    self.len
  }

  pub fn is_empty(&self) -> bool {
    self.data.is_empty()
  }

  pub fn create(&mut self, f: impl FnOnce(<T as Record>::Id) -> T) -> Option<T> {
    let id = <T as Record>::create_next_id(self);
    let record = f(id.clone());

    if record.id() == &id {
      let created = self.data.insert(id, record)?;
      self.len += 1;
      Some(created)
    } else {
      None
    }
  }

  pub fn get_all(&self) -> Vec<T> {
    self.data.values().cloned().collect()
  }

  pub fn get_by_id(&self, id: &<T as Record>::Id) -> Option<&T> {
    self.data.get(id)
  }

  pub fn delete(&mut self, id: &<T as Record>::Id) -> Option<T> {
    self.data.remove(id)
  }

  pub fn update(&mut self, id: &<T as Record>::Id, f: impl FnOnce(&mut T)) -> Option<&T> {
    let record = self.data.get_mut(id)?;
    let mut record_to_update = record.clone();
    f(&mut record_to_update);
    if record_to_update.id() == id {
      *record = record_to_update;
      Some(record)
    } else {
      None
    }
  }
}
