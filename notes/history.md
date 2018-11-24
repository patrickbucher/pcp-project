[The History of Rust](https://www.youtube.com/watch?v=79PSagCD_AY)

- initial compiler written in OCaml
- constant try and error
- Servo (rewrite of Gecko)
    - Feedback from the Servo team

2006-2010: "The Personal Years" (Graydon Hoare)

- type inference
- not really object oriented: not everything is an object
- short keywords (fn, ret, obj)
- generics
- memory safety (no wild pointers)
- typestate system (no null pointers)
- mutability control
- side-effect control
- possibility to break rules (unsafe)
- multi-paradigm
- 38ksloc Ocaml compiler
- garbage collector (!)
- channels

2010-2012: "The Graydon Years"

- 4.5Msloc C++ code
- goal: compiler eliminates human failure
- Graydon Hoare worked at Mozilla
- 2-3 programmers at the beginning
- development in type system
- elimination of garbage collector
- moving of features from the core language into libraries
- Graydon Hoare steps down
- core team: 6-9 people at Mozilla
- task and chan keywords (now in libraries)

2012-2014: "The Typesystem Years"

- three important pointer types
    - garbage collected (gone)
    - unique pointer (box)
    - borrowed pointer (reference)
- Cargo/Crates.io
- user camps
    - C++: low level programming, performance
    - scripting languages: user experience (convenience: Cargo)
    - functional languages: type system
- RFC process since 2014
    - also for the core team

2015-2016: "Release Year"

- May 15th 2015: 1.0.0
    - only backwards compatible changes will be introduced
    - more libraries were developed/stabilized
- main areas: games, operating systems, web development
- rustfmt
- minor release every six weeks

2016: "The Production Year"

- Rust since Firefox 45 (on Linux and Mac OS X; Windows: since 47)
    - .mp4-Parser
- Dropbox
    - file system in Rust (Go otherwise)
