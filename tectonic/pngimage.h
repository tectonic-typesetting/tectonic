/* tectonic/bmpimage.h -- BMP-format image functions
   Copyright 2016 the Tectonic Project
   Licensed under the MIT License
*/

#ifndef TECTONIC_PNGIMAGE_H
#define TECTONIC_PNGIMAGE_H

#include <tectonic/stubs.h>

struct png_info {
    int width;
    int height;
    double xdpi;
    double ydpi;
    int	bits_per_component;
    int	num_components;
};

extern int png_scan_file (struct png_info *info, rust_input_handle_t file);
extern int check_for_png (rust_input_handle_t file);

#endif
