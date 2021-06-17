# twemopick

:stuck_out_tongue_closed_eyes: Generate data URL of Twemoji image.

## Usage

*Currently no executable is available, so you have to use Cargo to run twemopick.*

### Get SVG data URL

```bash
cargo run ⌛
```

You can set `-s` or `--svg` to select SVG explicitly.

### Get PNG data URL

```
cargo run ⌛ -p
```

Or `--png` can be used.

## Why data URL, not just raw image data?

**TL;DR: I do not have to put the data into the clipboard directly.**

The implementation of using clipboards in Rust is not cross-platform (means we have to implement it for each platforms), and it makes me unhappy. If I try to put the raw image data directly to the clipboard, using clipboard in Rust is unavoidable.

In the other hand, data URL can contain binary data (and the type of data) in the text! The data URL is human-readable, so I can stream it into standard output without any worry.

And the data streamed into standard output can easily put into the clipboard using the pipe feature. Like this in Linux ( maybe you need to install `xsel` if this doesn't work ):

```
cargo run ⌛ | xsel -bi
```

or in macOS:

```
cargo run ⌛ | pbcopy
```

## License

MIT License. See **LICENSE.md**.

## TODO

- [ ] Release the executable using GitHub Actions