# pg_gensql

A utility for generating INSERT statements from existing PostgreSQL systems.

## Usage

```
pg_gensql <DB_URL> <SQL>
```

For example,
```
pg_gensql "host=localhost password=example user=postgres port=5433" "SELECT * FROM test_scalar_not_null"
```

## Motivation
`pg_gensql` was written as an exercise for the author to learn Rust, among
a few other things. Therefore, I welcome feedback on how to better use
Rust idioms. In particular, I'm wondering whether there is a better
way to implement `serialize` for `Raw`, or whether `Raw` could implement
`TryInto` for `FromSql` without conflicting definition errors.

## Building & Testing
`pg_gensql` is written in Rust and uses `cargo` for packaging. In a local
clone, you can run `cargo run ...` to build and run it locally.

### Testing
We use `docker-compose` to ensure that Postgres is setup in a consistent way. We
need to add a lot more tests to make sure this actually works.

1. Run `docker-compose up -d`
2. Run `cargo test`
3. Run `docker-compose down`
