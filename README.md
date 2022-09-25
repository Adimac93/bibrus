# Bibrus
Bibrus approach no. 2

## Commit structure

```dif
? - optional
* - zero or more
Name:
<category><project>?<!/?>?(breaking change/untested): <present tense>
Description:
<+/-/~>* (add/remove/comment)
```

Use backticks (\`text\`) when referring to code (e.g.`trait`, `counter`, `String` )

Breaking changes have to be tested!

### Categories
  - fix - fixing unwanted behaviour
  - perf - performance, optimization
  - doc - document, documentation
  - feat - feature
  - refactor - changing structure but not functionality
  - format - only text, visual changes
  - init - only for initialization purposes

## Environment

Currently we are using [Doppler](https://www.doppler.com) to keep our secrets and app configuration in sync and secure across devices.

Command line interface guide [here](https://docs.doppler.com/docs/cli)
## Database

### Prerequsities 

You'll need to download some tooling first.


#### Details about postgresql installation: 
  - [mac](https://github.com/diesel-rs/diesel/blob/master/guide_drafts/backend_installation.md#mac-osx)
  - [windows](https://github.com/diesel-rs/diesel/blob/master/guide_drafts/backend_installation.md#windows)

#### Details about libpq installation:
  - Mac [brew](https://brew.sh): `brew install libpq postgresql`
  - [windows](https://duredhelfinceleb.github.io/diesel_windows_install_doc/)
  - [windows error](https://luckystreak.ca/writing/fixing_diesel_cli_rust_build_error.html)
### Connection

To use databse locally you'll need to specify `DATABASE_URL` environnmental variable in `.env` file.
![Connection string schema](https://res.cloudinary.com/prismaio/image/upload/v1628761154/docs/m7l8KVo.png)
```
# Example .env file
DATABASE_URL="postgresql://postgres@localhost:5432/bibrus"
```

#### Bit.io
Online database is hosted on [bit.io](https://bit.io/) and checking in it's great if you would like to check out information like configuration, ussage, data and more.

#### Doppler
If you are connecting remotely you only need to use `DATABASE_URL` specified in [Doppler](https://www.doppler.com) with environment of your choice (e.g. Development, Staging, Production; see differences [here](https://dev.to/flippedcoding/difference-between-development-stage-and-production-d0p?signin=true)).

### CLI

To use [ORM](https://en.wikipedia.org/wiki/Object–relational_mapping) (object relational mapping) that's featured with Diesel you'll need to setup [diesel-cli](https://github.com/diesel-rs/diesel/tree/master/diesel_cli#diesel-cli).

If everything mentioned before is set up correctly: `cargo install diesel_cli --no-default-features --features "postgres"` would not output any errors.

### Useful tools
  - Administration tools:
    - Recommended: All in one, **postgres included**: [PgAdmin](https://www.pgadmin.org/download/)
    - JetBrains [DataGrip](https://www.jetbrains.com/datagrip/)
    - PostgreSQL interactive terminal [psql](https://www.postgresql.org/download/)

  - Database servers:
    -  Raw [PostgreSQL](https://www.postgresql.org/download/) server
    -  Only mac [app](https://postgresapp.com)

### Resources
  - Axum 
    - [docs.rs](https://docs.rs/axum/latest/axum/)
    - [examples](https://github.com/tokio-rs/axum/tree/main/examples)
  - Tokio
    - [docs.rs](https://docs.rs/tokio/latest/tokio/)
    - [tokio.rs](https://tokio.rs)
  - Diesel
    - [docs.rs](https://docs.rs/diesel/latest/diesel/)
    - [guides](https://diesel.rs/guides/)

## Files 

### .keep
`.keep` files are sometimes used by developers who use Git. Git only tracks files, which means it cannot track empty directories. By sticking a `.keep` file into the (no-longer) empty directory, the directory has some kind of representation in the Git repository.
The name `.keep` is just a convention. It has no meaning to Git itself.

### .env
Text configuration file for controlling your **local** Applications environment constants. 


## Acceptable langs
 - Rust
 - TypeScript

