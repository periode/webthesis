# latex-rs

## notes

this parser takes a specific latex file and returns an AST.

## todo

- [ ] preprocessor for includes
- [ ] postprocessor
    - [ ] create table of contents
    - [ ] create table list
    - [ ] create bibliography
- [ ] list all needed elements
- [ ] deal with item/itemize
- [ ] hashmap to store ast
    - [ ] have a clear description of what the data type of each node should be (env, expr, cmd, etc.)
    - [ ] environment -> name=name
    - [ ] command -> name=name, value=opts, children=args (this is the one that needs to be made into an additional struct, it needs at least three fields)
    - [ ] literal -> value


## done

- [x] environments
    - [x] problem with nesting (issues with push and peek?)
- [x] command statements (e.g. `\emph[opt1, opt2]{arg}`)
- [x] comments (currently silenced)
- [x] proper indentation system