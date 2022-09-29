use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::custom_errors::*;

pub type ElementId = usize;
pub type ComponentHash = HashMap<TypeId, Box<dyn Any>>;

pub struct World {
    element_components: HashMap<ElementId, ComponentHash>,
    new_element_id: ElementId,
}

impl World {
    pub fn new() -> Self {
        World {
            element_components: HashMap::new(),
            new_element_id: 0,
        }
    }

    ///
    /// Creates an element in the current world.
    /// ```
    /// use enco::world::*;
    /// struct MyComponent(i32);
    ///
    /// let mut world = World::new();
    /// let element_id = world
    ///          .create_element()
    ///          .with(MyComponent(12))
    ///          .done();
    /// ```
    pub fn create_element(&mut self) -> &mut Self {
        self.element_components
            .insert(self.new_element_id, HashMap::new());
        self
    }

    pub fn with<T: Any>(&mut self, component: T) -> &mut Self {
        self.element_components
            .get_mut(&self.new_element_id)
            .unwrap()
            .insert(TypeId::of::<T>(), Box::<T>::new(component));
        self
    }

    pub fn done(&mut self) -> ElementId {
        self.new_element_id += 1;
        self.new_element_id - 1
    }

    pub fn num_elements(&self) -> usize {
        self.element_components.len()
    }

    pub fn num_components(&self, element_id: &ElementId) -> Result<usize, WorldError> {
        if let Some(components) = self.element_components.get(&element_id) {
            return Ok(components.len());
        }
        Err(WorldError::ElementDoesNotExist)
    }

    ///
    /// Deletes an element from the world
    ///
    /// ```
    /// use enco::world::*;
    ///
    /// let mut world = World::new();
    /// let element_id = world
    ///          .create_element()
    ///          .done();
    /// world.delete_element(&element_id).unwrap();
    /// ```
    ///
    pub fn delete_element(&mut self, element_id: &ElementId) -> Result<(), WorldError> {
        if let Some(_) = self.element_components.remove(&element_id) {
            return Ok(());
        }
        Err(WorldError::DeleteElement)
    }

    /// Adds a component to an element
    ///
    /// ```
    /// use enco::world::*;
    /// struct SomeComponent(i32);
    ///
    /// let mut world = World::new();
    /// let element_id = world
    ///          .create_element()
    ///          .done();
    /// world.add_component(&element_id, SomeComponent(1)).unwrap();
    /// ```
    ///
    pub fn add_component<T: Any>(
        &mut self,
        element_id: &ElementId,
        component: T,
    ) -> Result<(), WorldError> {
        let type_id = TypeId::of::<T>();
        if let Some(components_hash) = self.element_components.get_mut(element_id) {
            if components_hash.contains_key(&type_id) {
                return Err(WorldError::ElementAlreadyHasComponent(type_id));
            }
            components_hash.insert(type_id, Box::new(component));
            return Ok(());
        }
        Err(WorldError::ElementAlreadyHasComponent(type_id))
    }

    ///
    /// Deletes a component from an element
    ///
    /// ```
    /// use enco::world::*;
    /// struct SomeComponent(i32);
    /// struct SomeOtherComponent(i32);
    ///
    /// let mut world = World::new();
    /// let element_id = world
    ///          .create_element()
    ///          .with(SomeComponent(12))
    ///          .with(SomeOtherComponent(12))
    ///          .done();
    /// world.delete_component::<SomeComponent>(&element_id).unwrap();
    /// ```
    ///
    pub fn delete_component<T: Any>(&mut self, element_id: &ElementId) -> Result<(), WorldError> {
        if let Some(element_components) = self.element_components.get_mut(element_id) {
            if let Some(_) = element_components.remove(&TypeId::of::<T>()) {
                return Ok(());
            }
            return Err(WorldError::ElementDoesNotHaveComponent(TypeId::of::<T>()));
        }
        Err(WorldError::DeleteElement)
    }

    pub fn get_element_component<T: Any>(&self, element_id: &ElementId) -> Option<&T> {
        let components = self.element_components.get(&element_id)?;
        components.get(&TypeId::of::<T>())?.downcast_ref::<T>()
    }

    pub fn get_element_component_mut<T: Any>(&mut self, element_id: &ElementId) -> Option<&mut T> {
        let components = self.element_components.get_mut(&element_id)?;
        components.get_mut(&TypeId::of::<T>())?.downcast_mut::<T>()
    }

    // Iterators

    ///
    /// Returns an iterator that goes through every element id
    /// ```
    /// use enco::world::*;
    ///
    /// let mut world = World::new();
    ///
    /// world.create_element().done();
    /// world.create_element().done();
    /// for element_id in world.iter() {
    ///     // Do something here
    /// }
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &usize> {
        self.element_components.iter().map(|entry| entry.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn create_element_in_world() {
        let mut world = World::new();
        let element_id = world.create_element().with(NodeDrawingComponent(1)).done();
        assert_eq!(element_id, 0);

        let element_id_2 = world.create_element().with(NodeDrawingComponent(2)).done();
        assert_eq!(element_id_2, 1);

        let c1_box =
            world.element_components[&element_id][&TypeId::of::<NodeDrawingComponent>()].as_ref();
        let c1 = c1_box.downcast_ref::<NodeDrawingComponent>().unwrap();
        assert_eq!(c1.0, 1);

        let c2_box =
            world.element_components[&element_id_2][&TypeId::of::<NodeDrawingComponent>()].as_ref();
        let c2 = c2_box.downcast_ref::<NodeDrawingComponent>().unwrap();
        assert_eq!(c2.0, 2);
    }

    #[test]
    pub fn delete_element() -> Result<(), WorldError> {
        let mut world = World::new();

        assert_eq!(world.element_components.len(), 0);

        let element_id = world.create_element().with(NodeDrawingComponent(1)).done();

        world.delete_element(&element_id)?;
        assert_eq!(world.element_components.len(), 0);
        Ok(())
    }

    #[test]
    pub fn world_capacity() -> Result<(), WorldError> {
        let mut world = World::new();

        assert_eq!(world.num_elements(), 0);

        world.create_element().with(NodeDrawingComponent(1)).done();

        assert_eq!(world.num_elements(), 1);

        let element_id_2 = world.create_element().with(NodeDrawingComponent(2)).done();

        assert_eq!(world.num_elements(), 2);

        world.delete_element(&element_id_2)?;
        assert_eq!(world.num_elements(), 1);

        world.create_element().with(NodeDrawingComponent(3)).done();

        assert_eq!(world.num_elements(), 2);

        Ok(())
    }

    struct NodeDrawingComponent(i32);
}
