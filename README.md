# Identicon_Tool
An identicon tool written in Rust.

A small command line utility that takes a string and creates an identicon based on that string. The output should always be identical, given the same input. An identicon is a grid of 5 x 5 squares, with each square 50 x 50 pixels, and the total grid is 250 x 250 pixels. The grids are colored according to the identicon algorithm, and each identicon is symmetrical along its vertical center axis.

Steps:
1. hash_input

2. pick_color

3. build_grid

4. filter_odd_squares

5. build_pixel_map

6. draw_image

7. save_image(input)
