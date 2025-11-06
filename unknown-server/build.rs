fn main() {
    // trigger recompilation when a new migration is added
    println!("cargo:rerun-if-changed=migrations/");

    // trigger recompilation when an asset or template is added
    println!("cargo:rerun-if-changed=templates/");
    println!("cargo:rerun-if-changed=assets/");

    // Load jinja templates
    minijinja_embed::embed_templates!("templates/", &[".j2.html"]);
}
