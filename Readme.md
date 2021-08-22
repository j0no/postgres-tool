# Postgres Tool
A postgres on and off switch for PostgreSQL on GTK+3.

![screenshot](https://raw.githubusercontent.com/j0no/postgres-tool/main/res/postgres-tool-screenshot.png)

## Requirements
- [Postgres without admin permission](https://saveriomiroddi.github.io/Quickly-setting-up-postgresql-for-running-without-admin-permissions/)
    Note: This setup is meant for development desktop environment. Do not use in production.
    
    

## MacOS Users

If you are running MacOS, I highly recommend [PostgresApp](https://github.com/PostgresApp/PostgresApp) instead as it comes with  more features.

## Development

Read the [Roadmap](Roadmap.md) if you want to contribute.

### Need

- Node 12+
- [Rust](https://www.rust-lang.org/tools/install)

### Setup

1. Install dependencies

```bash
yarn # or 'npm install'
```

2. Run Dev

``` bash
yarn tauri dev # or 'npm run tauri dev'
```

## Build 

```bash
yarn tauri build # or 'npm run tauri build'
```

## Stack

- [Tauri](https://tauri.studio/) - Rust + Web

## Works on
- Ubuntu 18.04, 20.04