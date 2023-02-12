# webthesis

front-end for the thesis.

## notes

would it be better to have it as a single page, or as distinct includes?

## doing

- [ ] listing
  - [ ] save whitespace during tex parsing
  - [ ] show lines
  - [ ] show highglight for language (library)

## todo

- [ ] refactor paragraph component (https://www.twilio.com/blog/dynamically-render-svelte-components)
- [ ] properly concatenate the literals inside a paragraph
- [ ] handle `\today` with Date in SSR
- [ ] handle deep nestedness of quotes
- [ ] specific components
  - [x] paragraph
  - [ ] image
  - [x] inline_listing
  - [x] reference
  - [ ] figure
  - [ ] footnote
    - [ ] this where i figure out where to put modular elements
  - [ ] quote
- [ ] deal with escaped characters
  - [ ] `\{` -> `{`
  - [ ] `\\` -> `\n`
  - [ ] etc.

## done

- [x] create basic node component to recursively display all information
- [x] include tailwind
- [x] emph
- [x] citep
- [x] section
- [x] subsection
- [x] subsubsection
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
