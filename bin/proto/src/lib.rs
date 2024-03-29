// Copyright 2019 EinsteinDB Project Authors. Licensed under Apache-2.0.

mod confchange;
mod confstate;

pub use crate::confchange::{
    new_conf_change_single, parse_conf_change, stringify_conf_change, ConfChangeI,
};
pub use crate::confstate::conf_state_eq;
pub use crate::protos::evioletabftpb;

#[allow(dead_code)]
#[allow(unknown_lints)]
#[allow(clippy::all)]
#[allow(renamed_and_removed_lints)]
#[allow(bare_trait_objects)]
mod protos {
    include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

    use self::evioletabftpb::Snapshot;

    impl Snapshot {
        /// For a given snapshot, determine if it's empty or not.
        pub fn is_empty(&self) -> bool {
            self.get_metadata().index == 0
        }
    }
}

pub mod prelude {
    pub use crate::evioletabftpb::{
        ConfChange, ConfChangeSingle, ConfChangeTransition, ConfChangeType, ConfChangeV2,
        ConfState, Entry, EntryType, HardState, Message, MessageType, Snapshot, SnapshotMetadata,
    };
}

pub mod util {
    use crate::evioletabftpb::ConfState;

    impl<Iter1, Iter2> From<(Iter1, Iter2)> for ConfState
    where
        Iter1: IntoIterator<Item = u64>,
        Iter2: IntoIterator<Item = u64>,
    {
        fn from((voters, learners): (Iter1, Iter2)) -> Self {
            let mut conf_state = ConfState::default();
            conf_state.mut_voters().extend(voters.into_iter());
            conf_state.mut_learners().extend(learners.into_iter());
            conf_state
        }
    }
}
