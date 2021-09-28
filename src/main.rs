use nu_plugin::serve_plugin;
use nu_plugin_kubectl::Kubectl;

fn main() {
	serve_plugin(&mut Kubectl::new());
}
