# prettify_pinyin
##### v1.0.1
---

### About
Turn pinyin written with tone numbers and turn it into pinyin with node marks. prettify_pinyin accepts input in the [CC-CEDICT](https://cc-cedict.org/wiki/format:syntax) pinyin format (space separated syllables with tone numbers at the end of each syllable), for example: "ni3 hao3" will get turned into "nǐ hǎo".

This project is a Rust translation of [John Heroy's](https://github.com/johnheroy) [prettify-pinyin](https://github.com/johnheroy/prettify-pinyin) JavaScript project.

### Usage
```rust
extern crate prettify_pinyin;

use prettify_pinyin::prettify;

let test = String::from("ma1 ma2 ma3 ma4 ma");
let formatted: String = prettify(test);

println!("{}", formatted); // --> mā má mǎ mà ma
```

### Contributors
- [Preston Wang-Stosur-Bassett](http://stosur.info)

### License
[MIT](https://github.com/sotch-pr35mac/prettify_pinyin/blob/master/LICENSE)
