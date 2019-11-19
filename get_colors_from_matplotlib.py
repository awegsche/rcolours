#!/usr/bin/env python
# coding: utf-8

# In[41]:


from matplotlib import colors
import re


def color_to_struct(name, col, i, which_set="base"):
    name = re.sub("/", "_slash_", re.sub("[: ']", "_", name)).upper()
    message = "Colour `{}` from the set `{}`. (Colour number `{}`)".format(
        name, which_set, i)
    return """
/// Colour `{}`
///
/// {}
/// ## Representations:
/// - int tuple `{}`
/// - float tuple `{}`
/// - hex: `{}`
pub const {}: Color = Color({}, {}, {});\n""".format(
        debug(col), message, to_int_tuple(col), to_float_tuple(col), to_hex(col), name, int(col[0]), int(col[1]), int(col[2]),)


def from_hex_b(b):
    if b == 'a' or b == 'A':
        return 10
    if b == 'b' or b == 'B':
        return 11
    if b == 'c' or b == 'C':
        return 12
    if b == 'd' or b == 'D':
        return 13
    if b == 'e' or b == 'E':
        return 14
    if b == 'f' or b == 'F':
        return 15
    if b == '1':
        return 1
    if b == '2':
        return 2
    if b == '3':
        return 3
    if b == '4':
        return 4
    if b == '5':
        return 5
    if b == '6':
        return 6
    if b == '7':
        return 7
    if b == '8':
        return 8
    if b == '9':
        return 9
    return 0


def from_hex(s):
    value = 0
    for i, b in enumerate(s):
        value += 16**(len(s)-i-1)*from_hex_b(b)
    return value


def color(col):
    if isinstance(col, tuple):
        return (255.0*col[0], 255.0*col[1], 255.0*col[2])
    if isinstance(col, str):
        if col[0] == '#' and len(col) == 7:
            return (from_hex(col[1:3]), from_hex(col[3:5]), from_hex(col[5:7]))
    print("error, type = {}".format(type(col)))


def to_hex(col):
    return "#{:02X}{:02X}{:02X}".format(int(col[0]), int(col[1]), int(col[2]))


def debug(col):
    return "(r = {}, g = {}, b = {})".format(int(col[0]), int(col[1]), int(col[2]))


def to_float_tuple(col):
    return "({}, {}, {})".format(round(col[0]/255.0, 2), round(col[1]/255.0), round(col[2]/255.0))


def to_int_tuple(col):
    return "({}, {}, {})".format(int(col[0]), int(col[1]), int(col[2]))


def main():
    i = 0
    with open("all_the_colors.txt", "w") as outfile:
        for k, col in colors.BASE_COLORS.items():
            outfile.write(color_to_struct(k, color(col), i, 'BASE_COLORS'))
            i += 1
        for k, col in colors.CSS4_COLORS.items():
            outfile.write(color_to_struct(k, color(col), i, 'CSS4_COLORS'))
            i += 1
        for k, col in colors.XKCD_COLORS.items():
            outfile.write(color_to_struct(k, color(col), i, 'XKCD_COLORS'))
            i += 1


if __name__ == "__main__":
    main()

# In[ ]:
