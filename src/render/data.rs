// Copyright 2020 Sebastian Wiesner <sebastian@swsnr.de>

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

// 	http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use pulldown_cmark::CowStr;

#[derive(Debug, PartialEq)]
pub struct Link<'a> {
    pub(crate) index: u16,
    pub(crate) target: CowStr<'a>,
    pub(crate) title: CowStr<'a>,
}

/// Data associated with rendering state.
///
/// Unlike state attributes state data represents cross-cutting
/// concerns which are manipulated across all states.
#[derive(Debug)]
pub struct StateData<'a> {
    /// A list of pending reference links.
    ///
    /// These are links which mdcat already created a reference number for
    /// but didn't yet write out.
    pub(super) pending_links: Vec<Link<'a>>,
    /// The reference number for the next link.
    pub(super) next_link: u16,
}

impl<'a> StateData<'a> {
    pub(crate) fn add_link(mut self, target: CowStr<'a>, title: CowStr<'a>) -> (Self, u16) {
        let index = self.next_link;
        self.next_link += 1;
        self.pending_links.push(Link {
            index,
            target,
            title,
        });
        (self, index)
    }

    pub(crate) fn take_links(self) -> (Self, Vec<Link<'a>>) {
        let links = self.pending_links;
        (
            StateData {
                pending_links: Vec::new(),
                ..self
            },
            links,
        )
    }
}

impl<'a> Default for StateData<'a> {
    fn default() -> Self {
        StateData {
            pending_links: Vec::new(),
            next_link: 1,
        }
    }
}
