# webthesis

this project takes a specific latex file and returns an AST.

from this AST, it generates a web rendering of the latex file.

## notes

the parsing of `\item` is a bit tricky (it does not require `{}`, but takes the rest of the line as its body). the workaround is to adapt the writing of the `.tex` input to only use `\item{}`.

## doing

- [ ] preprocessor for includes

## todo
- [ ] postprocessor
  - [ ] create table of contents
  - [ ] create list of figures
  - [ ] create bibliography
- [ ] list all needed elements
  - [ ] semantic commands
    - [ ] href (url + display text) __currently not used in thesis__
    - [ ] textquote __currently not used in thesis__
    - [ ] item __tricky one: the rest of the line should be itemized__

## done
- [x] make options their own node (currently they're not parsed)
- [x] have the `tag` field of the Node be a vector of traits, with the common trait being `value()`
- [x] write simple automated test
- [x] basic cli args parsing
- [x] environments
  - [x] minted (needs to have options handled better)
  - [x] problem with nesting (issues with push and peek?)
  - [x] listing
  - [x] itemize
  - [x] quote
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