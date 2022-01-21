use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use anyhow::Result;
use kstring::KString;
use liquid::model::Value;
use liquid::Object;
use super::TemplateParam;
use super::RenderTemplate;
pub struct JavaRender();

fn replace_package(path: String) -> String {
    if path.is_empty() {
        return path;
    }
    path.replace("/", ".").replace("\\", ".")
}

impl RenderTemplate for JavaRender {
    fn render(context: &mut Object, template: TemplateParam, root: String, output: String) -> Result<String> {
        let liquid = liquid::ParserBuilder::with_stdlib().build()?;

        let path = match output.is_empty() {
            true => Path::new(&root).to_path_buf(),
            false => {
                let out_path = JavaRender::check_path_str(output);
                let base_package = replace_package(out_path.clone());
                context.insert(KString::from("base_package"), Value::scalar(base_package));
                Path::new(&root).join(out_path)
            }
        };
        let file_path = JavaRender::check_path_str(template.file_path);
        let package = replace_package(file_path.clone());
        context.insert(KString::from("package"), Value::scalar(package));
        let mut temp_path = path.join(file_path);
        JavaRender::check_path(&temp_path)?;

        let file_name = liquid.parse(&template.file_name)?.render(context)?;
        temp_path = temp_path.join(file_name);

        let temp_str = liquid.parse(&template.content)?.render(context)?;
        let mut fs = OpenOptions::new().create(true).write(true).open(temp_path.clone())?;
        fs.write_all(temp_str.as_bytes())?;

        Ok(temp_path.to_str().unwrap().to_string())
    }

    fn type_mapper() -> Option<HashMap<String, String>> {
        let mut data = HashMap::new();
        data.insert("int".into(), "Integer".into());
        data.insert("bigint".into(), "Long".into());
        data.insert("smallint".into(), "Integer".into());
        data.insert("mediumint".into(), "Integer".into());
        data.insert("varchar".into(), "String".into());
        data.insert("char".into(), "String".into());
        data.insert("tinytext".into(), "String".into());
        data.insert("text".into(), "String".into());
        data.insert("mediumtext".into(), "String".into());
        data.insert("longtext".into(), "String".into());
        data.insert("datetime".into(), "Date".into());
        data.insert("date".into(), "Date".into());
        data.insert("time".into(), "Date".into());
        data.insert("timestamp".into(), "Long".into());
        data.insert("tinyint".into(), "Boolean".into());
        data.insert("decimal".into(), "Double".into());
        data.insert("float".into(), "Double".into());
        data.insert("double".into(), "Double".into());
        Some(data)
    }
}