//
// Created by Derek Witt on 12/8/20.
//

#include "gconsole.h"

// Returns false if terminated
bool GConsole::step() {
    if (this->pc == this->program.size()) return false; // Could except but I don't like exceptions in usual non-error states

    auto [op, arg] = this->program[this->pc];
    this->counters[pc]++; // Count executions of each instruction
    switch (op) {
        case Opcodes::NOP:
            this->pc++;
            break;
        case Opcodes::ACC:
            this->acc += arg;
            this->pc++;
            break;
        case Opcodes::JMP:
            this->pc +=arg;
            break;
    }

    return true;
}

// Return true if next step() would repeat
bool GConsole::would_repeat() {
        return  (this->counters[this->pc] == 1) ;
}
