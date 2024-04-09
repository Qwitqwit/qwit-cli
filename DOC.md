# Command-Line Help for `qwit`

This document contains the help content for the `qwit` command-line program.

**Command Overview:**

* [`qwit`↴](#qwit)
* [`qwit markdown`↴](#qwit-markdown)
* [`qwit show`↴](#qwit-show)
* [`qwit validation`↴](#qwit-validation)
* [`qwit excel`↴](#qwit-excel)

## `qwit`

qwit cli

**Usage:** `qwit [OPTIONS] [COMMAND]`

###### **Subcommands:**

* `markdown` — [STABLE] print markdown doc of qwit to std out
* `show` — [STABLE] show the dsv from the start in a nice way
* `validation` — [STABLE] validate a dsv file against a dsv schema
* `excel` — [PREVIEW] convert a excel file to a csv/dsv file

###### **Options:**

* `-s`, `--sep <SEP>`

  Default value: `;`
* `-w`, `--working-dir <WORKING_DIR>`



## `qwit markdown`

[STABLE] print markdown doc of qwit to std out

**Usage:** `qwit markdown`



## `qwit show`

[STABLE] show the dsv from the start in a nice way

**Usage:** `qwit show [OPTIONS] --source <SOURCE>`

###### **Options:**

* `-s`, `--source <SOURCE>`
* `-n`, `--num <NUM>`

  Default value: `100`



## `qwit validation`

[STABLE] validate a dsv file against a dsv schema

**Usage:** `qwit validation --schema <SCHEMA> --source <SOURCE>`

###### **Options:**

* `-s`, `--schema <SCHEMA>`
* `-s`, `--source <SOURCE>`



## `qwit excel`

[PREVIEW] convert a excel file to a csv/dsv file

**Usage:** `qwit excel --source <SOURCE> --target <TARGET>`

###### **Options:**

* `-s`, `--source <SOURCE>`
* `-t`, `--target <TARGET>`



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

