# Turso Experiment

How far can we get writing a TODO application that works both on the command line and the internet, backed by Turso?

## Running the experiment

### Native

```sh
$ cargo run -p cli -- --help
Usage: todo-list [OPTIONS]

Options:
  -p, --db-path <DB_PATH>
          Path to the database

          [default: $HOME/.local/share/todo-list/db.sqlite]

  -l, --log [<LEVEL>]
          Enable logging

          If this flag is set without an explicit level argument, defaults to "info".

          [possible values: trace, debug, info, warn, error]

  -h, --help
          Print help (see a summary with '-h')
```

### WASM

#### Setup

- `rustup target add wasm32-unknown-unknown`
- install `wasm-bindgen-cli`

#### Build

```sh
wasm-pack build ffi
```


## Notes and Findings

1. Migrations are a pain and we may have to write our own migration framework
    - `refinery` has no native support for Turso
    - and bringing in `rusqlite` only for purpose of migrations is silly, and will likely fail once we encrypt the database
    - so we introduce a connection wrapper, whatever
    - but the traits `refinery` uses are only sync, and `turso` is async-first
    - and we can't just `smol::block_on` because `block_on` is illegal in a wasm context
2. Getting a rowset is annoyingly manual
    - [`Rows`](https://docs.rs/turso/0.4.4/turso/struct.Rows.html) has an annoyingly manual interface: just `async fn next`, no iterator or map combinators
    - Can't get a value by column name, only by column index
    - `query_row` returns no error if more than one row is returned
3. Encryption is a planned feature, but it's experimental for now at least.
4. Builder feels underbaked: path argument is `&str` not `impl AsRef<Path>`.
5. We still occasionally encounter TODO items in the library
    - `Parse error: TODO: parenthesized expression with multiple arguments not yet supported` from `RETURNING (id, created_at)`.
        - Though that might have been an sql error anyway
6. Ultimately in the native context it appears to work, at least for simple cases.
    I would not yet call it ready for prime time in this context due to its various rough edges, but there's clear and substantial progress since last we looked at it.
7. Some features don't fail but also don't work:
    - `ON DELETE CASCADE` doesn't cascade.
8. Database encryption is underbaked
    - For example, [`EncryptionOpts`](https://docs.rs/turso/latest/turso/type.EncryptionOpts.html) fundamentally configure encryption:
        - `cipher` is a magic string where it should be an enum
        - `hexkey` needs to be `[u8; 32]` or equivalent but in fact is a runtime-checked hex-encoded string
        - nothing is documented
    - Key rotation is not implemented.
    - Key derivation is not implemented.
    - As this is all currently experimental, there have been no security audits; high chance of some surprising implementation problems showing up.
    - Overall this feature is not production-ready.