// Copyright (C) 2014 The 6502-rs Developers
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in the
//    documentation and/or other materials provided with the distribution.
// 3. Neither the names of the copyright holders nor the names of any
//    contributors may be used to endorse or promote products derived from this
//    software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.

use address::{Address, AddressDiff};
use memory::{STACK_ADDRESS_LO, STACK_ADDRESS_HI};

// Useful for constructing Status instances
pub struct StatusArgs {
    pub negative: bool,
    pub overflow: bool,
    pub unused: bool,
    pub brk: bool,
    pub decimal_mode: bool,
    pub disable_interrupts: bool,
    pub zero: bool,
    pub carry: bool,
}

impl StatusArgs {
    pub fn none() -> StatusArgs {
        StatusArgs { negative: false,
                     overflow: false,
                     unused: false,
                     brk: false,
                     decimal_mode: false,
                     disable_interrupts: false,
                     zero: false,
                     carry: false, }
    }
}

pub bitflags! {
    flags Status: u8 {
        static ps_negative           = 0b10000000,
        static ps_overflow           = 0b01000000,
        static ps_unused             = 0b00100000, // JAM: Should this exist?
        static ps_brk                = 0b00010000,
        static ps_decimal_mode       = 0b00001000,
        static ps_disable_interrupts = 0b00000100,
        static ps_zero               = 0b00000010,
        static ps_carry              = 0b00000001,
    }
}

impl Status {
    pub fn default() -> Status {
        // TODO akeeton: Revisit these defaults.

        Status::new(StatusArgs { negative:           false,
                                 overflow:           false,
                                 unused:             true,
                                 brk:                false,
                                 decimal_mode:       false,
                                 disable_interrupts: true,
                                 zero:               false,
                                 carry:              false, } )
    }

    pub fn new(StatusArgs { negative,
                            overflow,
                            unused,
                            brk,
                            decimal_mode,
                            disable_interrupts,
                            zero,
                            carry }: StatusArgs) -> Status
    {
        let mut out = Status::empty();

        if negative           { out = out | ps_negative           }
        if overflow           { out = out | ps_overflow           }
        if unused             { out = out | ps_unused             }
        if brk                { out = out | ps_brk                }
        if decimal_mode       { out = out | ps_decimal_mode       }
        if disable_interrupts { out = out | ps_disable_interrupts }
        if zero               { out = out | ps_zero               }
        if carry              { out = out | ps_carry              }

        out
    }

    pub fn set_with_mask(&mut self, mask: Status, rhs: Status) {
        *self = (*self & !mask) | rhs;
    }

    pub fn get_carry(self) -> i8 {
        if self.contains(ps_carry) { 1 } else { 0 }
    }
}

#[deriving(PartialEq, Eq, PartialOrd, Ord)]
pub struct StackPointer(pub u8);

impl StackPointer {
    pub fn to_address(&StackPointer(sp): &StackPointer) -> Address
    {
        STACK_ADDRESS_LO + AddressDiff(sp as u16)
    }
}

pub struct Registers {
    pub accumulator:     i8,
    pub index_x:         u8,
    pub index_y:         u8,
    pub stack_pointer:   StackPointer,
    pub program_counter: Address,
    pub status:          Status
}

impl Registers {
    pub fn new() -> Registers {
        // TODO akeeton: Revisit these defaults.
        Registers {
            accumulator:     0,
            index_x:         0,
            index_y:         0,
            stack_pointer:   StackPointer(STACK_ADDRESS_HI.get_offset()),
            program_counter: Address(0),
            status:          Status::default()
        }
    }
}

