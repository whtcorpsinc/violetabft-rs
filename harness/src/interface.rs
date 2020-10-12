// Copyright 2019 EinsteinDB Project Authors. Licensed under Apache-2.0.

// Copyright 2015 CoreOS, Inc.
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

use violetabft::{evioletabftpb::Message, storage::MemStorage, VioletaBFT, Result};
use std::ops::{Deref, DerefMut};

/// A simulated VioletaBFT façade for testing.
///
/// If the contained value is a `Some` operations happen. If they are a `None` operations are
/// a no-op.
///
// Compare to upstream, we use struct instead of trait here.
// Because to be able to cast Interface later, we have to make
// VioletaBFT derive Any, which will require a lot of dependencies to derive Any.
// That's not worthy for just testing purpose.
pub struct Interface {
    /// The violetabft peer.
    pub violetabft: Option<VioletaBFT<MemStorage>>,
}

impl Interface {
    /// Create a new interface to a new violetabft.
    pub fn new(r: VioletaBFT<MemStorage>) -> Interface {
        Interface { violetabft: Some(r) }
    }

    /// Step the violetabft, if it exists.
    pub fn step(&mut self, m: Message) -> Result<()> {
        match self.violetabft {
            Some(_) => VioletaBFT::step(self, m),
            None => Ok(()),
        }
    }

    /// Read messages out of the violetabft.
    pub fn read_messages(&mut self) -> Vec<Message> {
        match self.violetabft {
            Some(_) => self.msgs.drain(..).collect(),
            None => vec![],
        }
    }
}

impl From<Option<VioletaBFT<MemStorage>>> for Interface {
    fn from(violetabft: Option<VioletaBFT<MemStorage>>) -> Self {
        Self { violetabft }
    }
}

impl From<VioletaBFT<MemStorage>> for Interface {
    fn from(violetabft: VioletaBFT<MemStorage>) -> Self {
        Self { violetabft: Some(violetabft) }
    }
}

impl Deref for Interface {
    type Target = VioletaBFT<MemStorage>;
    fn deref(&self) -> &VioletaBFT<MemStorage> {
        self.violetabft.as_ref().unwrap()
    }
}

impl DerefMut for Interface {
    fn deref_mut(&mut self) -> &mut VioletaBFT<MemStorage> {
        self.violetabft.as_mut().unwrap()
    }
}
