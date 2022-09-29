use std::any::TypeId;

#[derive(Debug)]
pub enum WorldError {
    EntityDoesNotExist,
    EntityDoesNotHaveComponent(TypeId),
    EntityAlreadyHasComponent(TypeId),
    CreateEntity,
    DeleteEntity,
}
