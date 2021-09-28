use indexmap::indexmap;

use crate::Kubectl;

use nu_errors::ShellError;
use nu_plugin::Plugin;
use nu_protocol::{
    CallInfo, Primitive, ReturnSuccess, ReturnValue, ShellTypeName, Signature, SyntaxShape,
    UntaggedValue, Value, Dictionary,
};
use nu_source::{HasSpan, SpannedItem};

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
				.rest(
						"rest",
						SyntaxShape::String,
						"subsequent kubectl commands",
				)
				.filter()
		)
	}

	fn begin_filter(&mut self, call_info: CallInfo) -> Result<Vec<ReturnValue>, ShellError> {
		self.parse(call_info)?;
		Ok(self
				.exec()?
				.into_iter()
				.map(|x| {
						indexmap!{
							String::from("int") => Value::from(UntaggedValue::int(0)),
							String::from("str") => Value::from(x),
						}
				})
				.map(|x|
						ReturnSuccess::value(
								UntaggedValue::Row(Dictionary::from(x))
						)
				)
				.collect()
		)
	}

}
