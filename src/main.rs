use radiate::*;

fn main() {
    let store = vec![
        (
            NodeType::Input,
            vec![Op::var(0), Op::var(1), Op::var(2), Op::var(1)],
        ),
        (NodeType::Edge, vec![Op::weight(), Op::identity()]),
        (NodeType::Vertex, ops::all_ops()),
        (NodeType::Output, vec![Op::sigmoid()]),
    ];
    let codec = GraphCodec::directed(5, 4, store);
    let engine = GeneticEngine::builder()
        .codec(codec)
        .fitness_fn(|genotype| 5)
        .build();
}
