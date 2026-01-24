# Hamlet Benchmarks

Inspired by infinite monkey experiment and scramble effect

- Computer have gotten pretty fast, so here goes a dumb experiment and benchmarking tool

  -i, --italics            Italics
  -d, --delay <DELAY>      Delay in ms on each iteration [default: 0]
  -r, --random-color-mode  Enable random colors for each iteration
  -j, --just-print         Just print, no randomness
  -b, --benchmark          Benchmark mode, no prints
  -h, --help               Print help
  -V, --version            Print version

## Modes

### hamlet-benchmarking

There are two ways that this can be done - the finite way and the infinite way

finite way - where I can compare all typeable characters/punctuations character by characters which would give finite iterations always, but is less fun.

infinite way - where I pick a random ones from typeable characters/punctuations and hopefully it won't be in an infinite loop, which is kinda more fun. This is one of the thoery I am excited about, on long enough scale this won't matter lets say there are 1/95 chances of hitting correct character but at the speed of computer and length of Hamlet, this will get very normalized, which can give us sort of stable benchmarking tool. On the bell curve all three major specturm will be very interesting.

There are couple more modes
- random-color-mode - don't use this if you are sensitive flashing colors

### Screensaver mode

Add delay using `-d 1` then the program would keep typing Hamlet slowly - really fun to watch 

### Upcoming(actual infinite experiment)

Basically if the output deviates from actual hamlet it resets.

