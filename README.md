# webthesis

this project takes a specific latex file and returns an AST.

from this AST, it generates a web rendering of the latex file.

## notes

refer to tufte-css for a base layout

## doing

- [ ] hashmap to store ast
    - [ ] have different node types as `children` (trait implementation?)

## todo

- [ ] preprocessor for includes
- [ ] postprocessor
    - [ ] create table of contents
    - [ ] create table list
    - [ ] create bibliography
- [ ] list all needed elements
    - [ ] environments
        - [ ] listing
        - [ ] itemize / item
        - [ ] minted
        - [ ] quote
    - [ ] semantic commands
        - [x] chapter
        - [x] section
        - [x] subsection
        - [x] subsubsection
        - [x] emph
        - [x] caption
        - [x] lstinline
        - [ ] textquote (__has options__ + todo figure out proper layout for citations in tex file)
        - [x] footnote
        - [x] dots
        - [ ] label
        - [ ] ref (counterpart of the above ^)
        - [ ] hyperref (not sure what is the difference with `ref`? just for sections? I should harmonize)
        - [ ] url
        - [ ] citep
    - [ ]  layout commands (to be ignored in web, the idea would be to change the return type of `parse_cmd_stmt` to `Option<Node>` and return `None` if the command is a layout command)
        - [ ] linespread (only to make some listings fit in one page)
        - [ ] vspace (WARNING: there's a `\` in the argument list)
        - [ ] centerline (can probs ignore the arguments inside for now? the point is that it's an `<hr>`)
        - [ ] pagebreak (just for print layout, could be ignored?)

## done

- [x] environments
    - [x] problem with nesting (issues with push and peek?)
- [x] command statements (e.g. `\emph[opt1, opt2]{arg}`)
- [x] comments (currently silenced)
- [x] proper indentation system
- [x] serialization