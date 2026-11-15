# nanoshot

Simple screenshotting tool for MacOS.

- Puts a clickable icon on the menu bar.
- All screenshots go in `~/screenshots`.

## Install

Install pre-built binary:
```console
$ mise use -g github:antichrome1/nanoshot-rs@v0.9.5
$ nanoshot
```

Or, use nix to build from source:
```console
$ git clone https://github.com/antichrome1/autolift && cd autolift
$ nix-build
$ ./result/bin/nanoshot
```

## Use

- Click on icon - Take screenshot
- Ctrl+Click on icon - Take 5 screenshots in a row (esc to exit)

## License

ISC © 2026 Aditya Singh <antichrome1@proton.me> <br>
AI usage: No AI agents/tools were/are used in this project.<br>
