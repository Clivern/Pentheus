<p align="center">
    <img src="/static/logo.png" width="300" />
    <h3 align="center">Pentheus</h3>
    <p align="center">Your Database Guardian, Set up in Minutes.</p>
    <p align="center">
        <a href="https://github.com/Clivern/Pentheus/actions"><img src="https://github.com/Clivern/Pentheus/actions/workflows/build.yml/badge.svg"></a>
        <a href="https://github.com/Clivern/Pentheus/releases"><img src="https://img.shields.io/badge/Version-v0.1.0-green.svg"></a>
        <a href="https://github.com/Clivern/Pentheus/blob/main/LICENSE"><img src="https://img.shields.io/badge/LICENSE-MIT-green.svg"></a>
    </p>
</p>

Pentheus — your go-to command-line buddy for keeping your databases safe and sound! Built with Rust, this nifty tool makes backing up and storing your SQLite, MySQL, and PostgreSQL databases super easy. Let’s make sure your data is always protected!

## Features

- Multi-Database Support: Back up your SQLite, MySQL, and PostgreSQL databases without breaking a sweat.
- Storage Options Galore: Choose to stash your backups locally or in the cloud (Amazon S3) — your choice!
- Compression Magic: Want to save some space? Go ahead and compress those backups!
- Automated Backups: Set up cron jobs to handle backups automatically. Just set it and forget it!
- Backup Tracking: Keep tabs on your backups with a handy state file—no more guessing!

## Usage

First, you’ll want to install `pentheus` using `Cargo`. Open your terminal and run:

```zsh
$ cargo install pentheus
```

Next, create a `TOML` config file to tell `pentheus` how to back up your databases. Here’s a quick example to get you rolling:

```toml
# Database backups
[database]
  [database.sqlite_db_01]
  driver = "sqlite"
  path = "/opt/backup/app1-db.sqlite"

  [database.my_mysql_db_01]
  driver = "mysql"
  host = "mysql_host"
  port = 3306
  user = "mysql_user"
  password = "mysql_password"
  database = "my_mysql_db"

  [database.my_postgres_db_01]
  driver = "postgres"
  host = "postgres_host"
  port = 5432
  user = "postgres"
  password = "postgres_password"
  database = "my_postgres_db"

# Backup storage
[storage]
  [storage.local_store]
  driver = "local"
  path = "/path/to/local/backup"
  compress = "zip"

  [storage.s3_store]
  driver = "s3"
  bucket = "my-backup-bucket"
  region = "us-east-1"
  access_key = "my_access_key"
  secret_key = "my_secret_key"
  compress = "none"

# Cron Jobs
[cron]
  [cron.sqlite_db_01_cron]
  schedule = "5 4 * * *"
  database = "sqlite_db_01"
  storage = "local_store"

# Backups state file
[state]
storage = "s3_store"
path = "/state.json"
```


## Versioning

For transparency into our release cycle and in striving to maintain backward compatibility, Pentheus is maintained under the [Semantic Versioning guidelines](https://semver.org/) and release process is predictable and business-friendly.

See the [Releases section of our GitHub project](https://github.com/clivern/pentheus/releases) for changelogs for each release version of Pentheus. It contains summaries of the most noteworthy changes made in each release.


## Bug tracker

If you have any suggestions, bug reports, or annoyances please report them to our issue tracker at https://github.com/clivern/pentheus/issues


## Security Issues

If you discover a security vulnerability within Pentheus, please send an email to [hello@clivern.com](mailto:hello@clivern.com)


## Contributing

We are an open source, community-driven project so please feel free to join us. see the [contributing guidelines](CONTRIBUTING.md) for more details.


## License

© 2025, clivern. Released under [MIT License](https://opensource.org/licenses/mit-license.php).

**Pentheus** is authored and maintained by [@Clivern](http://github.com/clivern).
