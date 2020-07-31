# cl_plotter

## About
Developed by:
- Nik Steinbrügge
- Julian Bahner

The cl_potter is a command line plotter offering basic functionalities for linear algebraic functions. 
It prints the given function directly in the terminal. The terminal size is always automatically fully used when plotting.

## Functionalities
Multiple functions can be added to the cl_plotter. After adding, they are in 'active' state, meaning they will be plotted.
Available operations on a function are:
- fx: calculate the function at a specific value
- min/max: calculate the mininum/maximum of the function in a given range
- integral: calculate the Riemann integral in a given range
- diff: perform a differentiation and plot it in the same coordination system
- xmin/xmax: Change the x-axis range
- ymin/ymax: Change the y-axis range


## Self-criticism
- Good insights in Rust
- Happy with parser and implemented functionality
- Nice GUI (logic, structure)
- Well defined requirements
- Good basis for further development
- Good cooperation working on different modules

- Not a "real" CLI tool (no key listener)
- No time left for axis labeling 
- No time left for exponentiation
