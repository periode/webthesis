# webthesis

front-end for the thesis.

## notes

would it be better to have it as a single page, or as distinct includes?

## doing


## todo

- [ ] autodeploy on push?
- [ ] mobile styling
  - [ ] put footnotes on top
  - [ ] make them aligned left and smaller
- [ ] references
  - [ ] deal with finding the actual names for the chapters/sections/subsections, etc.
  - [ ] citation (parse bib.json to find the proper ones)

## done
- [x] render URL
- [x] assign icons for references
  - [x] listing
  - [x] figure
  - [x] footnote
    - [x] make it so that clicking anywhere on the rest of the screen makes the footnote disappear
- [x] list of items
- [x] paragraph
- [x] deal with escaped characters (based on pest grammar)
- [x] listing
  - [x] replace all inline code in tex source by minted input
  - [x] parse command minted input
  - [x] copy all `redaction/corpus/*` to web
  - [x] fill in the `snippet.ts`
  - [x] show highglight for language ([library](https://github.com/highlightjs/highlight.js))
- [x] image
  - [x] in makefile, copy all src images to target web asset dir
- [x] refactor paragraph component
- [x] handle deep nestedness of quotes
- [x] handle deep nestedness of footnotes
- [x] handle `\today` with Date in SSR (save a timestamp in rust)
- [x] properly concatenate the literals inside a paragraph
- [x] create basic node component to recursively display all information
- [x] include tailwind
- [x] emph
- [x] quote
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
