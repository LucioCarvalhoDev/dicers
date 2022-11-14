# dicers

cli rust tool to roll dices.

## Install

```
$ git clone https://github.com/LucioCarvalhoDev/dicers.git
$ cargo build --release
```

## Usage

Call `dicers` will make the program watch for multiple inputs with the format `<quantity>d<faces>`:

```
$ dicers
> 2d6
[4, 1] // example
> d20
[17] // example
```

Call `dicers -d <dice>` to roll only one dice and end the program.

### Flags

| Format | Effect                                            | Exemple                  |
| ------ | ------------------------------------------------- | ------------------------ |
| dice   | roll only one dice given as input                 | `--dice 1d6` or `-d 1d6` |
| output | output mode; `0` for minimalist, `1` for extended | `--output 0` or `-o 0`   |
