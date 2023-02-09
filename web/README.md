# webthesis

front-end for the thesis.

## todo

- [ ] include tailwind
- [ ] specific components
  - [ ] paragraph
  - [ ] code
  - [ ] image
  - [ ] citation
  - [ ] listing
  - [ ] figure
  - [ ] footnote
  - [ ] quote
- [ ] deal with escaped characters
  - [ ] `\{` -> `{`
  - [ ] etc.

## done

- [x] create basic node component to recursively display all information


## svelte

```bash
npm run dev

# or start the server and open the app in a new browser tab
npm run dev -- --open

npm run build

npm run preview
```

to deploy your app, need to install an [adapter](https://kit.svelte.dev/docs/adapters) for the target environment.
