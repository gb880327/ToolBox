use anyhow::Result;
use async_trait::async_trait;
use rbatis::{DriverType, Error};
use rbatis::crud::{CRUD, CRUDTable};
use rbatis::executor::Executor;
use rbatis::plugin::page::{Page, PageRequest};
use rbatis::rbatis::Rbatis;
use rbatis::wrapper::Wrapper;
use serde::Deserialize;
use serde_json::Value;

use crate::model::{Column, Table};

#[sql(super::MYSQL, "select table_name as org_name, table_comment as comment from information_schema.tables where table_schema = ?")]
pub async fn table_list(db: &str) -> Vec<Table> { todo!() }

#[sql(super::MYSQL, "select column_name as field_name, data_type as data_type, column_key as `key`, column_comment as comment from information_schema.columns where table_schema = ? and table_name = ?")]
pub async fn table_column(db_name: &str, table: &str) -> Vec<Column> { todo!() }

pub async fn exec_sql(rb: &Rbatis, sql: &str, args: Option<Vec<Value>>) -> Result<u64, Error> {
    let result = match args {
        Some(arg) => rb.exec(sql, arg).await?,
        None => rb.exec(sql, vec![]).await?
    };
    Ok(result.rows_affected)
}

pub async fn save<T: CRUDTable + for<'de> Deserialize<'de>>(rb: &Rbatis, bean: &T) -> Option<u64> {
    match rb.save(bean, &[]).await {
        Ok(result) => {
            if result.rows_affected > 0 {
                Some(result.last_insert_id.unwrap() as u64)
            } else {
                Some(0)
            }
        }
        Err(_err) => Some(0)
    }
}

pub async fn update<T: CRUDTable + for<'de> Deserialize<'de>>(rb: &Rbatis, bean: &T, id: i64) -> Option<u64> {
    unwrap(rb.update_by_wrapper(bean, rb.new_wrapper().eq("id", id), &[]).await)
}

pub async fn save_or_update<T: CRUDTable + for<'de> Deserialize<'de>>(rb: &Rbatis, bean: &T, id: Option<i64>) -> Option<u64> {
    match id {
        Some(i) => update(rb, bean, i).await,
        None => save(rb, bean).await
    }
}

pub fn new_wrapper() -> Wrapper {
    Wrapper::new(&DriverType::Sqlite)
}

fn unwrap<T>(task: Result<T, Error>) -> Option<T> {
    match task {
        Ok(result) => Some(result),
        Err(_err) => None
    }
}

#[async_trait]
pub trait BaseModel<T: CRUDTable + for<'de> Deserialize<'de>> {
    fn meta(&mut self) -> T;

    async fn list(rb: &Rbatis, wr: Option<Wrapper>) -> Option<Vec<T>> {
        match wr {
            Some(wrapper) => unwrap(rb.fetch_list_by_wrapper(wrapper).await),
            None => unwrap(rb.fetch_list().await)
        }
    }

    async fn list_by_page(rb: &Rbatis, wr: Option<Wrapper>, req: PageRequest) -> Option<Page<T>> {
        match wr {
            Some(wrapper) => unwrap(rb.fetch_page_by_wrapper(wrapper, &req).await),
            None => unwrap(rb.fetch_page_by_wrapper(rb.new_wrapper(), &req).await)
        }
    }

    async fn one(rb: &Rbatis, wr: Wrapper) -> Option<T> {
        unwrap(rb.fetch_by_wrapper(wr).await)
    }

    async fn update(&mut self, rb: &Rbatis, wr: Option<Wrapper>) -> Option<u64> {
        let mut bean = self.meta();
        match wr {
            Some(wrapper) => unwrap(rb.update_by_wrapper(&mut bean, wrapper, &[]).await),
            None => unwrap(rb.update_by_wrapper(&mut bean, rb.new_wrapper(), &[]).await)
        }
    }

    async fn remove(rb: &Rbatis, wr: Option<Wrapper>) -> Option<u64> {
        match wr {
            Some(wrapper) => unwrap(rb.remove_by_wrapper::<T>(wrapper).await),
            None => unwrap(rb.remove_by_wrapper::<T>(rb.new_wrapper()).await)
        }
    }

    async fn save(&mut self, rb: &Rbatis) -> Option<u64> {
        let bean = self.meta();
        match rb.save(&bean, &[]).await {
            Ok(result) => {
                if result.rows_affected > 0 {
                    Some(result.last_insert_id.unwrap() as u64)
                } else {
                    Some(0)
                }
            }
            Err(_err) => Some(0)
        }
    }
}