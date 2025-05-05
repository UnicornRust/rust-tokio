pub mod self_ref;
pub mod pin_project;


pub async fn run() {
    self_ref::self_ref_swap();
    pin_project::use_pin_raw().await;
}
