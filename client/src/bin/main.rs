use client::graphics::tempo::run;

fn main() {
    pollster::block_on(run());
}
