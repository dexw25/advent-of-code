//
// Created by Derek Witt on 12/8/20.
//

#ifndef AOC_2020_DAY_08_H
#define AOC_2020_DAY_08_H

#include <string_view>
#include "gconsole.h"

using namespace std;

// Run program without loop and return acc at end
constexpr int day_08_1(const string_view &in) {
    GConsole comp(in);

    comp.run_all(false);

    return comp.get_acc();
}

// Try to fix the program and find the accumulator value when it terminates
int constexpr day_08_2(const string_view &in) {
    bool run = true;
    int i = 0;

    const GConsole comp_base(in); // program, assemble!
    GConsole comp(comp_base); // keep separate for mods

    // Search until program does not loop or we run out of search space
    while (run) {
        // restore initial state
        comp = comp_base;

        // Modify program, seek next jmp or nop
        while ((comp.program[i].op) != Opcodes::NOP && (comp.program[i].op) != Opcodes::JMP) {
            i++;
        }

        // patch instruction, just nop a jump or jump a nop
        switch (comp.program[i].op) {
            case Opcodes::JMP:
                comp.program[i].op = Opcodes::NOP;
                break;
            case Opcodes::NOP:
                comp.program[i].op = Opcodes::JMP;
                break;
            default:
                throw; // Should not be here
        }
        i++; // Advance intruction ptr

        // Run until repeat or termination
        while (!comp.would_repeat() && run) {
            run = comp.step();
        }
    }

    return comp.get_acc();
}

#endif //AOC_2020_DAY_08_H
