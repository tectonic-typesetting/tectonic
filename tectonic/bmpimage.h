/* tectonic/bmpimage.h -- BMP-format image functions
   Copyright 2016 the Tectonic Project
   Licensed under the MIT License
*/

#ifndef TECTONIC_BMPIMAGE_H
#define TECTONIC_BMPIMAGE_H

#include <tectonic/stubs.h>

struct bmp_info {
    int	width;
    int height;
    double xdpi;
    double ydpi;
    int	bits_per_component;
    int	num_components;
};

extern int bmp_scan_file(struct bmp_info *info, rust_input_handle_t file);
extern int tt_check_for_bmp(rust_input_handle_t file);

#endif
