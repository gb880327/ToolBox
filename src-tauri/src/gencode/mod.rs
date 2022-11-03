use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use chrono::prelude::Local;
use tera::{Context, Tera};
use java::JavaRender;
use text::TextRender;

use crate::model::{Column, Table};

mod java;
mod text;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TemplateParam {
    pub file_path: String,
    pub file_name: String,
    pub content: String,
    pub lang: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TemplateRender {
    pub table: Vec<Table>,
    pub root_path: String,
    pub output: String,
    pub templates: Vec<TemplateParam>,
}

pub trait RenderTemplate {
    fn render(context: &mut Context, template: TemplateParam, root: String, output: String) -> Result<String> {
        let path = match output.is_empty() {
            true => Path::new(&root).to_path_buf(),
            false => {
                Path::new(&root).join(Self::check_path_str(output))
            }
        };
        let mut temp_path = path.join(Self::check_path_str(template.file_path));
        Self::check_path(&temp_path)?;

        let file_name = Tera::one_off(&template.file_name, &context, true)?;
        temp_path = temp_path.join(file_name);

        let temp_str = Tera::one_off(&template.content, &context, true)?;
        let mut fs = OpenOptions::new().create(true).write(true).open(temp_path.clone())?;
        fs.write_all(temp_str.as_bytes())?;
        Ok(temp_path.to_str().unwrap().to_string())
    }

    fn type_mapper() -> Option<HashMap<String, String>> {
        None
    }

    fn check_path_str(mut path: String) -> String {
        if path.is_empty() {
            return path;
        }
        if path.starts_with("/") || path.starts_with("\\") {
            path.remove(0);
        }
        path
    }

    fn check_path(path: &PathBuf) -> Result<()> {
        if !path.exists() {
            match std::fs::create_dir_all(path) {
                Ok(()) => Ok(()),
                Err(err) => Err(anyhow!(err.to_string()))
            }
        } else {
            Ok(())
        }
    }
}

impl TemplateRender {
    pub fn render(&mut self) -> Result<()> {
        let date = Local::now().format("%Y-%m-%d %H:%M").to_string();
        for table in self.table.iter_mut() {
            let mut context = Context::new();
            context.insert("date", &date);
            context.insert("table_name", &table.org_name.as_ref().unwrap());
            context.insert("remark", &table.comment.as_ref().unwrap());
            context.insert("entity_name", &table.name.as_ref().unwrap());
            for tp in self.templates.iter_mut() {
                if tp.lang == "java" {
                    TemplateRender::column_type(&mut context, tp.lang.clone(), table.column.clone());
                    match JavaRender::render(&mut context, tp.clone(), self.root_path.clone(), self.output.clone()) {
                        Ok(path) => super::SERVICE.lock().unwrap().console("console", format!("{} 生成成功！", path)),
                        Err(err) =>
                            super::SERVICE.lock().unwrap().console("console_error", format!("{} 生成失败！\n {}", tp.file_name, err))
                    };
                } else if tp.lang == "text" {
                    match TextRender::render(&mut context, tp.clone(), self.root_path.clone(), self.output.clone()) {
                        Ok(path) => super::SERVICE.lock().unwrap().console("console", format!("{} 生成成功！", path)),
                        Err(err) => super::SERVICE.lock().unwrap().console("console_error", format!("{} 生成失败！\n {}", tp.file_name, err))
                    };
                }
            }
        }
        super::SERVICE.lock().unwrap().console("console", "over".to_string());
        Ok(())
    }

    fn column_type(context: &mut Context, lang: String, column: Option<Vec<Column>>) {
        if lang == "java" {
            let mapping = JavaRender::type_mapper().unwrap();
            let mut columns = column.unwrap();
            for col in columns.as_mut_slice() {
                match mapping.get(&*col.field_type.as_ref().unwrap()) {
                    Some(val) => col.field_type = Some(val.to_string()),
                    None => {}
                }
                if col.key.as_ref().unwrap() == "PRI" {
                    context.insert("primary_key", &col.field_name.as_ref().unwrap());
                    context.insert("pri_type", &col.field_type.as_ref().unwrap());
                }
            }
            let mut arr = vec![];
            for col in columns.iter() {
                let mut data = HashMap::new();
                data.insert("name", col.name.as_ref().unwrap().clone()).unwrap_or_default();
                data.insert("field_name", col.field_name.as_ref().unwrap().clone()).unwrap_or_default();
                data.insert("data_type", col.field_type.as_ref().unwrap().clone()).unwrap_or_default();
                data.insert("key", col.key.as_ref().unwrap().clone()).unwrap_or_default();
                data.insert("comment", col.comment.as_ref().unwrap().clone()).unwrap_or_default();
                arr.push(data);
            }

            context.insert("fields", &arr);
        }
    }
}