# Bibrus
Bibrus approach no. 2

## Commit structure

```dif
? - optional
* - zero or more
Name:
<category><project>?<!/?>?(breaking change/untested): <present tense>
Description:
<+/-/~>* (add/remove/comment)
```

Use backticks (\`text\`) when referring to code (e.g.`trait`, `counter`, `String` )

Breaking changes have to be tested!

### Categories
  - fix - fixing unwanted behaviour
  - perf - performance, optimization
  - doc - document, documentation
  - feat - feature
  - refactor - changing structure but not functionality
  - format - only text, visual changes
  - init - only for initialization purposes

## Acceptable langs
 - Rust
 - TypeScript

