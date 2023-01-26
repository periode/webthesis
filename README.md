# latex-rs

## notes

this parser takes a specific latex file and returns an AST.

## todo
- [] environments
    - [] problem with nesting (issues with push and peek?)
- [] hashmap to store ast
- [] proper indentation system

## done

- [x] command statements (e.g. `\emph[opt1, opt2]{arg}`)
- [x] comments (currently silenced)