/* tectonic/jpegimage.h -- JPEG-format image functions
   Copyright 2016 the Tectonic Project
   Licensed under the MIT License
*/

#ifndef TECTONIC_JPEGIMAGE_H
#define TECTONIC_JPEGIMAGE_H

#include <tectonic/stubs.h>

struct jpeg_info
{
    unsigned short height;
    unsigned short width;
    unsigned char bits_per_component;
    unsigned char num_components;
    double xdpi;
    double ydpi;
};

extern int check_for_jpeg (rust_input_handle_t file);
extern int jpeg_scan_file (struct jpeg_info *info, rust_input_handle_t);

#endif
