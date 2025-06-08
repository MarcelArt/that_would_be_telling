use crate::error::Error;
pub use crud_derive::{ICreate, IRead, IUpdate, IDelete, IGetById};

pub trait ICreate<T, TDto> {
    async fn create(&self, input: TDto) -> Result<Option<T>, Error>;
}

pub trait IRead<T> {
    async fn read(&self) -> Result<Vec<T>, Error>;
}

pub trait IUpdate<T, TDto> {
    async fn update(&self, id: String, input: TDto) -> Result<Option<T>, Error>;
}
pub trait IDelete<T> {
    async fn delete(&self, id: String) -> Result<Option<T>, Error>;
}
pub trait IGetById<T> {
    async fn get_by_id(&self, id: String) -> Result<Option<T>, Error>;
}