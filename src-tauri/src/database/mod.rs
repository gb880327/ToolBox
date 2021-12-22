use anyhow::Result;
use async_trait::async_trait;
use rbatis::crud::{CRUD, CRUDTable};
use rbatis::{DriverType, Error};
use rbatis::executor::Executor;
use rbatis::plugin::page::{Page, PageRequest};
use rbatis::rbatis::Rbatis;
use rbatis::wrapper::Wrapper;
use serde::Deserialize;
use serde_json::Value;

pub async fn exec_sql(rb: &Rbatis, sql: &str, args: Option<Vec<Value>>) -> Result<u64, Error> {
    let result = match args {
        Some(arg) => rb.exec(sql, arg).await?,
        None => rb.exec(sql, vec![]).await?
    };
    Ok(result.rows_affected)
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