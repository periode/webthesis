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
        - [ ] lstlisting (should all be converted to listing + minted)
        - [ ] quotation / quote (should harmonize in the thesis itself)
    - [ ] commands
    - [x] chapter
    - [x] section
    - [x] subsection
    - [x] subsubsection
    - [ ] citep
    - [x] emph
    - [ ] caption
    - [ ] lstinline
    - [ ] mint (inline listing, bit of a weird syntax)
    - [ ] textquote
    - [ ] linespread (only to make some listings fit in one page)
    - [ ] footnote
    - [ ] vspace (WARNING: there's a `\` in the argument list)
    - [ ] centerline (can probs ignore the arguments inside for now? the point is that it's an `<hr>`)
    - [ ] dots
    - [ ] pagebreak (just for print layout, could be ignored?)
    - [ ] label (that's the one that is used as a hyperlink)
    - [ ] ref (counterpart of the above ^)
    - [ ] hyperref (not sure what is the difference with `ref`)
    - [ ] url

## done

- [x] environments
    - [x] problem with nesting (issues with push and peek?)
- [x] command statements (e.g. `\emph[opt1, opt2]{arg}`)
- [x] comments (currently silenced)
- [x] proper indentation system
- [x] serialization