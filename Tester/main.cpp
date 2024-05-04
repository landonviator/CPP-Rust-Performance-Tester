#include <iostream>
#include <cmath>
#include <chrono>
#include <fstream>

std::string file_name = "/Users/land00m/CLionProjects/Tester/log.txt";

float clip(float input, float drive)
{
    float input_drive = input * drive;
    return 2.0f / 3.14f * std::atan(input_drive);
}

extern "C" float landon_clip(float input, float drive);

// Function to log a message to a file
void logToFile(const std::string& message, const std::string& filename) {
    // Open the file in append mode
    std::ofstream logFile;
    logFile.open(filename, std::ios::app);

    // Check if the file is opened successfully
    if (logFile.is_open()) {
        logFile << message << std::endl;
        logFile.close();
    } else {
        std::cerr << "Failed to open log file: " << filename << std::endl;
    }
}

int main()
{

    const size_t block_size = 2048;
    const size_t sample_rate = 44100;
    float cpp_time = 0.0f;
    float rust_time = 0.0f;

    volatile float cpp_result = 0.0f;
    volatile float rust_result = 0.0f;

    int times_cpp_was_faster = 0;
    int times_rust_was_faster = 0;

    for (int q = 0; q < 10; ++q)
    {
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
                cpp_result += clip((float)i, (float)k);
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
                rust_result += landon_clip((float)i, (float)k);
            }
        }

        // =================================================================
        // End Rust function timer
        // =================================================================
        stop = std::chrono::high_resolution_clock::now();
        duration = std::chrono::duration_cast<std::chrono::microseconds>(stop - start);

        rust_time = duration.count() * 0.000001;

        if (cpp_time > rust_time)
        {
            times_rust_was_faster++;
        }

        else if (cpp_time < rust_time)
        {
            times_cpp_was_faster++;
        }
    }

    logToFile("Times CPP was faster: " + std::to_string(times_cpp_was_faster) + "\n" +
                      "Times Rust was faster: " + std::to_string(times_rust_was_faster) +
              "\n" + "" + "", file_name);

    return 0;
}
