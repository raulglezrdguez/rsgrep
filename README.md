# Project rsgrep

Project develop with rust to mimic grep command line tool.

<pre>
Usage: rsgrep [query] [file_path]
</pre>

## run project

<pre>
cargo run -- test test.txt
</pre>

## run with env variable

<pre>
IGNORE_CASE=true cargo run -- test test.txt
</pre>

## run test

<pre>
cargo test
</pre>

## run test & show output

<pre>
cargo test -- --show-output
</pre>
