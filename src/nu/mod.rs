use crate::Kubectl;

use nu_errors::ShellError;
use nu_plugin::Plugin;
use nu_protocol::{
	CallInfo, ReturnValue, Signature, SyntaxShape,
};

impl Plugin for Kubectl {

	fn config(&mut self) -> Result<Signature, ShellError> {
		Ok(Signature::build("kubectl")
				.desc("Nu shell kubectl wrapper")
				.named(
						"namespace",
						SyntaxShape::String,
						"Specifies kubernetes namespace. Set to 'default' if not passed.",
						Some('n'),
				)
				.named(
						"kubeconfig",
						SyntaxShape::String,
						"Specifies kubernetes config path.",
						None,
				)
				.rest(
						"rest",
						SyntaxShape::Any,
						"subsequent kubectl commands",
				)
				.filter()
		)
	}

	fn begin_filter(&mut self, call_info: CallInfo) -> Result<Vec<ReturnValue>, ShellError> {
		self.parse(call_info)?;
		Ok(self.exec()?)
	}

}
