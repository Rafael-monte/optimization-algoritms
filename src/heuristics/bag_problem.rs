use crate::default_structures::BagItem;

fn avg_reason(bag_items: &[BagItem]) -> f32 {
    let sum_of_reasons: f32 = bag_items.iter().map(|item|{
        item.utility as f32/item.weight
    }).sum();
    return sum_of_reasons/bag_items.len() as f32;
}

pub fn apply_avg_heuristic(items: &[BagItem], max_weight: f32) -> Vec<BagItem> {
    let average_reason = avg_reason(items);
    let mut current_weight: f32 = 0.0;
    
    let mut best_items: Vec<BagItem> = items.iter().filter(|item|{
        let item_reason = item.utility as f32/item.weight;
        if item_reason >= average_reason && current_weight+item.weight <= max_weight {
            current_weight+=item.weight;
            return true;
        }
        return false;
    }).cloned().collect();

    let mut weights_of_best_items: f32 = best_items.iter().map(|item| item.weight).sum();
    if weights_of_best_items == max_weight {
        return best_items
    }

    let mut worst_items: Vec<BagItem> = items.iter()
    .filter(|item| {
        best_items.iter().all(|best_item| {
            best_item.id != item.id
        })
    }).cloned().collect();

    worst_items.sort_by(|a, b| a.utility.cmp(&b.utility));

    for worst_item in worst_items {
        if (worst_item.weight + weights_of_best_items) <= max_weight {
            best_items.push(worst_item);
            weights_of_best_items += worst_item.weight;
        }
    }

    return best_items
}