use criterion::*;
use graph_builder::prelude::*;

mod common;

use common::gen::uniform_edge_list;
use common::*;

fn from_edge_list(c: &mut Criterion) {
    let mut group = c.benchmark_group("from_edge_list");
    group.sampling_mode(SamplingMode::Flat);

    for direction in [
        Direction::Outgoing,
        Direction::Incoming,
        Direction::Undirected,
    ] {
        for csr_layout in [
            CsrLayout::Unsorted,
            CsrLayout::Sorted,
            CsrLayout::Deduplicated,
        ] {
            group.bench_function(
                format!("{}_{:?}_{:?}", SMALL.name, direction, csr_layout),
                |b| bench_from_edge_list(b, SMALL, direction, csr_layout),
            );
            group.bench_function(
                format!("{}_{:?}_{:?}", MEDIUM.name, direction, csr_layout),
                |b| bench_from_edge_list(b, MEDIUM, direction, csr_layout),
            );
            group.bench_function(
                format!("{}_{:?}_{:?}", LARGE.name, direction, csr_layout),
                |b| bench_from_edge_list(b, LARGE, direction, csr_layout),
            );
        }
    }

    group.finish();
}

fn bench_from_edge_list(
    b: &mut criterion::Bencher,
    Input {
        name: _,
        node_count,
        edge_count,
    }: Input,
    direction: Direction,
    csr_layout: CsrLayout,
) {
    let edges: Vec<(usize, usize, ())> = uniform_edge_list(node_count, edge_count, |_, _| ());
    b.iter_batched(
        || EdgeList::new(edges.clone()),
        |mut edge_list| {
            black_box(Csr::from((
                &mut edge_list,
                node_count,
                direction,
                csr_layout,
            )))
        },
        criterion::BatchSize::SmallInput,
    )
}

criterion_group!(benches, from_edge_list);
criterion_main!(benches);
