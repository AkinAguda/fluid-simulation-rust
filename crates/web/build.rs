fn main() {
    css_mod::Compiler::new()
        .add_modules("src/**/*.css")
        .unwrap()
        .compile("../../public/app.css")
        .unwrap();
}
