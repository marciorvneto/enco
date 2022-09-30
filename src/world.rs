use std::{
    any::{Any, TypeId},
    borrow::BorrowMut,
    collections::HashMap,
};

use crate::custom_errors::*;

pub type EntityId = usize;
pub type ComponentHash = HashMap<TypeId, Box<dyn Any>>;

pub struct World {
    entity_components: HashMap<EntityId, ComponentHash>,
    new_entity_id: EntityId,
}

impl World {
    pub fn new() -> Self {
        World {
            entity_components: HashMap::new(),
            new_entity_id: 0,
        }
    }

    ///
    /// Creates an entity in the current world.
    /// ```
    /// use enco::world::*;
    /// struct MyComponent(i32);
    ///
    /// let mut world = World::new();
    /// let entity_id = world
    ///          .create_entity()
    ///          .with(MyComponent(12))
    ///          .done();
    /// ```
    pub fn create_entity(&mut self) -> &mut Self {
        self.entity_components
            .insert(self.new_entity_id, HashMap::new());
        self
    }

    pub fn with<T: Any>(&mut self, component: T) -> &mut Self {
        self.entity_components
            .get_mut(&self.new_entity_id)
            .unwrap()
            .insert(TypeId::of::<T>(), Box::<T>::new(component));
        self
    }

    pub fn done(&mut self) -> EntityId {
        self.new_entity_id += 1;
        self.new_entity_id - 1
    }

    pub fn num_entities(&self) -> usize {
        self.entity_components.len()
    }

    pub fn num_components(&self, entity_id: &EntityId) -> Result<usize, WorldError> {
        if let Some(components) = self.entity_components.get(&entity_id) {
            return Ok(components.len());
        }
        Err(WorldError::EntityDoesNotExist)
    }

    pub fn query<T: Any>(&self) -> impl Iterator<Item = &T> {
        let query = self
            .entity_components
            .iter()
            .filter_map(|(entity_id, _components)| {
                let component_option = self.get_entity_component::<T>(entity_id);
                component_option
            });
        query
    }

    pub fn query_mut<T: Any>(&mut self) -> impl Iterator<Item = &mut T> {
        let self_ptr = self as *mut Self;
        let query =
            self.entity_components
                .iter_mut()
                .filter_map(move |(entity_id, _components)| unsafe {
                    let component_option = (*self_ptr).get_entity_component_mut::<T>(entity_id);
                    component_option
                });
        query
    }

    ///
    /// Deletes an entity from the world
    ///
    /// ```
    /// use enco::world::*;
    ///
    /// let mut world = World::new();
    /// let entity_id = world
    ///          .create_entity()
    ///          .done();
    /// world.delete_entity(&entity_id).unwrap();
    /// ```
    ///
    pub fn delete_entity(&mut self, entity_id: &EntityId) -> Result<(), WorldError> {
        if let Some(_) = self.entity_components.remove(&entity_id) {
            return Ok(());
        }
        Err(WorldError::DeleteEntity)
    }

    /// Adds a component to an entity
    ///
    /// ```
    /// use enco::world::*;
    /// struct SomeComponent(i32);
    ///
    /// let mut world = World::new();
    /// let entity_id = world
    ///          .create_entity()
    ///          .done();
    /// world.add_component(&entity_id, SomeComponent(1)).unwrap();
    /// ```
    ///
    pub fn add_component<T: Any>(
        &mut self,
        entity_id: &EntityId,
        component: T,
    ) -> Result<(), WorldError> {
        let type_id = TypeId::of::<T>();
        if let Some(components_hash) = self.entity_components.get_mut(entity_id) {
            if components_hash.contains_key(&type_id) {
                return Err(WorldError::EntityAlreadyHasComponent(type_id));
            }
            components_hash.insert(type_id, Box::new(component));
            return Ok(());
        }
        Err(WorldError::EntityAlreadyHasComponent(type_id))
    }

    ///
    /// Deletes a component from an entity
    ///
    /// ```
    /// use enco::world::*;
    /// struct SomeComponent(i32);
    /// struct SomeOtherComponent(i32);
    ///
    /// let mut world = World::new();
    /// let entity_id = world
    ///          .create_entity()
    ///          .with(SomeComponent(12))
    ///          .with(SomeOtherComponent(12))
    ///          .done();
    /// world.delete_component::<SomeComponent>(&entity_id).unwrap();
    /// ```
    ///
    pub fn delete_component<T: Any>(&mut self, entity_id: &EntityId) -> Result<(), WorldError> {
        if let Some(entity_components) = self.entity_components.get_mut(entity_id) {
            if let Some(_) = entity_components.remove(&TypeId::of::<T>()) {
                return Ok(());
            }
            return Err(WorldError::EntityDoesNotHaveComponent(TypeId::of::<T>()));
        }
        Err(WorldError::DeleteEntity)
    }

    pub fn get_entity_component<T: Any>(&self, entity_id: &EntityId) -> Option<&T> {
        let components = self.entity_components.get(&entity_id)?;
        components.get(&TypeId::of::<T>())?.downcast_ref::<T>()
    }

    pub fn get_entity_component_mut<T: Any>(&mut self, entity_id: &EntityId) -> Option<&mut T> {
        let components = self.entity_components.get_mut(&entity_id)?;
        components.get_mut(&TypeId::of::<T>())?.downcast_mut::<T>()
    }

    // Iterators

    ///
    /// Returns an iterator that goes through every entity id
    /// ```
    /// use enco::world::*;
    ///
    /// let mut world = World::new();
    ///
    /// world.create_entity().done();
    /// world.create_entity().done();
    /// for entity_id in world.iter() {
    ///     // Do something here
    /// }
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &usize> {
        self.entity_components.iter().map(|entry| entry.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn create_entity_in_world() {
        let mut world = World::new();
        let entity_id = world.create_entity().with(NodeDrawingComponent(1)).done();
        assert_eq!(entity_id, 0);

        let entity_id_2 = world.create_entity().with(NodeDrawingComponent(2)).done();
        assert_eq!(entity_id_2, 1);

        let c1_box =
            world.entity_components[&entity_id][&TypeId::of::<NodeDrawingComponent>()].as_ref();
        let c1 = c1_box.downcast_ref::<NodeDrawingComponent>().unwrap();
        assert_eq!(c1.0, 1);

        let c2_box =
            world.entity_components[&entity_id_2][&TypeId::of::<NodeDrawingComponent>()].as_ref();
        let c2 = c2_box.downcast_ref::<NodeDrawingComponent>().unwrap();
        assert_eq!(c2.0, 2);
    }

    #[test]
    pub fn delete_entity() -> Result<(), WorldError> {
        let mut world = World::new();

        assert_eq!(world.entity_components.len(), 0);

        let entity_id = world.create_entity().with(NodeDrawingComponent(1)).done();

        world.delete_entity(&entity_id)?;
        assert_eq!(world.entity_components.len(), 0);
        Ok(())
    }

    #[test]
    pub fn world_capacity() -> Result<(), WorldError> {
        let mut world = World::new();

        assert_eq!(world.num_entities(), 0);

        world.create_entity().with(NodeDrawingComponent(1)).done();

        assert_eq!(world.num_entities(), 1);

        let entity_id_2 = world.create_entity().with(NodeDrawingComponent(2)).done();

        assert_eq!(world.num_entities(), 2);

        world.delete_entity(&entity_id_2)?;
        assert_eq!(world.num_entities(), 1);

        world.create_entity().with(NodeDrawingComponent(3)).done();

        assert_eq!(world.num_entities(), 2);

        Ok(())
    }

    struct NodeDrawingComponent(i32);
}
