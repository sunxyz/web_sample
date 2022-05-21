use crate::models::*;
use async_trait::async_trait;
use rbatis::core::Result;
use rbatis::crud::CRUDTable;
use rbatis::crud::Skip;
use rbatis::crud::CRUD;
use rbatis::db::db_adapter::DBExecResult;
use rbatis::plugin::page::{IPage, IPageRequest, Page};
use rbatis::rbatis::Rbatis;
use rbatis::wrapper::Wrapper;
use rbson::Bson;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use rbatis::error::Error;

#[async_trait]
pub trait Repository {
    async fn save<T>(&self, table: &T) -> Result<DBExecResult>
    where
        T: CRUDTable;

    async fn save_all<T>(&self, tables: &[T]) -> Result<DBExecResult>
    where
        T: CRUDTable;

    async fn delete_by_id<T, P>(&self, id: P) -> Result<u64>
    where
        T: CRUDTable,
        P: Serialize + Send + Sync;

    async fn update<T>(&self, table: &T) -> Result<u64>
    where
        T: CRUDTable;

    async fn get_one<T, P>(&self, id: P) -> Result<T>
    where
        T: CRUDTable + DeserializeOwned,
        P: Serialize + Send + Sync;

    async fn list_all<T>(&self, w: Wrapper) -> Result<Vec<T>>
    where
        T: CRUDTable + DeserializeOwned;

    /// fetch page result(prepare sql)
    async fn find_all<T>(
        &self,
        sql: &str,
        args: Vec<rbson::Bson>,
        page_request: &dyn IPageRequest,
    ) -> Result<Page<T>>
    where
        T: DeserializeOwned + Serialize + Send + Sync;
}

pub struct DbRepository {
    exc: Rbatis,
}

impl DbRepository {
    pub fn new() -> Self {
        let exc = Rbatis::new();
        DbRepository { exc }
    }

    pub async fn link(&self, driver_url: &str) -> core::result::Result<(), Error>
    {
        self.exc.link(driver_url).await
    }
}

#[async_trait]
impl Repository for DbRepository {
    async fn save<T>(&self, table: &T) -> Result<DBExecResult>
    where
        T: CRUDTable,
    {
        self.exc.save(table, &[]).await
    }

    async fn save_all<T>(&self, tables: &[T]) -> Result<DBExecResult>
    where
        T: CRUDTable,
    {
        self.exc.save_batch(tables, &[]).await
    }

    async fn delete_by_id<T, P>(&self, id: P) -> Result<u64>
    where
        T: CRUDTable,
        P: Serialize + Send + Sync,
    {
        self.exc.remove_by_column::<T, _>("id", id).await
    }

    async fn update<T>(&self, table: &T) -> Result<u64>
    where
        T: CRUDTable,
    {
        self.exc.update_by_column(&"id", table).await
    }

    async fn get_one<T, P>(&self, id: P) -> Result<T>
    where
        T: CRUDTable + DeserializeOwned,
        P: Serialize + Send + Sync,
    {
        self.exc.fetch_by_column("id", id).await
    }

    async fn list_all<T>(&self, w: Wrapper) -> Result<Vec<T>>
    where
        T: CRUDTable + DeserializeOwned,
    {
        self.exc.fetch_list_by_wrapper(w).await
    }

    async fn find_all<T>(
        &self,
        sql: &str,
        args: Vec<rbson::Bson>,
        page_request: &dyn IPageRequest,
    ) -> Result<Page<T>>
    where
        T: DeserializeOwned + Serialize + Send + Sync,
    {
        self.exc.fetch_page(sql, args, page_request).await
    }
}
