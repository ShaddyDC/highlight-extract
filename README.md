# highlight-extract

Tool to extract highlights and notes taken on an Onyx Boox e-reader and to export them as Markdown.

I have only tested on my personal files and some I have found online in similar projects.
If you have a file that is failing or unsupported, open an issue with a comprehensive sample, and I will attempt to fix it.

## Usage

```bash
$ highlight-extract ./data.txt # or simply cargo run -- ./data.txt
```

On Nix with [Flakes](https://nixos.wiki/wiki/Flakes) enabled, you can run it without installation with


```bash
$ nix run github:shaddydc/highlight-extract ./data.txt
```
