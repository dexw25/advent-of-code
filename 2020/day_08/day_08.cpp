//
// Created by Derek Witt on 12/8/20.
//

#include "day_08.h"
#include <string_view>
using namespace std;

#include "gconsole.h"

int day_08_1(const string_view &in) {

    GConsole comp(in);

    while(!comp.would_repeat()) {
        comp.step();
    }

    return comp.acc;
}

// Fix the program and try to find the accumulator value when it terminates
int day_08_2(const string_view &in) {
    bool run = true;
    int acc = 0;
    int i=0;

    // Search until program terminates naturally or we run out of search space
    while(run) {
        GConsole comp(in); // construct computer

        // Modify program, seek next jmp or nop
        while(get<0>(comp.program[i]) != comp.Opcodes::NOP && get<0>(comp.program[i]) != comp.Opcodes::JMP) {
            i++;
        }

        // Swap instruction
        switch (auto &op = get<0>(comp.program[i])) {
            case comp.Opcodes::JMP:
                op = comp.Opcodes::NOP;
                break;
            case comp.Opcodes::NOP:
                op = comp.Opcodes::JMP;
                break;
            default:
                throw; // Should not be here
        }
        i++; // Advance intruction ptr

        // Run until repeat or termination
        while (!comp.would_repeat() && run) {
            run = comp.step();
        }

        acc = comp.acc;
    }

    return acc;
}