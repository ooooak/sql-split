# cargo flamegraph --root -- ./resources/test_files/Sample-SQL-File-500000-Rows.sql --output 1mb

cargo build --release

cp ./target/release/sql-split-cli ./resources/output

cd ./resources/output
time ./sql-split-cli big.sql --output 400kb
cd -
