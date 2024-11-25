# OPENAI-FOLDER-SYNC

CLI tool for synchronizing a local directory with the OpenAI Vector Store.

# Install

We currently only support install through cargo

```
cargo install --git https://github.com/JensWalter/openai-folder-sync.git
```

# Usage

**Installed Binary**

```
openai-folder-sync --vector-store 'vs_ABCDEFGHIJK' --local-dir '/Users/jens/tmp/wiki/content' --extensions md
```

**Via Cargo**
```
cargo run -- --vector-store 'vs_ABCDEFGHIJK' --local-dir '/Users/jens/tmp/wiki/content' --extensions md
```