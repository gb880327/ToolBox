interface Project {
    id: number,
    name: string,
    path: string
}

interface TemplateItem {
    template_id: number,
    file_path: string,
    file_name: string
}

interface Profile {
    id: number,
    project_id: number,
    profile: string,
    need_upload: number,
    remote_dir: string,
    target_name: string,
    before: string,
    after: string
}

interface Server {
    id: number,
    name: string,
    label: string,
    host: string,
    port: string,
    user: string,
    password: string,
    private_key: string,
    auth_type: string,
    command: string
}

interface DataSource {
    id: number,
    name: string,
    db_type: string,
    host: string,
    port: string,
    db_name: string,
    prefix: string,
    user: string,
    password: string
}

interface Category {
    id: number,
    parent_id: string,
    name: string
}

interface GenProject {
    id: number,
    project_id: string,
    datasource: string,
    output: string,
    template: string,
    type_mapping: string
}

interface Template {
    id: number,
    category_id: string,
    language: string,
    name: string,
    content: string
}

interface Env {
    id: number,
    name: string,
    value: string
}

interface TableInfo {
    name: string,
    org_name: string,
    comment: string,
    column: Array<Column>
}

interface Column {
    name: string,
    field_name: string,
    data_type: string,
    field_type: string,
    key: string,
    comment: string
}

interface MsgNode {
    msg: string,
    type: number
}