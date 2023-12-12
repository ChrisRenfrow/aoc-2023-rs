#! /usr/bin/env just --justfile
set shell := ["fish", "-c"]
set dotenv-load

default_day_no := `datediff 2023-11-30 (dateadd today 3h)`
default_day_str := `printf "%02d" ` + default_day_no
template_day_format := "{{DAY}}"

get_input: (get_day_input default_day_no)

get_day_input day:
    aoc_input {{day}} > input/(printf "d%02d" {{day}})

test_day day_no:
    cargo test -p (printf "d%02d" {{day_no}})

test_all:
    cargo test --workspace

run:
    cargo run

new: get_input (new_day default_day_no)

new_day day:
    cargo init --lib solutions/d(printf "%02d" {{day}})
    sed "s/{{template_day_format}}/$(printf "%02d" {{day}})/g" day_template.rs.txt | sed -e '1,3d' > solutions/d(printf "%02d" {{day}})/lib.rs
