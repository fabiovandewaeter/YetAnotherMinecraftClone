use client::graphics::renderer::run;

fn main() {
    pollster::block_on(run());
}
