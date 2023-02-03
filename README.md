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
    - [ ] href (url + display text) __currently not used in thesis__
    - [ ] textquote __currently not used in thesis__

## done

- [x] environments
    - [x] problem with nesting (issues with push and peek?)
- [x] command statements (e.g. `\emph[opt1, opt2]{arg}`)
- [x] comments (currently silenced)
- [x] proper indentation system
- [x] serialization
- [x]  layout commands (to be ignored in web, the idea would be to change the return type of `parse_cmd_stmt` to `Option<Node>` and return `None` if the command is a layout command)
- [x]  commands
  - [x] chapter
  - [x] section
  - [x] subsection
  - [x] subsubsection
  - [x] emph
  - [x] caption
  - [x] lstinline
  - [x] textit
  - [x] footnote
  - [x] dots
  - [x] label
  - [x] ref
  - [x] url (pure url)
  - [x] citep
  - [x] linespread
  - [x] vspace
  - [x] centerline
  - [x] pagebreak
  - [x] rule
  - [x] linewidth
  - [x] baselineskip