cargo build --release

cp ./target/release/sql-split resources/db/sql-split

cd resources/db;

./sql-split ./database.sql --output 100mb

