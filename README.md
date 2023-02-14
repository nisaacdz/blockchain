# BLOCKCHAIN   
This is a general purpose blockchain crate built for implementing under other projects
To use it on a custom struct, let your transaction struct implement Record which is a trait I created to help with hashing and other stuff.


## libraries used

### rust crates
bincode = "1.3.3"
rusqlite = { version = "0.28.0", features = ["bundled"] }
serde = { version="1.0.152", features = ["derive"] }
serde_json = "1.0.93"
sha2 = "0.10.6"

### database
5. SQLite
