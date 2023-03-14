# webthesis

this project takes a specific latex file and returns an AST.

from this AST, it generates a web rendering of the latex file.

## notes

`Environment::Figure` refers to images, while `Environment::Listing` refers to code snippets (`Environment::Equation` is subsumed into `Environment::Listing`).

the parsing of `\item` is a bit tricky (it does not require `{}`, but takes the rest of the line as its body). the workaround is to adapt the writing of the `.tex` input to only use `\item{}`.

I removed the `\begin{center}` environment from some of the figures in the source latex. if that messes up the layou on pdf, i should modify the parsing process to account for the possible nestedness of environments (roughly `figures.rs:129`)

__WARNING__ there should always be a new line at the end of the document, otherwise `\end{document}` does not get picked up

## doing

## todo

- [ ] number paragraphs
- [ ] chore: include multiple possible paragraphs in quote environment
- [ ] chore: strip includes in the parse pass? the issue is that returning a `Vec<ToCNode>` from the parse_cmd implies we need to get the label then, which is tricky
- [ ] semantic commands
  - [ ] href (url + display text) __currently not used in thesis__
  - [ ] textquote __currently not used in thesis__
