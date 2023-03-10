mcfilter
========

This is a program that uses one McStas data file to filter another such data file.
The idea is that you have a large Source file that you run through an instrument
and put a virtual output at some point that intercepts only those traces that would
make it though the early part of the geometry. Then you can use the output of that
virtual output, the Filter file, to select only those traces in the original file
that could pass through the geometry. This could allow you to shorten the execution
time of some McStas simulations.

The intent is a command-line program with the following calling sequence

    mcfilter [-v] [-h] [-d] [<Source name> <Filter name>]

-d turns on debugging output
-v will print version information
-h will print essentially this information about the calling sequence

Note that the files may be omitted but that if the source is present then the filter is required.
