default:
  just --list

start_digikam:
    digikam --database-directory ./etc/digikam-files/database --config ./etc/digikam-files/digikam.config

generate_digikam_database_test_data:
    rm -f tests/fixtures/db.sql    
    sqlite3 ./etc/digikam-files/database/digikam4.db .dump >> tests/fixtures/db.sql

generate_test_fixtures: generate_digikam_database_test_data

test:
    cargo test

fmt:
    cargo fmt --all

check:
    cargo clippy --fix --bin "digikam-wallpaper" -p rust-digikam-orm

