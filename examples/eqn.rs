use genx::{
    crossover::simulated_binary_crossover,
    mutation::polynomial_mutation,
    selection::{random_selection, stochastic_universal_selection},
};
use rand::{distributions::Uniform, prelude::Distribution};

fn main() {
    let equation = |x: f32| 3.0 * x * x + 8.0 * (-x) + 5.0;
    let population_size = 15;
    let iterations = 1000;
    let mutation_probability = 0.5;
    let between = Uniform::from(-10.0..10.0);
    let mut prng = rand::thread_rng();

    let fitness_function = |x: f32| 1.0 / ((equation(x) - x).abs() + 1.0);
    let mut population = (0..population_size)
        .map(|_| between.sample(&mut prng))
        .collect::<Vec<f32>>();

    let get_fitness_values = |population: &Vec<f32>| {
        population
            .iter()
            .map(|x| fitness_function(*x))
            .collect::<Vec<f32>>()
    };

    let best_fitness_value = |population: &Vec<f32>| {
        population
            .iter()
            .max_by(|&a, &b| {
                fitness_function(*a)
                    .partial_cmp(&fitness_function(*b))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap()
            .clone()
    };

    let mut best_now = best_fitness_value(&population);
    let between = Uniform::from(0.0..1.0);
    for _ in 0..iterations {
        let idxs = random_selection(population_size, 2, None);
        let (mut child1, mut child2) =
            simulated_binary_crossover(population[idxs[0]], population[idxs[1]], 5.0, None);

        if between.sample(&mut prng) < mutation_probability {
            child1 = polynomial_mutation(child1, 100.0, 10.0, None)
        }
        if between.sample(&mut prng) < mutation_probability {
            child2 = polynomial_mutation(child2, 100.0, 10.0, None)
        }

        population.push(child1);
        population.push(child2);

        let fitness_values = get_fitness_values(&population);
        let selected_idx = stochastic_universal_selection(&fitness_values, population_size, None);
        population = selected_idx
            .iter()
            .map(|&a| population[a])
            .collect::<Vec<f32>>();
        let best = best_fitness_value(&population);
        if fitness_function(best) > fitness_function(best_now) {
            best_now = best;
        }
    }
    println!("{} {}", best_now, equation(best_now));
}
