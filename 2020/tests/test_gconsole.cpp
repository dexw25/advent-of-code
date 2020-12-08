//
// Created by Derek Witt on 12/8/20.
//
#include "gconsole.h"
#include <tuple>
using namespace std;

// Static tests for game console interpreter
int main() {
    static_assert(GConsole::stoi_impl("12345") == 12345);
    static_assert(GConsole::assemble_line("acc +1") == make_tuple(GConsole::Opcodes::ACC, 1));
    static_assert(GConsole::assemble_line("acc -1") == make_tuple(GConsole::Opcodes::ACC, -1));
}