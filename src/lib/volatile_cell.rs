// Zinc, the bare metal stack for rust.
// Copyright 2014 Vladimir "farcaller" Pouzanov <farcaller@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A cell that with volatile setter and getter.

use core::kinds::marker;
use core::intrinsics::{volatile_load, volatile_store};

// TODO(bharrisau) I don't know enough about markers - is it better
// to just use an Unsafe<T> here instead?
pub struct VolatileCell<T> {
  value: T,
  invariant: marker::InvariantType<T>,
}

impl<T> VolatileCell<T> {
  pub fn new(value: T) -> VolatileCell<T> {
    VolatileCell {
      value: value,
      invariant: marker::InvariantType::<T>,
    }
  }

  #[inline]
  pub fn get(&self) -> T {
    unsafe {
      volatile_load(&self.value)
    }
  }

  #[inline]
  pub fn set(&self, value: T) {
    unsafe {
      volatile_store(&self.value as *T as *mut T, value)
    }
  }
}
