use std::env;
use nu_errors::ShellError;
use nu_protocol::{CallInfo, ReturnValue};

pub struct Kubectl {
	pub command_map: Vec<Box<List>>,
	pub commands: Vec<String>,

	pub namespace: String,
	pub kubeconfig: String,
}

type FnPtr = fn(& Kubectl) -> Result<Vec<ReturnValue>, ShellError>;

pub enum List {
	Cons(String, Option<FnPtr>, Vec<Box<List>>),
}


impl Default for Kubectl {
	fn default() -> Kubectl {
		Kubectl {
			command_map: vec!{
				Box::new(List::Cons(String::from("config"), None, vec!{
					Box::new(List::Cons(String::from("view"), Some(Kubectl::config_view), vec!{})),
				})),
			},
			commands: vec!{},
			namespace: String::from("default"),
			kubeconfig: String::from(""),
		}
	}
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

		match call_info.args.get("kubeconfig") {
			Some(kubeconfig) => {
				self.kubeconfig = kubeconfig.as_string()?;
			},
			None => {
				match env::var("KUBECONFIG") {
					Ok(kubeconfig) => {
						self.kubeconfig = kubeconfig;
					},
					Err(_) => {
						match env::var("HOME") {
							Ok(home) => {
								let mut config: String = String::from("");
								config.push_str(&home);
								config.push_str("/.kube/config");
								self.kubeconfig = config;
							},
							Err(_) => Err(ShellError::unexpected(String::from("")))?
						}
					}
				}
			}
		}

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

	pub fn exec(& self) -> Result<Vec<ReturnValue>, ShellError> {
		match self.get_command(0, &self.command_map) {
			Some(ptr) => ptr(self),
			None => Ok(vec!{}), // TODO: Print help
		}
	}

	fn get_command(& self, index: usize, nodes: &Vec<Box<List>>) -> Option<FnPtr> {
		for node in nodes.iter() {
			let List::Cons(name, ptr, list) = &**node;

			if *name == self.commands[index] {
				if index + 1 == self.commands.len() {
					return *ptr
				}
				return self.get_command(index+1, list)
			}
		};
		None
	}

}
