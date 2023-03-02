# webthesis

front-end for the thesis.

## notes

## doing



## todo
- [ ] bug
  - [ ] jumping to a listing goes to the parent page. how to solve the issue of having the reference on the proper page? maybe the easiest way is indeed to have it as an inline <Code/>? e.g. if there is a listing on the page with the same label, jump to it. otherwise, show the inline.
- [ ] performance
  - [ ] load data on the server
  - [ ] switch from iframes to components?
  - [ ] add service worker for offline perf https://kit.svelte.dev/docs/service-workers
- [ ] layout
  - [ ] add footer (just the current section, expands into full ToC)
- [ ] performance
  - [ ] rollup optimization? https://stackoverflow.com/questions/69260715/skipping-larger-chunks-while-running-npm-run-build

## wishlist:
  - [ ] add full text search
  - [ ] add index of nouns and iframes to show where they are? [index](https://en.wikipedia.org/wiki/Index_(publishing))


## references

### tech

- https://github.com/caddyserver/cache-handler
- https://joshcollinsworth.com/blog/build-static-sveltekit-markdown-blog
- https://tradingstrategy.ai/blog/optimizing-page-load-speed-on-sveltekit
- https://joyofcode.xyz/sveltekit-routing
- https://pimpmytype.com/font-size/

### theoretical

- https://en.wikipedia.org/wiki/Table_of_contents
