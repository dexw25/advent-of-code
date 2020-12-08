//
// Created by Derek Witt on 12/8/20.
//

#ifndef AOC_2020_GCONSOLE_H
#define AOC_2020_GCONSOLE_H
#include <string_view>
#include <vector>
#include <tuple>


using namespace std;

// virtual machine for advent of code problems, it is an interpreter for a kind of assembly
struct GConsole {
    enum Opcodes {
        NOP,
        ACC,
        JMP,
    };

    vector<tuple<Opcodes, int>> program; // Stores program (could template and array<> this)
    vector<int> counters; // Counts the number of times each address has been run (see above)
    int acc = 0; // Accumulator
    size_t pc=0; // Initialize pc to 0

    constexpr static Opcodes str_to_opcode(const string_view &opcode) {
        struct pair{const string_view str; Opcodes val;};
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

    constexpr static bool is_digit(const char c) {
        return (c >= '0' && c <= '9');
    }

    // Convert ints with some basic recursion
    constexpr static int stoi_impl(const string_view &str) {
        int acc = 0;

        for(auto &c: str) {
            acc *= 10;
            if (is_digit(c)) {
                acc += c - '0';
            } else {
                break;
            }
        }

        return acc;
    }

    constexpr static tuple<Opcodes, int> assemble_line(const string_view &line) {
        auto opcode_str = line.substr(0, line.find(' '));
        auto arg_str = line.substr(line.find_first_of("+-0123456789"));

        Opcodes op = str_to_opcode(opcode_str);
        int arg = ((arg_str[0] == '+') ? 1 : -1) * stoi_impl(arg_str.substr(1));

        return make_tuple(op, arg);
    }

    // Initialize the system, assembling a given program into memory
    explicit GConsole(const string_view &prog) {
        size_t offset=0, end=0;
        while (end != string_view::npos) {
            end = prog.find('\n', offset);
            auto line = prog.substr(offset, end - offset);

            auto instr = assemble_line(line);

            this->program.emplace_back(instr);
            offset = end + 1;
        }

        // Initialize profiling counters
        this->counters = vector(program.size(), 0);
    };

    bool step(); // execute one program step
    bool would_repeat(); // Scan counters, determine if repeats exist

    [[nodiscard]] int get_acc() const {
        return this->acc;
    }

};
#endif //AOC_2020_GCONSOLE_H
