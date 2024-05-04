
##### Introduction
As an audio software developer, my journey through the world of digital signal processing (DSP) has been predominantly shaped by the robust and time-tested capabilities of C++. Known for its fine-grained control and efficiency, C++ has been a staple in the audio development community for crafting high-performance DSP applications. However, the evolving landscape of software development brings new languages that challenge the status quo, with Rust emerging as a notable contender. Intrigued by Rust's promises of safety and performance, I embarked on a project to evaluate whether Rust could stand toe-to-toe with C++ in high-stakes DSP tasks.

The quest began with a fundamental question: Is Rust as fast as C++ when it comes to the rigorous demands of audio processing? To answer this, it became essential to delve into precision benchmarking, ensuring that the performance measurements were accurate and that compiler optimizations did not skew the data. This article aims to share the insights gained from this meticulous approach to comparing C++ and Rust, offering a detailed guide on how to measure performance effectively while sidestepping potential pitfalls that could compromise the results.

##### Creating a Rust Library for Inclusion in C++
Before this project, I had never explicitly combined multiple languages into a single project, and it turned out that it wasn't even difficult! Here are the steps I took:

1. Create a Rust project in your C++ project. CLion has great options for generating projects with all the Rust project boiler-plate stuff.
2. Set up the Rust project to be a dynamic library, specifically in the Cargo.toml file:
```rust
[package]  
name = "rust"  
version = "0.1.0"  
edition = "2021"

[lib]  
crate-type = ["cdylib"]
```
3. Create a lib.rs file where you'll define functions to be used in the C++ project.
4. Define your functions. I started with a basic soft clipper algorithm:
```rust
#[no_mangle]  
pub extern "C" fn landon_clip(input: f32, drive: f32) -> f32  
{  
    let input_drive: f32 = input * drive;  
    return 2.0 / 3.14 * input_drive.atan();  
}
```
5. Lastly, build the library in the release configuration:
```
cargo build --release
```
Now you're done with the Rust part!

##### Pulling Rust into C++
For C++ to see our Rust library, we need to configure our CMakeLists.txt accordingly:
```cmake
cmake_minimum_required(VERSION 3.24)  
project(Tester)  
  
set(CMAKE_CXX_STANDARD 17)  
  
add_executable(Tester main.cpp)  
  
# Link the Rust library  
find_library(RUST_LIBRARY NAMES rust librust PATHS ${CMAKE_SOURCE_DIR}/rust/target/release)  
if(NOT RUST_LIBRARY)  
    message(FATAL_ERROR "Rust library not found")  
endif()  
  
target_link_libraries(Tester ${RUST_LIBRARY})
```
That's the entire CMakeLists.txt, pretty nice and simple!

The last step is telling C++ that our Rust function exists. Above the main function in your main.cpp, add this line:
```cpp
extern "C" float landon_clip(float input, float drive);
```
Make sure it's the same name and arguments as the function in the lib.rs. Now C++ knows that landon_clip exists and can run it. If you want to do a quick test to make sure it works, go ahead and try to use it in the main function like so:
```cpp
int main()
{
	float input = 0.5f;
	float drive = 5.0f;
	float signal = 0.0f;
	signal = landon_clip(float input, float drive);
	std::cout << "Signal: " << signal << std::endl; // signal should not be 0
}
```
You should see that signal is ~0.6, if it is, it worked! Now we can implement the same exact function in C++ and continue on to measuring performance.
##### Measure Code Completion Time with std::chrono
By far the simplest way to measure the performance of code is to time how long it takes to complete. We can do this pretty easily with std::chrono. Make sure you include chrono in main.cpp:
```cpp
#include <chrono>
```
Then in main.cpp, we'll implement the steps to time code. What we are going to do is wrap the code we want to test in a chrono timer. For now, let's just wrap it around the landon_clip function.
```cpp
// =================================================================  
// Start timer  
// =================================================================  
auto start = std::chrono::high_resolution_clock::now();

signal = landon_clip(input, drive);

// =================================================================  
// End C++ function timer  
// =================================================================  
auto stop = std::chrono::high_resolution_clock::now();  
auto duration = std::chrono::duration_cast<std::chrono::microseconds>(stop - start);

auto time = duration.count() * 0.000001;
```
With this, the `time` variable should show us the time it took to run this function in seconds. Running this function one time isn't going to be very helpful, because the time will read something like 0.000012 or whatever it is - some tiny number. It would be nice if it was more readable and more significant. Let's wrap the clip function in a series of for loops to simulate the behavior of it running through samples in our imaginary audio plugin. 

