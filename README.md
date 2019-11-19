# rcolours [![Build Status](https://travis-ci.com/awegsche/rcolours.svg?branch=master)](https://travis-ci.com/awegsche/rcolours)
CSS and [XKCD](https://blog.xkcd.com/2010/05/03/color-survey-results/) colour package for rust.

Makes CSS and XKCD colours (copied shamelessly from [matplotlib](https://matplotlib.org/)'s [color module](https://matplotlib.org/2.0.2/api/colors_api.html)) available as `u8`-tuples and provides basic conversion functions into different representations.

An emphasis is on transforming the colour tuple to [ANSI escape sequences](https://en.wikipedia.org/wiki/ANSI_escape_code) for colouring console output.

## Documentation

The doc should be available [here](https://awegsche.github.io/rcolours/colors/index.html). For each colour there is detailled information on how to represent it in different ways (for now hex, int/float tuple in RGB space). Other colour spaces (HUV, L\*AB, maybe also XYZ and spectrum samples) to come). 
