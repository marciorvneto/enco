use std::any::TypeId;

#[derive(Debug)]
pub enum WorldError {
    ElementDoesNotExist,
    ElementDoesNotHaveComponent(TypeId),
    ElementAlreadyHasComponent(TypeId),
    CreateElement,
    DeleteElement,
}
