extern crate rocksdb;

use rocksdb::{DB, Options};
// NB: db is automatically closed at end of lifetime

fn main() {
    let path = "D:\\rocksdb";
{
   let db = DB::open_default(path).unwrap();
       db.put(b"my key", b"my value").unwrap();
    match db.get(b"my key") {
        Ok(Some(value)) => println!("retrieved value {}", String::from_utf8(value).unwrap()),
        Ok(None) => println!("value not found"),
        Err(e) => println!("operational problem encountered: {}", e),
   }
   db.delete(b"my key").unwrap();
}
 let _ = DB::destroy(&Options::default(), path);
}