Let's define two variables to act as loop bounds:
```cpp
const size_t block_size = 2048;  
const size_t sample_rate = 44100;
```

Then we'll wrap the clip function inside:
```cpp
for (int i = 0; i < block_size; ++i)  
{  
    for (int k = 0; k < sample_rate; ++k)  
    {  
        signal = landon_clip(input, drive);  
    }  
}
```
Now this function will run a bunch of times and give us easier to read results. Let's do the same for our C++ function. Here's the entire main.cpp:
```cpp
// =================================================================  
// Start C++ function timer  
// =================================================================  
auto start = std::chrono::high_resolution_clock::now();  
  
// =================================================================  
// C++ clip  
// =================================================================  
for (size_t i = 0; i < block_size; ++i)  
{  
    for (size_t k = 0; k < sample_rate; ++k)  
    {  
        auto cpp_result = clip((float)i, (float)k);  
    }  
}  
  
// =================================================================  
// End C++ function timer  
// =================================================================  
auto stop = std::chrono::high_resolution_clock::now();  
auto duration = std::chrono::duration_cast<std::chrono::microseconds>(stop - start);  
  
cpp_time = duration.count() * 0.000001;  
  
// =================================================================  
// Start Rust function timer  
// =================================================================  
start = std::chrono::high_resolution_clock::now();  
  
// =================================================================  
// Rust clip  
// =================================================================  
for (int i = 0; i < block_size; ++i)  
{  
    for (int k = 0; k < sample_rate; ++k)  
    {  
        auto rust_result = landon_clip((float)i, (float)k);  
    }  
}  
  
// =================================================================  
// End Rust function timer  
// =================================================================  
stop = std::chrono::high_resolution_clock::now();  
duration = std::chrono::duration_cast<std::chrono::microseconds>(stop - start);  
  
rust_time = duration.count() * 0.000001;
```
Now the C++ function runs first and the time is measured, then the Rust function runs and the time is measured.

##### C++ Results Ruined by Compiler
Here's where things became complicated for me. Initially, when I ran this sequence, the results were perplexing. The C++ function was completing in 0 seconds, while the Rust function took 0.5 seconds. To further test, I increased the loop bounds by 1000—a substantial amount. Even then, C++ finished in 0 seconds, but Rust now took 19 seconds! This discrepancy seemed far too great. After much searching online with little to show for it, ChatGPT offered a simple insight I hadn't thought of. The compiler was eliminating the C++ function call because its result wasn't being used. So, I made some adjustments to the function calls:
```cpp
cpp_result += clip((float)i, (float)k); 
```
Now, the result was being utilized, and for a bit of variety, I changed the arguments as well. When I reran the test, the same issue occurred. Impossible, I thought. The compiler must still be optimizing something away. To definitively prevent this, I declared `cpp_result` as a `volatile float`, ensuring the compiler couldn't ignore it, and this solved the problem! I returned the loop bounds to their original values and finally began receiving realistic measurements from both languages.
##### Results
To view the results nicely, I created a function to log the results into a text file. I also ran the test 1000 times by wrapping the entire test in a for loop. I kept track of how many times each language was faster than the other. Here are the final results from the text file:
```
Times CPP was faster: 527  
Times Rust was faster: 472
```
C++ was faster more times by about 11.65%, not super significant, but when it was faster than Rust, it was faster by about 8%. 

These results bolster my confidence that writing DSP code in Rust will not significantly hinder performance. However, I plan to conduct further tests with various types of algorithms to gain a more comprehensive understanding of the overall performance landscape.

##### Final Thoughts
I am not a Rust expert, and there may be additional factors affecting performance that I'm not yet aware of. That's why I plan to extensively test more algorithms and deepen my understanding of Rust. As an initial exploration into Rust's capabilities for DSP, I believe it's entirely justified to experiment with it.

Remember, it's okay to be wrong (as I often am). Unless your code is critical for life support, it’s unlikely to cause harm. Embrace being wrong as an opportunity to experiment, learn, and grow. Don’t wait to become a master before you try new things or share your findings. If you do, you might never take the first step.

# Get the Project!
You can find the [repo](https://github.com/landonviator/CPP-Rust-Performance-Tester) for this project on my Github. Feel free to contribute by opening PR's.


