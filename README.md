# ENCO (**EN**tities and **CO**mponents)

This is a very simple library for managing entities and components. This is not an entity component system (ECS) library.

ENCO was designed with simplicity in mind and should not be used in projects where performance is critical.

Having said that, it is very straightforward to use.

## Basic usage

Entities are referred to as Elements and live in a `World`.

Entities can be given components, which can be of any type.

```rs
use enco::world::*;

// Defining the components

struct PositionComponent {
  x: i32,
  y: i32,
}

struct TagComponent {
  tag: String
}

// Creating a world

let mut world = World::new();

// Creating entities (elements)

let element_id = world
         .create_element()
         .with(PositionComponent{x: 0, y: 0})
         .with(TagComponent{tag: "First element".to_string()})
         .done();

// Entity components can be accessed through the element id

let tag_component = world
    .get_element_component::<TagComponent>(&element_id)
    .unwrap();

// Entity components can be deleted

let tag_component = world
    .delete_element_component::<TagComponent>(&element_id)
    .unwrap();

// Entities themselves can be deleted

world.delete_element(&element_id).unwrap();
```
