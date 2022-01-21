use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use chrono::prelude::Local;
use kstring::KString;
use liquid::model::{Array, Value};
use liquid::Object;
use tauri::Window;

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
    fn render(context: &mut Object, template: TemplateParam, root: String, output: String) -> Result<String> {
        let liquid = liquid::ParserBuilder::with_stdlib().build()?;

        let path = match output.is_empty() {
            true => Path::new(&root).to_path_buf(),
            false => {
                Path::new(&root).join(JavaRender::check_path_str(output))
            }
        };
        let mut temp_path = path.join(JavaRender::check_path_str(template.file_path));
        JavaRender::check_path(&temp_path)?;

        let file_name = liquid.parse(&template.file_name)?.render(context)?;
        temp_path = temp_path.join(file_name);

        let temp_str = liquid.parse(&template.content)?.render(context)?;
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
    pub fn render(&mut self, win: Window) -> Result<()> {
        let date = Local::now().format("%Y-%m-%d %H:%M").to_string();
        for table in self.table.iter_mut() {
            let mut context = liquid::Object::new();
            context.insert(KString::from("date"), Value::scalar(date.clone()));
            context.insert(KString::from("table_name"), Value::scalar(table.org_name.as_ref().unwrap().clone()));
            context.insert(KString::from("remark"), Value::scalar(table.comment.as_ref().unwrap().clone()));
            context.insert(KString::from("entity_name"), Value::scalar(table.name.as_ref().unwrap().clone()));
            for tp in self.templates.iter_mut() {
                if tp.lang == "java" {
                    TemplateRender::column_type(&mut context, tp.lang.clone(), table.column.clone());
                    match JavaRender::render(&mut context, tp.clone(), self.root_path.clone(), self.output.clone()) {
                        Ok(path) => win.emit("console", format!("{} 生成成功！", path)).unwrap(),
                        Err(err) =>
                            win.emit("console_error", format!("{} 生成失败！\n {}", tp.file_name, err)).unwrap()
                    };
                } else if tp.lang == "text" {
                    match TextRender::render(&mut context, tp.clone(), self.root_path.clone(), self.output.clone()) {
                        Ok(path) => win.emit("console", format!("{} 生成成功！", path)).unwrap(),
                        Err(err) => win.emit("console_error", format!("{} 生成失败！\n {}", tp.file_name, err)).unwrap()
                    };
                }
            }
        }
        win.emit("console", "over").unwrap();
        Ok(())
    }

    fn column_type(context: &mut Object, lang: String, column: Option<Vec<Column>>) {
        if lang == "java" {
            let mapping = JavaRender::type_mapper().unwrap();
            let mut columns = column.unwrap();
            for col in columns.as_mut_slice() {
                match mapping.get(&*col.field_type.as_ref().unwrap()) {
                    Some(val) => col.field_type = Some(val.to_string()),
                    None => {}
                }
                if col.key.as_ref().unwrap() == "PRI" {
                    context.insert(KString::from("primary_key"), Value::scalar(col.field_name.as_ref().unwrap().clone()));
                    context.insert(KString::from("pri_type"), Value::scalar(col.field_type.as_ref().unwrap().clone()));
                }
            }
            let mut arr: Array = Array::new();
            let mut index = 0;
            for col in columns.iter() {
                arr.insert(index, Value::from(liquid::object!(
                    { "name": col.name.as_ref().unwrap().clone(), "field_name": col.field_name.as_ref().unwrap().clone(),
                        "data_type": col.field_type.as_ref().unwrap().clone(), "key": col.key.as_ref().unwrap().clone(),
                        "comment": col.comment.as_ref().unwrap().clone()})));
                index = index + 1;
            }

            context.insert(KString::from("fields"), Value::from(arr));
        }
    }
}