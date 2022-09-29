#[cfg(test)]
mod tests {

    use enco::custom_errors::*;
    use enco::world::*;

    #[test]
    pub fn get_element_component() {
        let mut world = World::new();

        let element_id_1 = world.create_element().with(NodeDrawingComponent(1)).done();

        let element_id_2 = world.create_element().with(NodeDrawingComponent(2)).done();

        let c1 = world
            .get_element_component::<NodeDrawingComponent>(&element_id_1)
            .unwrap();
        assert_eq!(c1.0, 1);

        let c2 = world
            .get_element_component::<NodeDrawingComponent>(&element_id_2)
            .unwrap();
        assert_eq!(c2.0, 2);
    }

    #[test]
    pub fn get_element_component_mut() {
        let mut world = World::new();

        let element_id_1 = world.create_element().with(NodeDrawingComponent(1)).done();

        let mut c1 = world
            .get_element_component_mut::<NodeDrawingComponent>(&element_id_1)
            .unwrap();
        c1.0 = 10;

        assert_eq!(c1.0, 10);
    }

    #[test]
    pub fn get_num_components() -> Result<(), WorldError> {
        let mut world = World::new();

        let element_id = world.create_element().done();

        assert_eq!(world.num_components(&element_id)?, 0);

        let element_id = world.create_element().with(NodeDrawingComponent(1)).done();

        assert_eq!(world.num_components(&element_id)?, 1);

        let element_id = world
            .create_element()
            .with(NodeDrawingComponent(1))
            .with(ConnectorDrawingComponent(1))
            .done();

        assert_eq!(world.num_components(&element_id)?, 2);

        world.create_element();

        Ok(())
    }

    #[test]
    pub fn remove_component() -> Result<(), WorldError> {
        let mut world = World::new();

        let element_id = world
            .create_element()
            .with(NodeDrawingComponent(1))
            .with(ConnectorDrawingComponent(1))
            .done();

        assert_eq!(world.num_components(&element_id)?, 2);

        world.delete_component::<NodeDrawingComponent>(&element_id)?;

        assert_eq!(world.num_components(&element_id)?, 1);

        Ok(())
    }

    #[test]
    pub fn iterate_through_elements() -> Result<(), WorldError> {
        let mut world = World::new();

        world.create_element().done();
        world.create_element().done();
        world.create_element().done();
        world.create_element().done();

        let mut count = 0;

        for _ in world.iter() {
            count += 1
        }

        assert_eq!(count, 4);

        Ok(())
    }

    #[test]
    pub fn add_component_to_element() -> Result<(), WorldError> {
        let mut world = World::new();

        let element_id = world.create_element().done();
        world.add_component(&element_id, NodeDrawingComponent(1))?;

        assert!(world
            .get_element_component::<NodeDrawingComponent>(&element_id)
            .is_some());
        assert!(world
            .get_element_component::<ConnectorDrawingComponent>(&element_id)
            .is_none());

        world.add_component(&element_id, ConnectorDrawingComponent(1))?;

        assert!(world
            .get_element_component::<NodeDrawingComponent>(&element_id)
            .is_some());
        assert!(world
            .get_element_component::<ConnectorDrawingComponent>(&element_id)
            .is_some());

        Ok(())
    }

    struct NodeDrawingComponent(i32);
    struct ConnectorDrawingComponent(i32);
}
