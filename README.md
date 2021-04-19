# heic-to-dynamic-gnome-wallpaper 🌅 🎞 🌇

This project offers a cli to convert MacOS's dynamic wallpaper stored in `.heic` image containers to dynamic wallpaper definitions usable in GNOME.

Both solar position and time based wallpaper definitions are supported. Although due to the nature of the gnome wallpapers, solar based wallpapers will be transferred to a time based division, approximated from the solar position defined for each image.

## 🧰 Usage

Since most options are dictated by the image information, tweakable options are sparse. All you need to do is specify the path to the image you want to convert.  
Optionally you can specify a path under which the new images extracted from the `heic` are to be stored including the `xml` specification for GNOME.

``` sh
heic-to-dynamic-gnome-wallpaper

USAGE:
    heic-to-gnome-xml-wallpaper [OPTIONS] <IMAGE>

FLAGS:
    -h, --help
            Prints help information

    -V, --version
            Prints version information


OPTIONS:
    -d, --dir <DIR>
            Specifies into which directory created images should be written to. Default is the parent directory of the
            given image.

ARGS:
    <IMAGE>
            Image which should be transformed

```

## 📦 Installation

You may either use the provided pre-built binary on this repository. Or build the project yourself on your local machine, you'll need a working rust toolchain in this case, check out the instructions [here](https://www.rust-lang.org/tools/install).

### Pre-built binaries
> not yet provided, CI has to be set up
``` sh
$ mkdir -p ~/.local/bin
$ curl resource-link --output heic-to-dynamic-gnome-wallpaper
$ chmod +x heic-to-dynamic-gnome-wallpaper
$ ./heic-to-dynamic-gnome-wallpaper /path/to/some/image
```


### Local via `cargo install`

``` sh
$ cargo install heic-to-dynamic-gnome-wallpaper
```

### Local manual

``` sh
$ git clone https://github.com/jwuensche/heic-to-dynamic-gnome-wallpaper
$ cd heic-to-dynamic-gnome-wallpaper
$ cargo install --path .
$ # OR
$ cargo build --release
```
