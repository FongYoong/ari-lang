### Documentation:
📖 [**Read the docs!**]()

***
### Download:
⭐ [**Linux**](https://github.com/FongYoong/ari-lang/releases/download/0.1.0/ari-linux)
⭐ [**Windows**](https://github.com/FongYoong/ari-lang/releases/download/0.1.0/ari-windows.exe)

***
### [🎮 Try it now](https://repl.it/@FongChien/Ari-Language-Linux?lite=true)

***
### Clone on REPL.IT
[![Run on Repl.it](https://repl.it/badge/github/FongYoong/ari-lang)](https://repl.it/github/FongYoong/ari-lang)
***
Ari 0.1.0 's notable features include:
* Compiled in one lonely executable
* Colored and helpful errors
* Array arithmetic for Number and String types
* Various native functions:
    *  Number operations (power, log, modulo, absolute, floor, ceiling, max, min)
    * String/Number conversions (to_string, to_number)
    * String operations (split, to_lowercase, to_uppercase)
    * Array/String operations (length, insert, remove)
    * Functional Array operations (map, filter, reduce)
    * Quick Array creation (range, linspace, repeat)
    * Random array generation (random_choose, random_normal)
    * File operations (read_file, write_file)
    * Web stuff (serve_static_folder, web_get, web_post)
* Rust dependencies
    * [lazy_static 1.4.0](https://docs.rs/lazy_static/1.4.0/lazy_static/) for mutable global singletons
    * [termcolor 1.1](https://docs.rs/termcolor/1.1.2/termcolor/) for colored terminal output
    * [rayon 1.5.0](https://docs.rs/rayon/1.5.0/rayon/) to parallelize array arithmetic
    * [rand 0.8.3](https://crates.io/crates/rand) to generate random values
    * [rand_distr 0.4.0](https://docs.rs/rand_distr/0.4.0/rand_distr/) for normal distribution
    * [reqwest 0.11](https://docs.rs/reqwest/0.11.0/reqwest/) for GET/POST requests
    * [rocket 0.4.6](https://api.rocket.rs/v0.4/rocket/) to setup web server
    * [rocket_contrib 0.4.6](https://api.rocket.rs/v0.4/rocket_contrib/) to serve static folder in server
***