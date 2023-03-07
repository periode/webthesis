# webthesis

front-end for the thesis.

## notes

can i try and integrate multiple panels?

## doing

- [ ] listing should also hold its own full path (handleMissingId warn)
- [ ] footer (showing the current section, expands into full ToC w/ https://github.com/metonym/svelte-intersection-observer and writing to store passed as props)

## todo
- [ ] tests?
- [ ] performance
  - [ ] add service worker for offline perf https://kit.svelte.dev/docs/service-workers, https://vite-pwa-org.netlify.app/frameworks/sveltekit.html
  - [ ] lazy load chapter sections?
- [ ] layout
  - [ ] Have each chapter and section as components rather than pages.
  - [ ] fix issues with spacing in references (e.g. should be a space after a [a-zA-Z], but not after `(`)
- [ ] routing bug:
    - [ ] manually invalidate to force a reload of data? https://kit.svelte.dev/docs/load#invalidation-manual-invalidation

## wishlist:
  - [ ] add full text search (meaning across pages!)
  - [ ] add index of nouns and list where they are (how??) [index](https://en.wikipedia.org/wiki/Index_(publishing))
  - [ ] overlay/pop up for code snippet <Code/>?
  - [ ] bookmarking using localstorage?
  - [ ] margin scribbles using localstorage?
  - [ ] graph of listings references? https://twitter.com/genmon/status/1631347161823694849?s=20
  - [ ] download to read offline -> service workers
  - [ ] "continue from your last visit?"
- [ ] interface
  - [ ] option to highlight inline refs, and other elements

## references

### tech

- https://joshcollinsworth.com/blog/build-static-sveltekit-markdown-blog
- https://tradingstrategy.ai/blog/optimizing-page-load-speed-on-sveltekit
- https://joyofcode.xyz/sveltekit-routing
- https://pimpmytype.com/font-size/

### theoretical

- https://en.wikipedia.org/wiki/Table_of_contents
