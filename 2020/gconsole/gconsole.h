//
// Created by Derek Witt on 12/8/20.
//

#ifndef AOC_2020_GCONSOLE_H
#define AOC_2020_GCONSOLE_H

#include <string_view>
#include <array>
#include <tuple>
#include <algorithm>
#include <compare>

using namespace std;

enum Opcodes {
    NOP,
    ACC,
    JMP,
};

// Default to NOP 0
struct Instruction {
    Opcodes op = Opcodes::NOP;
    int arg = 0;

    // Use default comparisons, since each element is comparable directly
    bool operator==(const Instruction &) const = default;
};

// Wrap static methods in regular class, to simplify tests
struct GameAssembler {
    // Find opcode for str, not the most optimal but it can run at compile time
    constexpr static Opcodes str_to_opcode(const string_view &opcode) {
        struct pair {
            const string_view str;
            Opcodes val = Opcodes::NOP;
        };
        constexpr pair const tab[] = {
                {"nop", Opcodes::NOP},
                {"acc", Opcodes::ACC},
                {"jmp", Opcodes::JMP},
        };

        for (auto &e: tab) {
            if (opcode == e.str) return e.val;
        }

        throw;
    }

    // Is decimal digit
    constexpr static bool is_digit(const char c) {
        return (c >= '0' && c <= '9');
    }

    // Convert ints with sign, throws if int is not at start of str
    constexpr static int stoi_impl(const string_view &str) {
        int acc = 0;
        int mult = 1;

        auto it = str.begin();

        if (*it == '-') {
            mult = -1;
            it++;
        } else if (*it == '+') {
            it++;
        }

        for (; it < str.end(); it++) {
            if (is_digit(*it)) {
                acc *= 10;
                acc += *it - '0';
            } else {
                if (it == str.begin()) throw;
                else break;
            }
        }

        return acc * mult;
    }

    // Asemble an instruction instance from a line of a program file
    static constexpr Instruction assemble_line(const string_view &line) {\
        // Divide input string first
        auto opcode_str = line.substr(0, line.find(' '));
        auto arg_str = line.substr(line.find_first_of("+-0123456789"));

        // Parse opcode
        Opcodes op = str_to_opcode(opcode_str);

        // parse int using custom stoi written for constexpr
        int arg = stoi_impl(arg_str);

        return {op, arg};
    }
};

// virtual machine for advent of code problems, uses the above assembler and executes programs
template<size_t ROM_Size = 1024>
struct GConsole : GameAssembler {
    array<Instruction, ROM_Size> program; // Stores program ROM
    array<int, ROM_Size + 1> counters = {0}; // Counts the number of times each address has been run (see above)
    int acc = 0; // Accumulator
    size_t pc = 0; // Initialize pc to 0
    size_t prog_length; // Length of program, this is a hint to step() to shorten execution time, derived at instantiation

    // Initialize the system, assembling a given program into memory (presuming it fits)
    explicit constexpr GConsole(const string_view &prog) {
        size_t offset = 0, end = 0, i = 0;
        while (end != string_view::npos) {
            // Stay in bounds
            if (i >= program.size()) {
                throw std::runtime_error("ROM Overflow!!!");
            }

            end = prog.find('\n', offset);
            auto line = prog.substr(offset, end - offset);

            auto instr = assemble_line(line);

            program[i++] = instr;
            offset = end + 1;
        }
        prog_length = i + 1; // Count of the number of directives in the program while assembling
    };

    // Reeset state of machine
    constexpr void reset() {
        acc = 0;
        for (auto &i: counters) {
            i = 0;
        }

        pc = 0;
    }

// Returns false if terminated, all instruction implementations are here
    constexpr bool step() {
        if (pc == prog_length)
            return false; // signal program termination

        auto instr = program[pc];
        counters[pc]++; // Count executions of each instruction
        switch (instr.op) {
            case Opcodes::NOP:
                pc++;
                break;
            case Opcodes::ACC:
                acc += instr.arg;
                pc++;
                break;
            case Opcodes::JMP:
                pc += instr.arg;
                break;
        }

        return true;
    }

    // Run to termination, return total count of steps executed, do not loop if not directed to
    constexpr int run_all(bool loop = false) {
        int steps = 0;
        if (loop) {
            while (step()) {
                steps++;
            }
        } else {
            while (!would_repeat() && step()) {
                steps++;
            }
        }
        return steps;
    }


// Return true if next step() would repeat, signaling infinite loop if no conditional branches are implemented
    constexpr bool would_repeat() {
        return (counters[pc] == 1);
    }

    // get accumulator value
    [[nodiscard]] constexpr int get_acc() const {
        return acc;
    }

};

#endif //AOC_2020_GCONSOLE_H
