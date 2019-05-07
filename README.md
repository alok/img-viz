# `img-viz`

A utility to allow you to view random images inside specified
directories. It's more memory-efficient than the simple Bash and Python
scripts because it uses reservoir sampling and avoids the cost of
storing a bunch of file paths in a list.

I wrote it so I could get a feeling for data in ML code.

## Installation

-   `icat` script in [kitty](https://github.com/kovidgoyal/kitty). *May*
    require you to use `kitty` as your terminal emulator with `$TERM`
    set to `xterm-kitty`.
-   `cargo install --git https://github.com/alok/img-viz`

## Usage

``` {.shell}
img-viz -n N [dirs]
```

where N is a natural number and `dirs` are directories. They default to
100 and the current working directory.
