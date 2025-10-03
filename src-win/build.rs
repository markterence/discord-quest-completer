fn main() {
    embed_resource::compile("embed_icon.rc",embed_resource::NONE).manifest_optional().unwrap();
}