# Roadmap

### Settings Window

The current version of this project work with PosgreSQL 11 as set in `psql.rs`. This means that the app looks for the pg_ctl tool in pg 11 directory. A settings window will help make the project work with other versions by allows the use to change path to pg_ctl.

- **Simple guess the PostgreSQL Version:** This mostly to get the default PostgreSQL version and other installed versions. Use Rust Api.
- **Settings Page**: Use tauri web to display the current paths (`pg_ctl`, `pgsql_data`, `and PGSQL_VERSION`) in forms. When form/dropdown gets changed, path variables get changed.

