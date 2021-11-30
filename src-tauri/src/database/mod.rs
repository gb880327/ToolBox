use async_trait::async_trait;
use rbatis::crud::{CRUD, CRUDTable};
use rbatis::Error;
use rbatis::executor::Executor;
use rbatis::plugin::page::{Page, PageRequest};
use rbatis::rbatis::Rbatis;
use rbatis::wrapper::Wrapper;
use serde::Deserialize;
use serde_json::Value;

pub async fn init(path: String) -> Result<Rbatis, Error> {
    let rb = Rbatis::new();
    rb.link(&path).await?;
    Ok(rb)
}

pub async fn exec_sql(rb: &Rbatis, sql: &str, args: Option<Vec<Value>>) -> Result<u64, Error> {
    let result = match args {
        Some(arg) => rb.exec(sql, arg).await?,
        None => rb.exec(sql, vec![]).await?
    };
    Ok(result.rows_affected)
}

#[async_trait]
pub trait BaseModel<T: CRUDTable + for<'de> Deserialize<'de>> {
    fn meta(&self) -> T;

    async fn list(rb: &Rbatis, wr: Option<Wrapper>) -> Result<Vec<T>, Error> {
        let rows: Vec<T> = match wr {
            Some(wrapper) => rb.fetch_list_by_wrapper(wrapper).await?,
            None => rb.fetch_list().await?
        };
        Ok(rows)
    }

    async fn list_by_page(rb: &Rbatis, wr: Option<Wrapper>, req: PageRequest) -> Result<Page<T>, Error> {
        let rows: Page<T> = match wr {
            Some(wrapper) => rb.fetch_page_by_wrapper(wrapper, &req).await?,
            None => rb.fetch_page_by_wrapper(rb.new_wrapper(), &req).await?
        };
        Ok(rows)
    }

    async fn one(rb: &Rbatis, wr: Wrapper) -> Result<T, Error> {
        let row: T = rb.fetch_by_wrapper(wr).await?;
        Ok(row)
    }

    async fn update(&self, rb: &Rbatis, wr: Option<Wrapper>) -> Result<u64, Error> {
        let mut bean = self.meta();
        let count = match wr {
            Some(wrapper) => rb.update_by_wrapper(&mut bean, wrapper, &[]).await?,
            None => rb.update_by_wrapper(&mut bean, rb.new_wrapper(), &[]).await?
        };
        Ok(count)
    }

    async fn remove(rb: &Rbatis, wr: Option<Wrapper>) -> Result<u64, Error> {
        let count = match wr {
            Some(wrapper) => rb.remove_by_wrapper::<T>(wrapper).await?,
            None => rb.remove_by_wrapper::<T>(rb.new_wrapper()).await?
        };
        Ok(count)
    }

    async fn save(&self, rb: &Rbatis) -> Result<u64, Error> {
        let bean = self.meta();
        let result = rb.save(&bean, &[]).await?;
        if result.rows_affected > 0 {
            Ok(result.last_insert_id.unwrap() as u64)
        } else {
            Ok(0)
        }
    }
}