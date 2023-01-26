# latex-rs

## notes

this parser takes a specific latex file and returns an AST.

## todo
- [x] environments
    - [x] problem with nesting (issues with push and peek?)
- [ ] hashmap to store ast
    - [ ] have a clear description of what the data type of each node should be (env, expr, cmd, etc.)
    - [ ] environment -> name=name
    - [ ] command -> name=name, value=opts, children=args
    - [ ] literal -> value
- [x] proper indentation system

## done

- [x] command statements (e.g. `\emph[opt1, opt2]{arg}`)
- [x] comments (currently silenced)