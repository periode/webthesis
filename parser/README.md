# webthesis

this project takes a specific latex file and returns an AST.

from this AST, it generates a web rendering of the latex file.

## notes

`Environment::Figure` refers to images, while `Environment::Listing` refers to code snippets (`Environment::Equation` is subsumed into `Environment::Listing`).

the parsing of `\item` is a bit tricky (it does not require `{}`, but takes the rest of the line as its body). the workaround is to adapt the writing of the `.tex` input to only use `\item{}`.

I removed the `\begin{center}` environment from some of the figures in the source latex. if that messes up the layou on pdf, i should modify the parsing process to account for the possible nestedness of environments (roughly `figures.rs:129`)

__WARNING__ there should always be a new line at the end of the document, otherwise `\end{document}` does not get picked up

## doing

- [x] **PRIORITY** change the grammar to not wrap single commands and single environments inside paragraphs

## todo

- [ ] postprocessor
  - [ ] when parsing toc, ignore comments following on the same line
- [ ] use `erased_serde` or `typetag` crate to have only one `save_ast` function
- [ ] semantic commands
  - [ ] href (url + display text) __currently not used in thesis__
  - [ ] textquote __currently not used in thesis__

## done

- [x] create table of contents
- [x] issue that only the first children are being picked up
- [x] have a "spacer" command
- [x] store the include value (i.e. the current chapter) for each label (added external state)
- [x] item __tricky one: the rest of the line should be itemized__
- [x] create bibliography
- [x] give a value to the `include`
- [x] get rid of empty paragraph nodes
- [x] block on line `593` in `ideals.tex` -> solved by adding new line before \end
- [x] preprocessor for includes (text and graphics)
- [x] update grammar to allow for nested `{}` blocks
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
  - [x] figure
  - [x] center
  - [x] enumerate
  - [x] mathematics
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
  - [x] title
  - [x] author
  - [x] affil
  - [x] date
  - [x] today
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
  - [x] mathematics-specific