use radiate::*;

// Copied from radiate math.rs file
// use radiate::ops::math;
// const MAX_VALUE: f32 = 1e+10_f32;
// const MIN_VALUE: f32 = -1e+10_f32;
// const ZERO: f32 = 0.0_f32;
// const fn clamp(value: f32) -> f32 {
//     if value.is_nan() {
//         return ZERO;
//     }
//
//     value.clamp(MIN_VALUE, MAX_VALUE)
// }
//
// fn softmax(inputs: &[f32]) -> Vec<f32> {
//     let max_value = inputs.iter().cloned().reduce(f32::max).unwrap_or(0.0);
//     let max_value = max_value.clamp(MIN_VALUE, MAX_VALUE);
//     let exp_values: Vec<f32> = inputs
//         .iter()
//         .map(|value| (value - max_value).exp().clamp(MIN_VALUE, MAX_VALUE))
//         .collect();
//     let sum_exp_values = exp_values.iter().sum::<f32>();
//     exp_values
//         .iter()
//         .map(|value| clamp(value / sum_exp_values))
//         .collect()
// }

mod snake;
fn main() {
    let store = vec![
        (
            NodeType::Input,
            vec![Op::var(0), Op::var(1), Op::var(2), Op::var(1)],
        ),
        (NodeType::Edge, vec![Op::weight(), Op::identity()]),
        (NodeType::Vertex, ops::all_ops()),
        (NodeType::Output, vec![Op::linear()]),
    ];
    let codec = GraphCodec::directed(5, 4, store);
    let engine = GeneticEngine::builder()
        .codec(codec)
        .fitness_fn(snake::snake_game_fitness_func)
        .alter(alters!(
            GraphMutator::new(0.15, 0.05).allow_recurrent(false),
            GraphCrossover::new(0.5, 0.5),
            OperationMutator::new(0.15, 0.15)
        ))
        .build();

    // let best = radiate::ui(engine).iter().take(50000).last().unwrap();
    let best = engine.iter().logging().take(10000).last().unwrap();
    let genotype = best.value().clone();

    // println!("\x1Bc");

    println!("{:?}", best.score());
    let snake_game = snake::Game::new().print();
    snake_game.run_game_ai(genotype);
}
