# zed-unicode

Add completion to unicode characters in Zed. It completes by default the mathematical symbols, the arrows, the greek letters.

If you want all characters from unicode, or almost all, you can can set up in your `settings.json`:

```json
{
  "lsp": {
    "unicode": {
      "settings": {
        "include_all_symbols": false
      }
    }
  }
}
```

It's very simple to use, you can just type `->` which transforms into `→` and `->>` into `↠`.
