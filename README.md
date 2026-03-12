# rust-digikam-orm

![GitHub Actions Workflow
Status](https://img.shields.io/github/actions/workflow/status/afistfullofash/rust-digikam-orm/rust.yml?branch=master)

This implements a rust ORM for querying the [digiKam](https://www.digikam.org/) database.

Currently this only supports querying the SQLite database not the MySQL version.

In addition this also has a program which leverages the ORM to set the wallpaper based on digiKam tags.

## Features

- Working Model\'s for:
  - Image\'s
  - Tag\'s

## Build

``` shell
cargo build
```

## digikam-wallpaper

This sets the wallpaper by filtering Image Metadata in the Digikam database. It can also get if the system is in `Dark` or `Light` mode and apply extra tags based on which mode the system is in.

### Usage

When run without arguments it will attempt to read from a configuration file and if that fails if will apply default values Run `digikam-wallpaper -h` to view command line arguments.

#### Automatically setting wallpaper when the Light/Dark Mode setting changes
With [Darkman](https://gitlab.com/WhyNotHugo/darkman) installed add the following to `${XDG_DATA_HOME}/darkman/digikam-wallpaper.sh`

``` shell
#!/bin/sh
digikam-wallpaper --dark-mode $1
```

Then every time `darkman` is invoked or the System setting changes `digikam-wallpaper` will be invoked with the correct mode according to `darkman`.

### Configuration File

The default configuration directory depends on your system:

- Linux: XDG Directory Specification i.e: `$HOME/.config/digikam-wallpaper/config.json`

To generate a configuration file with default values run:

``` shell
digikam-wallpaper --config-template
```

### Todo

- Filters
  - Time Based
    - Sunrise/Sunset
    - Season
    - Holidays
    - Current Weather Based
- Run as a deamon to dynamically change the wallpaper when filters are met
