extern crate permutohedron;
extern crate time;

pub fn solve(matrix: &mut Vec<Vec<i32>>,
             iterations: i32,
             population_size: i32,
             children_pairs_size: i32,
             mutation_probability: f32,
             max_time_in_seconds: i32) {

    // Początkowe rozwiązanie ma maksymalną wartość
    let mut best_solution: i32 = <i32>::max_value();
    // Liczba miast
    let mut number_of_cities: i32 = matrix.len() as i32;
    // Tablica zawierająca uszeregowaną listę wszystkich wierzchołków w postaci tablicy
    let mut nodes: Vec<i32> = (0..number_of_cities).collect();
    // Populacja jest dwuwymiarową tablicą permutacji
    let mut population: Vec<Vec<i32>> = create_starting_population(&number_of_cities, &population_size, &nodes);
}

fn create_starting_population(number_of_cities: &i32,
                              population_size: &i32,
                              nodes: &Vec<i32>) -> Vec<Vec<i32>> {
    println!("Generowanie populacji początkowej, rozmiar: {}", &number_of_cities);

    let mut population: Vec<Vec<i32>> = Vec::new();

    return population;
}

fn swap_elements_in_population(population: Vec<i32>,
                               first_element_index: i32,
                               second_element_index: i32) -> Vec<i32> {

    // Kocham ten język
    let first_element_index: usize = first_element_index as usize;
    let second_element_index: usize = second_element_index as usize;
    // Nowa populacja, będąca klonem starej
    let mut new_population: Vec<i32> = population.clone();
    // Swap elementów w nowej populacji
    let mut saved_element: i32 = population[first_element_index];
    new_population[first_element_index] = population[second_element_index];
    new_population[second_element_index] = saved_element;
    // Populacja po zamianie zwracana jako wynik
    return new_population;
}