# webthesis

front-end for the thesis.

## notes

can i try and integrate multiple panels?

## doing

- [ ] split text.json into includes and load data on the server

## todo
- [ ] bug
  - [ ] jumping to a listing goes to the parent page. how to solve the issue of having the reference on the proper page? maybe the easiest way is indeed to have it as an inline <Code/>? e.g. if there is a listing on the page with the same label, jump to it. otherwise, show the inline.
- [ ] performance
  - [ ] add service worker for offline perf https://kit.svelte.dev/docs/service-workers
  - [ ] rollup optimization? https://stackoverflow.com/questions/69260715/skipping-larger-chunks-while-running-npm-run-build
- [ ] layout
  - [ ] add footer (just the current section, expands into full ToC)
  - [ ] Have each chapter and section as components rather than pages.
  - [ ] have some sort of perception of how far we are in the book (percent? graphic? pages?)

## wishlist:
  - [ ] add full text search (meaning across pages!)
  - [ ] add index of nouns and list where they are (how??) [index](https://en.wikipedia.org/wiki/Index_(publishing))
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
