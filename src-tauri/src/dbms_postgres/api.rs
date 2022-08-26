use postgres::{types::Type, Client, NoTls};

use crate::dbms_postgres::entity::*;

pub async fn get_table_struct(table: String) {
    
}

#[tauri::command]
pub async fn select(skip: i32, limit: i32, page: i32, size: i32, table: String) -> Vec<Vec<Field>> {
    let mut skip_count = skip;
    let mut limit_count = limit;
    if page > 0 && size > 0 {
        skip_count = (page - 1) * size;
        limit_count = size;
    }
    let mut client = Client::connect("host=localhost user=postgres", NoTls).unwrap();
    let rows = client
        .query(
            "SELECT * FROM $1 LIMIT $2 OFFSET $3",
            &[&table, &limit_count, &skip_count],
        )
        .unwrap();
    let mut result_data: Vec<Vec<Field>> = Vec::new();
    for row in rows.iter() {
        let mut result_row: Vec<Field> = Vec::new();
        let columns = row.columns();
        for column in columns {
            let name = column.name();
            let typ = column.type_();

            match typ {
                &Type::BOOL => {
                    let data: bool = row.get(name);
                    result_row.push(Field::Bool(KV {
                        key: name.to_string(),
                        value: data,
                    }));
                }
                _ => {
                    let data: String = row.get(name);
                    result_row.push(Field::String(KV {
                        key: name.to_string(),
                        value: data,
                    }));
                }
            };
        }
        result_data.push(result_row);
    }
    result_data
}
