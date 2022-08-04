use anyhow::Result;
use rbatis::Error;
use rbatis::rbatis::Rbatis;
use async_trait::async_trait;
use crate::database::DbUtil;
use crate::model::{Column, Table};

pub struct MySQL {
	rb: Rbatis,
}

impl MySQL {
	pub async fn default(url: &str) -> Result<MySQL> {
		let rb = Rbatis::new();
		rb.link(url).await?;
		Ok(MySQL { rb })
	}
}

#[async_trait]
impl DbUtil for MySQL {
	async fn table_list(&mut self, db: &str) -> Result<Vec<Table>, Error> {
		let sql = format!("select table_name as org_name, table_comment as comment from information_schema.tables where table_schema = '{}'", db);
		let rst: Vec<Table> = self.rb.fetch(&sql, vec![]).await?;
		Ok(rst)
	}

	async fn table_column(&mut self, db: &str, table: &str) -> Result<Vec<Column>, Error> {
		let sql = format!("select column_name as field_name, data_type as data_type, column_key as `key`, column_comment as comment from information_schema.columns where table_schema = '{}' and table_name = '{}'", db, table);
		let rst: Vec<Column> = self.rb.fetch(&sql, vec![]).await?;
		Ok(rst)
	}
}