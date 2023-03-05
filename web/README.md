# webthesis

front-end for the thesis.

## notes

can i try and integrate multiple panels?

## doing

- [ ] chapter page only displays the beginning

## todo
- [ ] performance
  - [ ] add service worker for offline perf https://kit.svelte.dev/docs/service-workers
  - [ ] rollup optimization? https://stackoverflow.com/questions/69260715/skipping-larger-chunks-while-running-npm-run-build
- [ ] layout
  - [ ] add footer (just the current section, expands into full ToC)
  - [ ] Have each chapter and section as components rather than pages.
  - [ ] have some sort of perception of how far we are in the book (percent? graphic? pages?)
  - [ ] fix issues with spacing in references (e.g. should be a space after a [a-zA-Z], but not after `(`)

## wishlist:
  - [ ] add full text search (meaning across pages!)
  - [ ] add index of nouns and list where they are (how??) [index](https://en.wikipedia.org/wiki/Index_(publishing))
  - [ ] overlay/pop up for code snippet <Code/>?
  - [ ] bookmarking using localstorage?
  - [ ] margin scribbles using localstorage?
  - [ ] graph of listings references? https://twitter.com/genmon/status/1631347161823694849?s=20
  - [ ] download to read offline -> service workers

## references

### tech

- https://github.com/caddyserver/cache-handler
- https://joshcollinsworth.com/blog/build-static-sveltekit-markdown-blog
- https://tradingstrategy.ai/blog/optimizing-page-load-speed-on-sveltekit
- https://joyofcode.xyz/sveltekit-routing
- https://pimpmytype.com/font-size/

### theoretical

- https://en.wikipedia.org/wiki/Table_of_contents
