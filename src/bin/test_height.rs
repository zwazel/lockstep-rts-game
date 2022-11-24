use ds_heightmap::Runner;

fn main() {
    let mut runner = Runner::new();
    runner.set_width(10);
    runner.set_height(10);
    runner.set_depth(10.0);
    runner.set_rough(0.3); // q: what is this? a: the roughness of the terrain, 0.0 is flat, 1.0 is very rough. Default: 1.0.

    let output = runner.ds();

    println!("data:");
    for row in output.data {
        println!("{:?}", row);
    }
    println!("max: {}", output.max);
    println!("min: {}", output.min);
}
