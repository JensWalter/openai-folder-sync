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

# Help

```
CLI tool for syncing files with the OpenAI vector store.

Usage: openai-folder-sync [OPTIONS] --openai-api-key <OPENAI_API_KEY> --vector-store <VECTOR_STORE> --local-dir <LOCAL_DIR>

Options:
  -o, --openai-api-key <OPENAI_API_KEY>
          [env: OPENAI_API_KEY=]
  -v, --vector-store <VECTOR_STORE>
          [env: VECTOR_STORE=]
  -l, --local-dir <LOCAL_DIR>
          [env: LOCAL_DIR=]
  -e, --extensions <EXTENSIONS>
          comma separated list of file extensions to sync [env: EXTENSIONS=]
  -g, --git-info <GIT_INFO>
          embed git info from git cli into the file content [env: GIT_INFO=] [possible values: true, false]
  -h, --help
          Print help
  -V, --version
          Print version
```