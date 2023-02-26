# webthesis

front-end for the thesis.

## notes

make layout into a full width block for each paragraph, divded in two blocks, one is the text, the other one holds citations/footnotes. citations and footnotes are at different z-indexes, interverting as they become more recently clicked (also toggle for all citations vs. all footnotes)

## doing

- [ ] table of contents
  - [ ] iframe to show the excerpt of chapters / sections
  - [ ] separate pages into sections as well, keep full chapter page

## todo

- [ ] navigation
  - [ ] next / previous / up navigation both in header and at bottom of page
- [ ] listing
  - [ ] show as iframe on hover
  - [ ] jump to on click
- [ ] citation
  - [ ] hovering above citation highlights the corresponding item
- [ ] layout
  - [x] make left column for footnotes (behave the same as citations)
  - [ ] add footer (just the current section, expands into full ToC)
  - [x] use the system default when using dark theme
  - [ ] have lightmode/darkmode as a store?
  - [ ] make font-size smaller
  - [ ] make font-size mobile be base, scale to lg on md:+ / https://pimpmytype.com/font-size/ (-2px for citations and foonotes)
- [ ] mobile
  - [ ] footnotes click is janky
- [ ] references
  - [ ] deal with finding the actual names for the chapters/sections/subsections, etc. (probably client side, similar to how we find the labels)
- [ ] add full text search
- [ ] add index of nouns and iframes to show where they are? [index](https://en.wikipedia.org/wiki/Index_(publishing))

## done
- [x] write to taeyoon
- [x] add header (just a single icon opening up a bar with all other icons)
- [x] added location to anchors, so that one can jump between pages
- [x] render single chapters
- [x] overlap of right col full width
- [x] citation (parse bib.json to find the proper ones)
  - [x] have a `citationItem` component which renders the long one
  - [x] have a message passing from a nested child short footnote or citation, caught by the node, and generating the approriate element with unique ID
  - [x] the right side of the row (the citation+footnote, is flex col, with overflow scrolling
- [x] dark mode
- [x] autodeploy on push? makefile
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
