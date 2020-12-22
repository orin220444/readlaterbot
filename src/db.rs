use rusqlite::Connection;
use serde::{ser::Serialize, de::DeserializeOwned};
use rusqlite::NO_PARAMS;
use serde_rusqlite::*;
pub struct db;
impl db {
    fn connect() -> std::result::Result<Connection, rusqlite::Error> {

        let path = "./readlaterdb.db3";
        let conn = Connection::open(&path)?;
        Ok(conn)
    }
    pub fn insert_one<T>(data:&T, table: String, fields:String ,values: String)-> rusqlite::Result<()> 
where T: Serialize
    {
        let conn = Self::connect()?;
        conn.execute_named(&format!("INSERT INTO {} ({}) VALUES {}]", table, fields, values),&to_params_named(data).unwrap().to_slice())?;
Ok(())
    }
    pub fn get_all<T>(table:String, _:&T) -> rusqlite::Result<Vec<T>>
    where T: DeserializeOwned
    {
        let conn = Self::connect()?;
        let mut statement = conn.prepare(&format!("SELECT * FROM {}", table))?;
        let mut rows = from_rows::<T>(statement.query(NO_PARAMS).unwrap());
        let row = rows
            .map(|data| data.unwrap()).collect();//                        .collect::<Vec<T>>?;

        Ok(row)
    }
    pub fn get_specific<T>(table: String, _:&T, condition: String) 
        -> rusqlite::Result<Vec<T>>
        where T: DeserializeOwned
    {
        let conn = Self::connect()?;
            let mut statement = conn.prepare(&format!("SELECT * FROM {} WHERE {}", table, condition))?;
            let mut rows = from_rows::<T>(statement.query(NO_PARAMS).unwrap());
        let row = rows.map(|data| data.unwrap()).collect();
Ok(row)
    }
    pub fn delete (table: String, condition:String) -> rusqlite::Result<()>{
        let conn = Self::connect()?;
        conn.execute(&format!("DELETE FROM {} WHERE {}", table, condition), NO_PARAMS)?;
            Ok(())
    }
    pub fn update (table: String, set: String, condition:String) -> rusqlite::Result<()>{
        let conn = Self::connect()?;
        conn.execute(&format!("UPDATE {} SET {} WHERE {}", table, set, condition), NO_PARAMS)?;
            Ok(())
    }
}
