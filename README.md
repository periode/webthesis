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
    - [ ] environments
        - [ ] listing
        - [ ] itemize / item
        - [ ] minted
        - [ ] lstlisting (should all be converted to listing + minted)
        - [ ] quotation / quote (should harmonize in the thesis itself)
    - [ ] commands
    - [ ] chapter
    - [ ] section
    - [ ] subsection
    - [ ] subsubsection
    - [ ] citep
    - [ ] emph
    - [ ] caption
    - [ ] lstinline
    - [ ] mint (inline listing, bit of a weird syntax)
    - [ ] textquote
    - [ ] linespread (only to make some listings fit in one page)
    - [ ] same as above
    - [ ] footnote
    - [ ] vspace (WARNING: there's a `\` in the argument list)
    - [ ] centerline (can probs ignore the arguments inside for now? the point is that it's an `<hr>`)
    - [ ] dots
    - [ ] pagebreak (just for print layout, could be ignored?)
    - [ ] label (that's the one that is used as a hyperlink)
    - [ ] ref (counterpart of the above ^)
    - [ ] hyperref (not sure what is the difference with `ref`)
    - [ ] url
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