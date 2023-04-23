cargo build --release

cp target/release/sql-split resources/db/sql-split

./sql-split ./database.sql --output 100mb

