import ".just/commit.just"

default: clippy

ci_opt := if env("PRE_COMMIT_HOME", "") != "" { "-ci" } else { "" }

precommit:
    just pc{{ci_opt}}

pc:     fmt code-quality
pc-fix: fmt code-quality-fix
pc-ci:  pc-fix

prepush: check clippy docs

prepush-rs:
    #!/usr/bin/env -S bash -euo pipefail
    just check-core
    just check-cli
    just clippy-core
    just clippy-cli
    just docs-core
    just docs-cli
    

fmt:     code-quality-fix

full:    pc prepush build test
full-ci: pc-ci prepush        

# usage:
#   just e                -> open Justfile normally
#   just e foo            -> search for "foo" and open Justfile at that line
#   just e @bar           -> search for "^bar" (recipe name) and open Justfile at that line
#
e target="":
    #!/usr/bin/env -S echo-comment --color bold-red
    if [[ "{{target}}" == "" ]]; then
      $EDITOR Justfile
    else
      pat="{{target}}"
      if [[ "$pat" == @* ]]; then
        pat="^${pat:1}"   # strip @ and prefix with ^
      fi
      line=$(rg -n "$pat" Justfile | head -n1 | cut -d: -f1)
      if [[ -n "$line" ]]; then
        $EDITOR +$line Justfile
      else
        # No match for: $pat
        exit 1
      fi
    fi

lint-action:
    actionlint .github/workflows/CI.yml

# -------------------------------------

build:
    cargo build --workspace

check:
    cargo check --workspace

check-core:
    cargo check -p polite

check-cli:
    cargo check -p polite-cli

# -------------------------------------

clippy: clippy-all

clippy-all:
    cargo clippy --workspace --all-targets --all-features --target-dir target/clippy-all-features -- -D warnings

clippy-core:
    cargo clippy -p polite -- -D warnings

clippy-cli:
    cargo clippy -p polite-cli -- -D warnings

# -------------------------------------

test *args:
    just test-core {{args}}
    just test-cli {{args}}
    just test-js {{args}}

test-ci *args:
    #!/usr/bin/env -S echo-comment --color bright-green
    # 🏃 Running Rust tests...
    cargo test {{args}}
    
    # 📚 Running documentation tests...
    cargo test --doc {{args}}

[working-directory: 'polite']
test-core *args:
    cargo nextest run {{args}}
    
[working-directory: 'polite-cli']
test-cli *args:
    cargo nextest run {{args}}
    
# -------------------------------------

# Test CLI with example input
run-cli input="'{\"name\": \"test\", \"value\": 42}'":
    echo '{{input}}' | cargo run -p polite-cli

# Run CLI with file
run-cli-on *args:
    cargo run -p polite-cli -- {{args}}

# ------------------------------------

install-hooks:
   pre-commit install

run-pc:
   pre-commit run --all-files

# -------------------------------------

fix-eof-ws mode="":
    #!/usr/bin/env sh
    ARGS=''
    if [ "{{mode}}" = "check" ]; then
        ARGS="--check-only"
    fi
    whitespace-format --add-new-line-marker-at-end-of-file \
          --new-line-marker=linux \
          --normalize-new-line-markers \
          --exclude ".git/|target/|dist/|\.so$|.json$|.lock$|.parquet$|.venv/|.stubs/|\..*cache/" \
          $ARGS \
          .

code-quality:
    # just ty-ci
    taplo lint
    taplo format --check
    just fix-eof-ws check
    cargo machete
    cargo fmt --check --all

code-quality-fix:
    taplo lint
    taplo format
    just fix-eof-ws
    cargo machete
    cargo fmt --all

# -------------------------------------

docs:
    cargo doc --workspace --all-features --no-deps --document-private-items --keep-going

docs-core:
    cargo doc -p polite --all-features --no-deps --document-private-items --keep-going

docs-cli:
    cargo doc -p polite-cli --all-features --no-deps --document-private-items --keep-going

# -------------------------------------

clean:
    cargo clean

# -------------------------------------

# Example: JSON schema inference
example-basic:
    just test-cli '{"name": "Alice", "age": 30}'

example-array:
    just test-cli '[{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25, "city": "NYC"}]'

example-complex:
    echo '{"users": [{"name": "Alice", "profile": {"age": 30, "active": true}}, {"name": "Bob", "profile": {"age": 25, "premium": false}}]}' | just run-cli

# --------------------------------------------------------------------------------------------------

# Rust release workflow using release-plz
ship:
    #!/usr/bin/env -S echo-comment --shell-flags="-euo pipefail" --color="\\033[38;5;202m"

    # 🔍 Refuse to run if not on master branch or not up to date with origin/master
    branch="$(git rev-parse --abbrev-ref HEAD)"
    if [[ "$branch" != "master" ]]; then
        # ❌ Refusing to run: not on 'master' branch (current: $branch)
        exit 1
    fi
    # 🔍 Fetch master branch
    git fetch origin master
    local_rev="$(git rev-parse HEAD)"
    remote_rev="$(git rev-parse origin/master)"
    # 🔍 Local: $local_rev\n🔍 Remote: $remote_rev
    if [[ "$local_rev" != "$remote_rev" ]]; then
        # ❌ Refusing to run: local master branch is not up to date with origin/master
        # Local HEAD:  $local_rev
        # Origin HEAD: $remote_rev
        # Please pull/rebase to update.
        exit 1
    fi

    # 🔍 Dry-run release...
    just publish --dry-run
    # ✅ Dry-run went OK, proceeding to real release
    
    # 🦀 Update Cargo.toml versions and changelogs
    release-plz update
    git add .
    # Run a pre-precommit lint pass to avoid the linter halting our release!
    just precommit || true
    git commit -m "chore(release): 🦀 Upgrades"
    # Note: if already pushed you would just need to revert the additions (delete changelogs)

    # 🦀 Run prepush only for the Rust crates we are releasing
    just prepush-rs
    # 🚀 Push the version bump commit
    git push --no-verify

    # 📦 Create releases and tags
    just publish

publish mode="":
    #!/usr/bin/env -S bash -euo pipefail
    git_token=$(gh auth token 2>/dev/null) || git_token=$PUBLISH_GITHUB_TOKEN

    ## 🦀 Let release-plz handle workspace crate tagging
    ## It will create tags like: polite-v0.2.1, polite-cli-v0.1.5, etc.
    release-plz release --backend github --git-token $git_token {{mode}}
