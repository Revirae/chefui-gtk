fn main() {
    glib_build_tools::compile_resources(
        &["chef/1/resources"],
        "chef/1/resources/resources.gresource.xml",
        "chef_1.gresource",
    )
}
