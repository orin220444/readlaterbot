use rusqlite::Connection;
use serde::ser::Serialize;
use serde_rusqlite::*;
struct db;
impl db {
    fn connect() -> Result<Connection, rusqlite::Error> {

        let path = "./readlaterdb.db3";
        let conn = Connection::open(&path)?;
        Ok(conn)
    }
    pub fn insert_one<T>(&data:T, table: String, fields:String ,values: String)-> rusqlite::Result<()> 
where T: Serialize
    {
        let conn = Self::connect()?;
        conn.execute_named(format!("INSERT INTO {} ({}) VALUES {}]", table, fields, values),&to_params_named(data).unwrap())?

    }
}
