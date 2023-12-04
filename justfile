#! /usr/bin/env just --justfile
set shell := ["fish", "-c"]
set dotenv-load

default_day_no := `datediff 2023-11-30 (dateadd today 3h)`
default_day_str := `printf "%02d" ` + default_day_no

get_input: (get_day_input {{default_day_no}})

get_day_input day:
    aoc_input {{day}} > input/(printf "d%02d" {{day}})
