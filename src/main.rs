use radiate::*;

mod snake;
fn main() {
    let snake_game = snake::Game::new();
    let score = snake_game.run_game_human();
    println!("{}", score);

    // let store = vec![
    //     (
    //         NodeType::Input,
    //         vec![Op::var(0), Op::var(1), Op::var(2), Op::var(1)],
    //     ),
    //     (NodeType::Edge, vec![Op::weight(), Op::identity()]),
    //     (NodeType::Vertex, ops::all_ops()),
    //     (NodeType::Output, vec![Op::sigmoid()]),
    // ];
    // let codec = GraphCodec::directed(5, 4, store);
    // let engine = GeneticEngine::builder()
    //     .codec(codec)
    //     .fitness_fn(snake::snake_game_fitness_func)
    //     .build();
    //
    // let best = engine.iter().take(500).last();
    // // let genotype = best.unwrap().value().clone();
    //
    // println!("\x1Bc");
    //
    // println!("{:?}", best.unwrap().score());
    // snake::snake_game_fitness_func(genotype);
    //
    //
}
