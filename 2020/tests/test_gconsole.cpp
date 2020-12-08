//
// Created by Derek Witt on 12/8/20.
//
#include "gconsole.h"
#include <string_view>
#include <iostream>
using namespace std;

// test program, will leave 5 in acc when it starts to loop
constexpr string_view loop = "nop +0\n"
                             "acc +1\n"
                             "jmp +4\n"
                             "acc +3\n"
                             "jmp -3\n"
                             "acc -99\n"
                             "acc +1\n"
                             "jmp -4\n"
                             "acc +6";
// test program, will leave 8 after terminating
constexpr string_view ret8 = "nop +0\n"
                             "acc +1\n"
                             "jmp +4\n"
                             "acc +3\n"
                             "jmp -3\n"
                             "acc -99\n"
                             "acc +1\n"
                             "nop -4\n"
                             "acc +6";

constexpr GConsole<10> assemble_and_run_program(const string_view &in) {
    GConsole<10> comp(in);

    comp.run_all();

    return comp;
}

// Static tests for game console interpreter
int main() {
    // Test the assembler (which runs on instantiation)
    static_assert(GameAssembler::stoi_impl("12345") == 12345);
    static_assert(GameAssembler::stoi_impl("-12345asdfg") == -12345);
    static_assert(GameAssembler::stoi_impl("+12345") == 12345);
//    static_assert(GameAssembler::stoi_impl("dsafdgh") == 0); // This should throw, IE not compile
    static_assert((GameAssembler::assemble_line("acc +1") == Instruction{Opcodes::ACC, 1}));
    static_assert((GameAssembler::assemble_line("acc -1") == Instruction{Opcodes::ACC, -1}));

    // Program Tests
    constexpr GConsole comp = assemble_and_run_program(loop);
    static_assert(comp.acc == 5);

    constexpr GConsole comp2 = assemble_and_run_program(ret8);
    static_assert(comp2.acc == 8);
}