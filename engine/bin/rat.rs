use xander_engine::core::dice::{D100, D12, D20, D4, D6};

fn main() {
    let expr = D100 + D20 + D12 - (D6 - D4);
    let evaled = expr.evaluate();
    println!("{expr} => {evaled} == {}", evaled.result());
}
