# prettify_pinyin

### About
Turn pinyin written with tone numbers and turn it into pinyin with node marks. prettify_pinyin accepts input in the [CC-CEDICT](https://cc-cedict.org/wiki/format:syntax) pinyin format (space separated syllables with tone numbers at the end of each syllable), for example: "ni3 hao3" will get turned into "nǐ hǎo".

### Usage
```rust
use prettify_pinyin::prettify;

let formatted: String = prettify("ma1 ma2 ma3 ma4 ma");

println!("{}", formatted); // --> mā má mǎ mà ma
```

### License
[MIT](https://github.com/sotch-pr35mac/prettify_pinyin/blob/master/LICENSE)
