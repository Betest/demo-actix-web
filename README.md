[![Build Status](https://dev.azure.com/yohantobon/actix-web/_apis/build/status%2FBetest.demo-actix-web?branchName=main)](https://dev.azure.com/yohantobon/actix-web/_build/latest?definitionId=2&branchName=main)

[![Board Status](https://dev.azure.com/yohantobon/43ba1271-bbe8-4b28-8b6f-d4e718c40874/b547150f-c053-4d8b-b202-8e31c65d8d7a/_apis/work/boardbadge/ca9d3e5a-bf8a-405d-9991-359ba55ca18a)](https://dev.azure.com/yohantobon/43ba1271-bbe8-4b28-8b6f-d4e718c40874/_boards/board/t/b547150f-c053-4d8b-b202-8e31c65d8d7a/Microsoft.RequirementCategory/)

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