[![Board Status](https://dev.azure.com/yohantobon/b1a6b405-cd00-42bc-8b5a-a6cdc0e3f0f5/1a400bba-40f1-4e2c-a87c-595142748733/_apis/work/boardbadge/263b0569-c981-4542-9698-db1949bdf9c1)](https://dev.azure.com/yohantobon/b1a6b405-cd00-42bc-8b5a-a6cdc0e3f0f5/_boards/board/t/1a400bba-40f1-4e2c-a87c-595142748733/Microsoft.RequirementCategory)

[![Build Status](https://dev.azure.com/yohantobon/actix-web/_apis/build/status%2Factix-web?branchName=azure-boards)](https://dev.azure.com/yohantobon/actix-web/_build/latest?definitionId=1&branchName=azure-boards)

Libs:
cargo-edit

- build proyect
cargo build --release


- Docker
docker-compose logs -f


-Add diesel (migraciones y control de version de db)
cargo add diesel chrono --features "diesel/postgres diesel/r2d2"
cargo install diesel_cli --no-default-features --features postgres
diesel setup
diesel migration generate create_posts