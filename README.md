# dicers

cli rust tool to roll dices.

## Install

```
$ git clone https://github.com/LucioCarvalhoDev/dicers.git
$ cargo build --release
```

## Usage

Call `dicers <dice>` to roll only one dice and end the program. `<dice>` follow the format `<quantity?>d<faces>`:

```
$ dicers 1d20
[15] // example
```
```
$ dicers d6
[5] // example
```

Call `dicers` will make the program watch for multiple inputs of `<dice>`:

```
$ dicers
> 2d6
[4, 1] // example
> d20
[17] // example
```

### Flags

| Format | Effect                                            | Exemple                  |
| ------ | ------------------------------------------------- | ------------------------ |
| output | output mode; `0` for minimalist, `1` for extended | `--output 0` or `-o 0`   |
