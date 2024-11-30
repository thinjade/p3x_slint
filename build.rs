fn main() {
    slint_build::compile("ui/ui.slint").unwrap();
    shadow_rs::new().unwrap();
}
