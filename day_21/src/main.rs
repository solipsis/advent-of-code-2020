use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    // map of each allegen to possible ingredients
    // map of each ingredient to possible allergens
    //

    // Question I need to answer ->
    //  for a given allergen, what ingredients appear every time that allergen does

    let input: Vec<String> = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|x| x.to_string())
        .collect();

    let mut food_to_ingredients: HashMap<usize, HashSet<String>> = HashMap::new();
    let mut food_to_allergens: HashMap<usize, HashSet<String>> = HashMap::new();
    let mut allergen_to_ingredients: HashMap<String, HashSet<String>> = HashMap::new();
    let mut ingredient_to_allergens: HashMap<String, HashSet<String>> = HashMap::new();

    for (idx, food) in input.iter().enumerate() {
        let prefix = "(contains ";
        let allergens_idx = food.find(prefix).unwrap();

        let ingredients: HashSet<String> = food[..allergens_idx]
            .trim()
            .split(" ")
            .map(|x| x.to_string())
            .collect();
        let allergens: HashSet<String> = food[(allergens_idx + prefix.len())..(food.len() - 1)]
            .trim()
            .split(", ")
            .map(|x| x.to_string())
            .collect();

        for allergen in &allergens {
            for ingredient in &ingredients {
                if !allergen_to_ingredients.contains_key(allergen) {
                    allergen_to_ingredients.insert(allergen.clone(), HashSet::new());
                }
                if !ingredient_to_allergens.contains_key(ingredient) {
                    ingredient_to_allergens.insert(ingredient.clone(), HashSet::new());
                }
                allergen_to_ingredients
                    .get_mut(allergen)
                    .unwrap()
                    .insert(ingredient.clone());
                ingredient_to_allergens
                    .get_mut(ingredient)
                    .unwrap()
                    .insert(allergen.clone());
            }
        }
        food_to_ingredients.insert(idx, ingredients.clone());
        food_to_allergens.insert(idx, allergens.clone());
    }

    // for each allergen
    //     make a set from each each food that contains it
    //     intersect all of those sets
    let mut possible_allergens: HashSet<String> = HashSet::new();
    for allergen in allergen_to_ingredients.keys() {
        let mut intersection: HashSet<String> = HashSet::new();
        for (food, allergens) in food_to_allergens.iter() {
            if !allergens.contains(allergen) {
                continue;
            }
            let ingredients = food_to_ingredients.get(food).unwrap().clone();
            if intersection.len() == 0 {
                intersection = ingredients;
            } else {
                intersection = intersection
                    .intersection(&ingredients)
                    .map(|x| x.to_string())
                    .collect();
            }
        }
        for possible in &intersection {
            possible_allergens.insert(possible.clone());
        }
        println!("allergen: {}, intersection: {:?}", allergen, intersection);
    }

    let mut sum = 0;
    for (_food, ingredients) in food_to_ingredients.iter() {
        for ingredient in ingredients {
            if !possible_allergens.contains(ingredient) {
                sum += 1;
            }
        }
    }
    println!("Part1: {}", sum);

    let mut solved: HashMap<String, String> = HashMap::new();
    let mut solved_ingredients: HashSet<String> = HashSet::new();
    loop {
        println!("solved: {:#?}", solved);
        if solved.len() == allergen_to_ingredients.keys().len() {
            break;
        }
        // if length of intersection is 1, add to solved + remove from all maps
        //
        for allergen in allergen_to_ingredients.keys() {
            if solved.contains_key(allergen) {
                continue;
            }
            let mut intersection: HashSet<String> = HashSet::new();
            for (food, allergens) in food_to_allergens.iter() {
                if !allergens.contains(allergen) {
                    continue;
                }

                let mut ingredients = food_to_ingredients.get_mut(food).unwrap().clone();
                ingredients.retain(|e| !solved_ingredients.contains(e));

                if intersection.len() == 0 {
                    intersection = ingredients;
                } else {
                    intersection = intersection
                        .intersection(&ingredients)
                        .map(|x| x.to_string())
                        .collect();
                }
            }
            if intersection.len() == 1 {
                let only_ingredient = intersection.iter().next().unwrap().clone();
                solved_ingredients.insert(only_ingredient.clone());
                solved.insert(
                    allergen.clone(),
                    only_ingredient.clone(),
                );
            }
            println!("allergen: {}, intersection: {:?}", allergen, intersection);
        }
    }
    // sort the allergen keys
    let mut answer: Vec<String> = Vec::new();
    let mut keys: Vec<&String> = solved.keys().collect();
    keys.sort();
    for key in keys {
        answer.push(solved.get(key).unwrap().clone());
    }


    println!("part2: {}", answer.join(","));
}
