mod heuristics;
mod meta_heuristics;
mod simplex;

use heuristics::bag_problem;
fn main() {
    let bag_items = vec![
        bag_problem::BagItem::from(1, 2_000.00, 8),
        bag_problem::BagItem::from(2, 3_000.00, 12),
        bag_problem::BagItem::from(3, 500.00, 5),
        bag_problem::BagItem::from(4, 800.00, 12),
        bag_problem::BagItem::from(5, 950.00, 10),
        bag_problem::BagItem::from(6, 100.00, 1)
    ];

    let bag = bag_problem::apply_avg_heuristic(&bag_items, 6_000.00);
    for item in &bag {
        println!("Item id: {}, weight: {}, utility: {}", item.id, item.weight, item.utility);
    }
    let total_utility: u32 = bag.clone().into_iter().map(|item| item.utility).sum();
    let total_weight: f32 = bag.clone().iter().map(|item| item.weight).sum();
    println!("Total utility: {total_utility}, total weight: {total_weight}");
    println!("Using average heuristic");
}
