# What
`svscr` stands for '**s**a**v**e **scr**eenshot' and does exactly that.
If you have an image saved in your clipboard, this command line tool will
save it into a file. It can be used with any picture in the clipboard,
not just screenshots.

# Why
`svscr` is a quick and dirty way to save a clipped screenshot or a picture from
a browser to any folder with any name using your terminal. Primarily used to
screencap jack's discord messages.

# How
Put an image into your clipboard somehow and call `svscr`. This will create a
`.png` image in user's home directory with a timestamp for file name. This 
directory is platform specific.
An example of such name: `2024-05-19-131457.png`.

You can supply a positional argument `NAME` with desired file name. This
argument can also have file path and file extension. If the supplied path is 
relative it is appended to user's home folder path. If it is absolute, it
will replace the default path.

`-d` or `--dir` can be optionally used to supply a target directory to save the
file to. If you supply directory with this flag *and* within file name, then
they will be concatenated. E.g. `svscr inner/pic -d outer` will create a file
with this path: `.../outer/inner/pic.png`.

`-x` or `--extension` can be used to add an extension to your file. It will 
override the extension you supply within file name. E.g. `svscr pic.png -x webp`
will save `pic.webp`, not `pic.png`.

# Credits
- anyhow
- arboard
- chrono
- clap
- home
- image
- jack
