#[cfg(test)]
mod tests {

    use enco::custom_errors::*;
    use enco::world::*;

    #[test]
    pub fn get_entity_component() {
        let mut world = World::new();

        let entity_id_1 = world.create_entity().with(NodeDrawingComponent(1)).done();

        let entity_id_2 = world.create_entity().with(NodeDrawingComponent(2)).done();

        let c1 = world
            .get_entity_component::<NodeDrawingComponent>(&entity_id_1)
            .unwrap();
        assert_eq!(c1.0, 1);

        let c2 = world
            .get_entity_component::<NodeDrawingComponent>(&entity_id_2)
            .unwrap();
        assert_eq!(c2.0, 2);
    }

    #[test]
    pub fn get_entity_component_mut() {
        let mut world = World::new();

        let entity_id_1 = world.create_entity().with(NodeDrawingComponent(1)).done();

        let mut c1 = world
            .get_entity_component_mut::<NodeDrawingComponent>(&entity_id_1)
            .unwrap();
        c1.0 = 10;

        assert_eq!(c1.0, 10);
    }

    #[test]
    pub fn get_num_components() -> Result<(), WorldError> {
        let mut world = World::new();

        let entity_id = world.create_entity().done();

        assert_eq!(world.num_components(&entity_id)?, 0);

        let entity_id = world.create_entity().with(NodeDrawingComponent(1)).done();

        assert_eq!(world.num_components(&entity_id)?, 1);

        let entity_id = world
            .create_entity()
            .with(NodeDrawingComponent(1))
            .with(ConnectorDrawingComponent(1))
            .done();

        assert_eq!(world.num_components(&entity_id)?, 2);

        world.create_entity();

        Ok(())
    }

    #[test]
    pub fn remove_component() -> Result<(), WorldError> {
        let mut world = World::new();

        let entity_id = world
            .create_entity()
            .with(NodeDrawingComponent(1))
            .with(ConnectorDrawingComponent(1))
            .done();

        assert_eq!(world.num_components(&entity_id)?, 2);

        world.delete_component::<NodeDrawingComponent>(&entity_id)?;

        assert_eq!(world.num_components(&entity_id)?, 1);

        Ok(())
    }

    #[test]
    pub fn iterate_through_entities() -> Result<(), WorldError> {
        let mut world = World::new();

        world.create_entity().done();
        world.create_entity().done();
        world.create_entity().done();
        world.create_entity().done();

        let mut count = 0;

        for _ in world.iter() {
            count += 1
        }

        assert_eq!(count, 4);

        Ok(())
    }

    #[test]
    pub fn filter_entities() -> Result<(), WorldError> {
        let mut world = World::new();

        world.create_entity().with(NodeDrawingComponent(1)).done();
        world.create_entity().with(NodeDrawingComponent(2)).done();
        world
            .create_entity()
            .with(ConnectorDrawingComponent(1))
            .done();
        world
            .create_entity()
            .with(ConnectorDrawingComponent(2))
            .done();
        world.create_entity().with(NodeDrawingComponent(3)).done();

        let query = world.query::<NodeDrawingComponent>();
        let mut sz_1 = 0;
        for _value in query {
            sz_1 += 1;
        }
        assert_eq!(sz_1, 3);

        let query = world.query::<ConnectorDrawingComponent>();
        let mut sz_2 = 0;
        for _value in query {
            sz_2 += 1;
        }
        assert_eq!(sz_2, 2);

        Ok(())
    }

    #[test]
    pub fn filter_entities_mut() -> Result<(), WorldError> {
        let mut world = World::new();

        world.create_entity().with(NodeDrawingComponent(1)).done();
        world.create_entity().with(NodeDrawingComponent(2)).done();
        world
            .create_entity()
            .with(ConnectorDrawingComponent(1))
            .done();
        world
            .create_entity()
            .with(ConnectorDrawingComponent(2))
            .done();
        world.create_entity().with(NodeDrawingComponent(3)).done();

        let query = world.query_mut::<NodeDrawingComponent>();
        let mut sz_1 = 0;
        for value in query {
            value.0 += 10;
            sz_1 += 1;
        }
        assert_eq!(sz_1, 3);

        let query = world.query_mut::<ConnectorDrawingComponent>();
        let mut sz_2 = 0;
        for value in query {
            value.0 += 10;
            sz_2 += 1;
        }
        assert_eq!(sz_2, 2);

        Ok(())
    }

    #[test]
    pub fn add_component_to_entity() -> Result<(), WorldError> {
        let mut world = World::new();

        let entity_id = world.create_entity().done();
        world.add_component(&entity_id, NodeDrawingComponent(1))?;

        assert!(world
            .get_entity_component::<NodeDrawingComponent>(&entity_id)
            .is_some());
        assert!(world
            .get_entity_component::<ConnectorDrawingComponent>(&entity_id)
            .is_none());

        world.add_component(&entity_id, ConnectorDrawingComponent(1))?;

        assert!(world
            .get_entity_component::<NodeDrawingComponent>(&entity_id)
            .is_some());
        assert!(world
            .get_entity_component::<ConnectorDrawingComponent>(&entity_id)
            .is_some());

        Ok(())
    }

    struct NodeDrawingComponent(i32);
    struct ConnectorDrawingComponent(i32);
}
