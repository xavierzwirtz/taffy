pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_grow: 1f32,
                flex_shrink: 0f32,
                flex_basis: taffy::style::Dimension::Points(0f32),
                min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), ..Size::auto() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_grow: 1f32,
                flex_shrink: 0f32,
                flex_basis: taffy::style::Dimension::Percent(0.5f32),
                max_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(20f32), ..Size::auto() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(120f32),
                    height: taffy::style::Dimension::Points(50f32),
                    ..Size::auto()
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
