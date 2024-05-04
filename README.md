# Running

If you use CLion, the process is simple. Open the `Tester` directory with CLion, add a release configuration and press the play button!

## Contributing

Participation in this open-source project is not only appreciated but also actively encouraged.

The simplest way to contribute is by testing out your own math/algorithms. The only way to get a true picture of C++ vs Rust is to test many different algorithms and scenarios.

### Add Functions to Rust
To add your own functions to test, simply define your function in lib.rs and build with `cargo build --release`.

### Running Rust Function from CPP
To tell CPP that your Rust functin exists, define it like so:
```cpp
extern "C" float landon_clip(float input, float drive);
```
making sure that the name and args are the same from lib.rs.

## Support

If you like my work and would like to support me creating more audio applications, check out my [Patreon](https://www.patreon.com/ViatorDSP) where you can donate and download all of my current plugins! I also have a [GumRoad](https://viatordsp.gumroad.com/?_gl=1*18tqfoy*_ga*MTg2MjcxNDgyNS4xNjg5OTI3NDE3*_ga_6LJN6D94N6*MTY5MjM5NjQ1Ni4xODguMS4xNjkyMzk2NTExLjAuMC4w) store with my most recent plugins.

## License

MIT License

Copyright (c) 2023 Landon Viator

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
 
