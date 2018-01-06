extern crate time;
extern crate rand;

use self::rand::Rng as random_number_generator;

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
    // Populacja startowa jest generowana losowo
    let mut population: Vec<Vec<i32>> = create_starting_population(&number_of_cities, &population_size, &nodes);
    // Podbnie zbiór rodziców jest tablicą dwuwymiarową osobników wybranych z populacji
    let mut parents: Vec<Vec<i32>> = Vec::new();

}

fn create_starting_population(number_of_cities: &i32,
                              population_size: &i32,
                              nodes: &Vec<i32>) -> Vec<Vec<i32>> {
    println!("Generowanie populacji początkowej, rozmiar: {}", &number_of_cities);

    // Tablica dwuwymiarowa przechowująca gotową populację
    let mut population: Vec<Vec<i32>> = Vec::new();
    // Pojedyncza permutacja wierzchołków, odpowiadająca elementowi populacji
    let mut single_population_element: Vec<i32> = Vec::new();

    // Wygenerowanie losowej populacji
    // Ogólnie, powinno tutaj być sprawdzenie czy dany element nie istnieje już w populacji
    // Ale mam to w dupie, bo sam ma siostry bliźniaczki
    // Więc mój algorytm dopuszcza opcję dwóch takich samych elementów w populacji
    for i in 0..(population_size.clone()) {
        single_population_element = nodes.clone();
        rand::thread_rng().shuffle(&mut single_population_element);
        population.push(single_population_element);
    }

    // Zwrot gotowej populacji startowej
    return population;
}

fn find_parents_in_population() {

    // Suma całkowita wszystkich wartości permutacji
    // Uwaga, spora liczba może tu wyjść
    let mut permutations_sum: i64;
    // Tablica przechowująca wartości wszystkich permutacji
    let mut permutation_values: Vec<i32> = Vec::new();

    //TODO: Dokończyć
}

// Funkcja obliczająca koszt ścieżki
fn permutation_value(matrix: &Vec<Vec<i32>>,
                     permutation: &Vec<i32>) -> i32 {

    // Początkowy koszt ścieżki to 0
    let mut value: i32 = 0;
    // Pierwszy wierzchołek
    let mut previous_node: usize = 0;
    // Iteracja po wszystkich kolejnych wierzchołkach
    for i in 0..(permutation.len() as i32) {
        // Zwiększenie kosztu
        value = value + matrix[previous_node][(permutation[(i as usize)] as usize)];
        // Przypisanie aktualnego wierzchołka jako poprzedniego
        previous_node = permutation[(i as usize)] as usize;
    }
    // Zwiększenie kosztu trasy o koszt powrotu do wierzchołka początkowego
    value = value + matrix[previous_node][0];
    // Zwrot obliczonego kosztu trasy
    return value;
}

// Funkcja wyliczająca wartość funkcji przystosowania dla wybranego elementu populacji
fn permutation_evaluation_value(matrix: &Vec<Vec<i32>>,
                                permutation: &Vec<i32>) -> i32 {

    // Koszt ścieżki zawartej w danej permutacji
    let mut path_value: i32 = permutation_value(&matrix, &permutation);
    // Maksymalna wartość kosztu
    let mut max_path_value: i32 = <i32>::max_value();
    // Zwracana wartość jest różnicą pomiędzy maksimum a otrzymanym kosztem
    // Im większa wartość, tym lepszy wynik funkcji przystosowania
    return max_path_value - path_value;
}

// Metoda zamienia dwa wybrane elementy w populacji
// Zwraca permutację, jako wektor z zamienionymi elementami
fn swap_elements_in_permutation(permutation: Vec<i32>,
                                first_element_index: i32,
                                second_element_index: i32) -> Vec<i32> {

    // Cast jest potrzebny bo nie można enumerować po i32
    // Kocham ten język
    let first_element_index: usize = first_element_index as usize;
    let second_element_index: usize = second_element_index as usize;
    // Nowa populacja, będąca klonem starej
    let mut new_population: Vec<i32> = permutation.clone();
    // Swap elementów w nowej populacji
    let mut saved_element: i32 = permutation[first_element_index];
    new_population[first_element_index] = permutation[second_element_index];
    new_population[second_element_index] = saved_element;
    // Populacja po zamianie zwracana jako wynik
    return new_population;
}