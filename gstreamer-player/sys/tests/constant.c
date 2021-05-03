// Generated by gir (https://github.com/gtk-rs/gir @ b5ff6f8)
// from gir-files (https://github.com/gtk-rs/gir-files @ 6088bb6)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git @ 208138a)
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT((gint) GST_PLAYER_COLOR_BALANCE_BRIGHTNESS);
    PRINT_CONSTANT((gint) GST_PLAYER_COLOR_BALANCE_CONTRAST);
    PRINT_CONSTANT((gint) GST_PLAYER_COLOR_BALANCE_HUE);
    PRINT_CONSTANT((gint) GST_PLAYER_COLOR_BALANCE_SATURATION);
    PRINT_CONSTANT((gint) GST_PLAYER_ERROR_FAILED);
    PRINT_CONSTANT((gint) GST_PLAYER_STATE_BUFFERING);
    PRINT_CONSTANT((gint) GST_PLAYER_STATE_PAUSED);
    PRINT_CONSTANT((gint) GST_PLAYER_STATE_PLAYING);
    PRINT_CONSTANT((gint) GST_PLAYER_STATE_STOPPED);
    PRINT_CONSTANT((gint) GST_PLAYER_THUMBNAIL_JPG);
    PRINT_CONSTANT((gint) GST_PLAYER_THUMBNAIL_PNG);
    PRINT_CONSTANT((gint) GST_PLAYER_THUMBNAIL_RAW_BGRx);
    PRINT_CONSTANT((gint) GST_PLAYER_THUMBNAIL_RAW_NATIVE);
    PRINT_CONSTANT((gint) GST_PLAYER_THUMBNAIL_RAW_xRGB);
    return 0;
}
