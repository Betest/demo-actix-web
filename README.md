[![Build Status](https://dev.azure.com/yohantobon/actix-web/_apis/build/status%2FBetest.demo-actix-web?branchName=refs%2Fpull%2F2%2Fmerge)](https://dev.azure.com/yohantobon/actix-web/_build/latest?definitionId=2&branchName=refs%2Fpull%2F2%2Fmerge)

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