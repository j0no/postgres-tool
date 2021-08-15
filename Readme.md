# Postgres Tool
A postgres on and off switch for PostgreSQL on GTK+3.

## Requirements
- [Postgres without admin permission](https://saveriomiroddi.github.io/Quickly-setting-up-postgresql-for-running-without-admin-permissions/)
    Note: This setup is meant for development desktop environment. Do not use in production.
    
    

## MacOS Users

If you are running MacOS, I highly recommend [PostgresApp](https://github.com/PostgresApp/PostgresApp) instead as it comes with  more features.

## Development

### Need

- Node 12+
- [Rust](https://www.rust-lang.org/tools/install)

### Setup

1. Install dependencies

```bash
yarn
```

2. Run Dev

``` bash
yarn tauri dev
```

## Build 

```bash
yarn tauri build
```

## Stack

- [Tauri](https://tauri.studio/) - Rust + Web

## Works on
- Ubuntu 18.04, 20.04