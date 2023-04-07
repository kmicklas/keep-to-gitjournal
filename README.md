# Google Keep to GitJournal Markdown

This is a small Rust tool to convert Google Keep notes exported with Takeout to [GitJournal](https://gitjournal.io/) compatible Markdown files.

The maker of GitJournal also provides [google-keep-exporter](https://github.com/vHanda/google-keep-exporter), but I wasn't able to get it to work. (The precompiled binaries may still work but I din't try.)

## Usage

Export Keep data with Google Takeout and extract the archive somehwere.

```shell
keep-to-gitjournal Takeout/Keep -o path/to/gitjournal/repo
```

Then commit/push the repo.

## Missing Features

* Configuration of output format
* Keep formatting besides bullet lists (e.g. checkbox lists)
* Images
* Handling of newlines is very rudimentary

PRs or issues for missing features are welcome.
