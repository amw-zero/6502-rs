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

use std::num::Int;
use std::ops::Add;

// The idea here is that it doesn't make sense to add two addresses, but it
// does make sense to add an address and an "address-difference". (If this
// is too annoying to work with we should let it go.)
#[derive(Copy, PartialEq, Eq, PartialOrd, Ord, Show)]
pub struct AddressDiff(pub i32);

#[derive(Copy, PartialEq, Eq, PartialOrd, Ord, Show)]
pub struct Address(pub u16);

impl Add<AddressDiff> for Address {
    type Output = Address;

    fn add(self, AddressDiff(rhs): AddressDiff) -> Address {
        let Address(lhs) = self;

        // TODO akeeton: Do a checked cast.
        Address(((lhs as i32) + rhs) as u16)
    }
}

impl Add for AddressDiff {
    type Output = AddressDiff;

    fn add(self, AddressDiff(rhs): AddressDiff) -> AddressDiff {
        let AddressDiff(lhs) = self;
        AddressDiff(lhs + rhs)
    }
}

#[derive(Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CheckedAddressDiff(u16);

impl Add<CheckedAddressDiff> for Address {
    type Output = Address;

    fn add(self, CheckedAddressDiff(rhs): CheckedAddressDiff) -> Address {
        let Address(lhs) = self;

        // We probably don't want to overflow when doing arithmetic in our own
        // code.
        debug_assert!(lhs.checked_add(rhs).is_some());

        Address(lhs + rhs)
    }
}

impl Address {
    pub fn to_u16(&self) -> u16 {
        match *self {
            Address(address_) => address_
        }
    }

    pub fn to_usize(&self) -> usize {
        self.to_u16() as usize
    }

    pub fn get_page_number(&self) -> u8 {
        (self.to_u16() & 0xff00 >> 8) as u8
    }

    pub fn get_offset(&self) -> u8 {
        (self.to_u16() & 0x00ff) as u8
    }
}

