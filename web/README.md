# webthesis

front-end for the thesis.

## notes

can i try and integrate multiple panels?

## doing

- [ ] on desktop, show `next section/chapter ->`, with the literal stacked below
  - [ ] and add an animation on hover?
- [ ] make line length smaller (66 characters ideal)
  - [ ] ```paragraph { max-inline-size: 66ch; }```

## todo
- [ ] listing should also hold its own full path (handleMissingId warn)
- [ ] performance
  - [ ] add service worker for offline perf https://kit.svelte.dev/docs/service-workers, https://vite-pwa-org.netlify.app/frameworks/sveltekit.html
- [ ] layout
  - [ ] toc: animate unfolding subsections (with a > or + icon)
  - [ ] footer (just the current section, expands into full ToC w/ interaction observer api)
  - [ ] Have each chapter and section as components rather than pages.
  - [ ] have some sort of perception of how far we are in the book (percent? graphic? pages?)
  - [ ] fix issues with spacing in references (e.g. should be a space after a [a-zA-Z], but not after `(`)
- [ ] interface
  - [ ] option to highligh inline refs, and other elements
  - [ ] past the max depth, if a toc heading has children, the visibility is toggled, animated line on the left and clickable on whole body

## wishlist:
  - [ ] add full text search (meaning across pages!)
  - [ ] add index of nouns and list where they are (how??) [index](https://en.wikipedia.org/wiki/Index_(publishing))
  - [ ] overlay/pop up for code snippet <Code/>?
  - [ ] bookmarking using localstorage?
  - [ ] margin scribbles using localstorage?
  - [ ] graph of listings references? https://twitter.com/genmon/status/1631347161823694849?s=20
  - [ ] download to read offline -> service workers
  - [ ] "continue from your last visit?"

## references

### tech

- https://joshcollinsworth.com/blog/build-static-sveltekit-markdown-blog
- https://tradingstrategy.ai/blog/optimizing-page-load-speed-on-sveltekit
- https://joyofcode.xyz/sveltekit-routing
- https://pimpmytype.com/font-size/

### theoretical

- https://en.wikipedia.org/wiki/Table_of_contents
