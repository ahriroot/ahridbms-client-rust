use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    let mut plugin = Builder::new("mysql");
    plugin = plugin.invoke_handler(tauri::generate_handler![]);
    plugin.build()
}

#[cfg(test)]
mod tests {
    use mysql_async::prelude::Queryable;

    #[tokio::test]
    async fn test1() {
        let database_url = "mysql://root:Aa12345.@localhost:3306/mysql";
        let pool = mysql_async::Pool::new(database_url);
        let mut conn = pool.get_conn().await.unwrap();
        let mut res = conn.query_iter("SELECT * FROM user").await.unwrap();
        while let Ok(Some(row)) = res.next().await {
            let columns = row.columns_ref();
            for column in columns {
                let name = column.name_str();
                let mut error_count = 0;
                match name {
                    std::borrow::Cow::Borrowed(n) => {
                        let typ = column.column_type();
                        match typ {
                            mysql_async::consts::ColumnType::MYSQL_TYPE_SHORT => {
                                // let value = row.get::<i16, &str>(n).unwrap();
                                // println!("1{}: {}", n, value);
                            }
                            mysql_async::consts::ColumnType::MYSQL_TYPE_STRING => {
                                let value = row.get_opt::<String, _>(n).unwrap();
                                match value {
                                    Ok(v) => {
                                        println!("{}: {}", n, v);
                                    }
                                    Err(_) => {
                                        let value2 = row.get_opt::<i16, _>(n).unwrap();
                                        match value2 {
                                            Ok(v) => {
                                                println!("{}: {}", n, v);
                                            }
                                            Err(_) => {
                                                error_count += 1;
                                            }
                                        }
                                    }
                                }
                            }
                            mysql_async::consts::ColumnType::MYSQL_TYPE_LONG => {
                                let value = row.get::<i32, &str>(n).unwrap();
                                println!("1{}: {}", n, value);
                            }
                            _ => {}
                        }
                    }
                    std::borrow::Cow::Owned(_) => {}
                }
                println!("error_count: {}", error_count);
            }
            break;
            // println!("data: {:?}", data);
            // for i in row {
            //     println!("{}", i.into());
            //     match i {
            //         mysql_async::Value::Bytes(bytes) => {
            //             let s = String::from_utf8(bytes).unwrap();
            //             println!("{}", s);
            //         }
            //         _ => {}
            //     }
            // }
        }
        // assert!(result.stream::<Row>().await.unwrap_err().to_string().contains("FOO"));
        // let result = from_row(res);
        assert_eq!(1, 1);
    }
}
