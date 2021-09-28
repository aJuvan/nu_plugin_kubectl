use std::mem::take;
use nu_errors::ShellError;
use nu_protocol::{CallInfo};

#[derive(Default)]
pub struct Kubectl {
	pub commands: Vec<String>,
	pub namespace: String,
}

impl Kubectl {

	pub fn new() -> Self {
		Default::default()
	}

	pub fn parse(&mut self, call_info: CallInfo) -> Result<(), ShellError> {
		self.namespace = if let Some(namespace) = call_info.args.get("namespace") {
			match namespace.as_string() {
				Ok(n) => n,
				Err(_) => String::from("default"),
			}
		} else {
			String::from("default")
		};

		for arg in call_info.args.positional {
			for a in arg {
				match a.as_string() {
					Ok(a) => {
						self.commands.push(a);
						Ok(())
					},
					Err(_) => Err(ShellError::labeled_error(
						"Unrecognised argument type",
						"'parse' given non-string positional parameter",
						a.tag.span,
					))
				}?
			}
		}

		Ok(())
	}

	pub fn exec(&mut self) -> Result<Vec<String>, ShellError> {
		Ok(take(&mut self.commands))
	}

}
