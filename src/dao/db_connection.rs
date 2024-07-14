use std::{fs::File, path::Path};

use diesel::{Connection, SqliteConnection};

pub fn get_connection() -> SqliteConnection {
     let path = Path::new("banco.db");
     let display = path.display();

     match File::open(&path) {
         Err(why) => {
             match File::create("./banco.db") {
                 Err(why) => panic!("couldn't create {}: {}", display, why),
                 Ok(file) => file,
             };
             println!("{}", why);
             return SqliteConnection::establish("banco.db").unwrap_or_else(|_| panic!("Error connecting to banco.db"));
         }
         Ok(_file) => {
            return SqliteConnection::establish("banco.db").unwrap_or_else(|_| panic!("Error connecting to banco.db"));
        }
     };
}



