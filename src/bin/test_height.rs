use ds_heightmap::Runner;

fn main() {
    let mut runner = Runner::new();
    let output = runner.ds();

    println!("data: {:?}", output.data);
    println!("max: {}", output.max);
    println!("min: {}", output.min);
}
