use anyhow::Result;
use yazi_macro::{act, succ};
use yazi_shared::{data::Data, event::CmdCow};

use crate::input::{Input, InputMode, InputOp};

impl Input {
	pub fn execute(&mut self, cmd: CmdCow) -> Result<Data> {
		macro_rules! on {
			($name:ident) => {
				if cmd.name == stringify!($name) {
					return act!($name, self, cmd);
				}
			};
			($name:ident, $alias:literal) => {
				if cmd.name == $alias {
					return act!($name, self, cmd);
				}
			};
		}

		macro_rules! on_if {
			($cond:expr, $name:ident) => {
				if $cond && cmd.name == stringify!($name) {
					return act!($name, self, cmd);
				}
			};
		}

		let op = self.snap().op;

		on!(r#move, "move");
		on!(backward);
		on!(forward);

		match self.mode() {
			InputMode::Normal => {
				on!(insert);
				on!(visual);
				on!(replace);

				on!(delete);
				on!(yank);
				on!(paste);

				on_if!(matches!(op, InputOp::None), undo);
				on!(redo);

				on_if!(matches!(op, InputOp::Select(_)), lowercase);
				on_if!(matches!(op, InputOp::Select(_)), uppercase);
			}
			InputMode::Insert => {
				on!(visual);

				on!(backspace);
				on!(kill);

				on!(complete);
			}
			InputMode::Replace => {}
		}

		succ!();
	}
}
