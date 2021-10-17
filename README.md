# petz_ih (Petz Image Helper)
Drag and drop image files onto petz_ih.exe (or onto a shortcut or symbolic link to petz_ih.exe) to resave them as PNG, with the alpha/opacity of all white (R: 255, G: 255, B: 255) pixels set to 0. The resulting images will be saved to a folder named `output` in the same directory as petz_ih.exe.

petz_ih may also be invoked through the command line:
```
petz_ih 0.1.0
Jeremiah Lister <jwlister@protonmail.com>
Converts images to PNG and makes all white pixels transparent

USAGE:
    petz_ih.exe [FLAGS] [OPTIONS] <PATH>...

FLAGS:
    -h, --help        Prints help information
    -n, --no-pause    Exit immediately after finishing, not requiring the user to press enter
    -V, --version     Prints version information

OPTIONS:
    -o, --output <DIR>    Sets a custom output directory

ARGS:
    <PATH>...    Paths of the image files to process (and/or of directories containing them)
```
