mod heuristics;
mod meta_heuristics;
mod simplex;
mod default_structures;
use default_structures::BagItem;
use heuristics::bag_problem;
use meta_heuristics::bag_problem_using_ag;
fn main() {
    bag_problem();
    bag_problem_with_ag();
}

fn bag_problem() {
    let bag_items = vec![
        BagItem::from(1, 2_000.00, 8),
        BagItem::from(2, 3_000.00, 12),
        BagItem::from(3, 500.00, 5),
        BagItem::from(4, 800.00, 12),
        BagItem::from(5, 950.00, 10),
        BagItem::from(6, 100.00, 1)
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

fn bag_problem_with_ag() {
    let bag_items = vec![
        BagItem::from(1, 2_000.00, 8),
        BagItem::from(2, 3_000.00, 12),
        BagItem::from(3, 500.00, 5),
        BagItem::from(4, 800.00, 12),
        BagItem::from(5, 950.00, 10),
        BagItem::from(6, 100.00, 1),
        
    ];

    let bag = bag_problem_using_ag::apply_genetic_algorithm(&bag_items, 6_000.00, Some(0.2), Some(90));
    for item in &bag {
        println!("Item id: {}, weight: {}, utility: {}", item.id, item.weight, item.utility);
    }
    let total_utility: u32 = bag.clone().into_iter().map(|item| item.utility).sum();
    let total_weight: f32 = bag.clone().iter().map(|item| item.weight).sum();
    println!("Total utility: {total_utility}, total weight: {total_weight}");
    println!("Using genetic-algorithm meta-heuristic");
}
