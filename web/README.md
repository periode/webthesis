# webthesis

front-end for the thesis.

## notes

would it be better to have it as a single page, or as distinct includes?

## doing

## todo

- [ ] refactor paragraph component (https://www.twilio.com/blog/dynamically-render-svelte-components)
- [ ] handle `\today` with Date in SSR (save a timestamp in rust)
- [ ] handle deep nestedness of quotes
- [ ] handle deep nestedness of footnotes
- [ ] listing
  - [ ] save whitespace during tex parsing
  - [ ] show lines
  - [ ] show highglight for language (library)
- [ ] specific components
  - [x] paragraph
  - [ ] image
  - [ ] figure
  - [x] footnote
    - [x] this where i figure out where to put modular elements
    - [ ] make it so that clicking anywhere on the rest of the screen makes the footnote disappear
  - [ ] quote
- [ ] deal with escaped characters
  - [ ] `\{` -> `{`
  - [ ] `\\` -> `\n`
  - [ ] etc.

## done
- [x] properly concatenate the literals inside a paragraph
- [x] create basic node component to recursively display all information
- [x] include tailwind
- [x] emph
- [x] citep
- [x] section
- [x] subsection
- [x] subsubsection
- [x] inline_listing
- [x] reference
- [**x**] error node


## svelte

```bash
npm run dev

# or start the server and open the app in a new browser tab
npm run dev -- --open

npm run build

npm run preview
```

to deploy your app, need to install an [adapter](https://kit.svelte.dev/docs/adapters) for the target environment.
