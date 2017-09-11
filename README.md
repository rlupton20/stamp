# stamp

`stamp -c command` runs command for each line it receives on stdin, and prepends it to the line on stdout.

# Example usage

```
$ yes foo | stamp -c date
.
.
.
Mon Sep 11 19:03:02 BST 2017 foo
Mon Sep 11 19:03:02 BST 2017 foo
Mon Sep 11 19:03:03 BST 2017 foo
Mon Sep 11 19:03:03 BST 2017 foo
Mon Sep 11 19:03:03 BST 2017 foo
Mon Sep 11 19:03:03 BST 2017 foo
.
.
.
```

## Building from source

You will need the rust toolchain (in particular, cargo) installed. Build with

```
cargo build --release
```

The built binary can then be found in `target/release` and added to your path in a suitable place (e.g. `/usr/local/bin`).
