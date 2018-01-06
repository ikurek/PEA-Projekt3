extern crate time;
extern crate rand;

use self::rand::Rng as random_number_generator;

// Podstawowa metoda zawierająca całość algorytmu
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
    let mut population: Vec<Vec<i32>> = create_starting_population(&number_of_cities,
                                                                   &population_size,
                                                                   &nodes);
    // Podbnie zbiór rodziców jest tablicą dwuwymiarową osobników wybranych z populacji
    // Zakłądamy, że jest on połową całej populacji
    let mut parents_population_size: i32 = population_size / 2;
    let mut parents_population: Vec<Vec<i32>> = Vec::new();

    // Pętla wykonująca całość algorytmu
    // Napisana wg kroków podanych na wikipedii XD
    for iteration in 0..iterations {
        // Określenie populacji, która będzie rodzicami dla kolejnego pokolenia
        parents_population = find_parents_in_population(&population,
                                                        &population_size,
                                                        &parents_population_size,
                                                        &matrix);

        for i in 0..children_pairs_size {
            generate_children_pair(&parents_population);
        }

    }
}

// Funckja generująca w losowy sposób populację początkową
// Nie korzystam z algorytmu zachłannego
// Lubię jak jest losowo
fn create_starting_population(number_of_cities: &i32,
                              population_size: &i32,
                              nodes: &Vec<i32>) -> Vec<Vec<i32>> {
    println!("Generowanie populacji początkowej, rozmiar: {}", &population_size);

    // Tablica dwuwymiarowa przechowująca gotową populację
    let mut population: Vec<Vec<i32>> = Vec::new();
    // Pojedyncza permutacja wierzchołków, odpowiadająca elementowi populacji
    let mut single_population_element: Vec<i32> = Vec::new();

    // Wygenerowanie losowej populacji
    // Ogólnie, powinno tutaj być sprawdzenie czy dany element nie istnieje już w populacji
    // Ale mam to w dupie, bo sam mam siostry bliźniaczki
    // Więc mój algorytm dopuszcza opcję dwóch takich samych elementów w populacji
    for i in 0..(population_size.clone()) {
        single_population_element = nodes.clone();
        rand::thread_rng().shuffle(&mut single_population_element);
        population.push(single_population_element);
    }

    // Zwrot gotowej populacji startowej
    return population;
}

// Funkcja wybierająca rodziców spośród populacji
// Przy użyciu kryterium celu i funkcji ewaluacji wartości osobników
fn find_parents_in_population(population: &Vec<Vec<i32>>,
                              population_size: &i32,
                              parents_population_size: &i32,
                              matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    println!("Wyszukiwanie rodziców w populacji o rozmiarze {}", &population_size);
    // Populacja która będzie przechowywać wybranych rodziców
    let mut selected_parents: Vec<Vec<i32>> = Vec::new();
    // Suma całkowita wszystkich wartości funkcji przystosowania populacji
    // Uwaga, spora liczba może tu wyjść
    let mut permutations_evaluation_sum: i64 = 0;
    // Tablica przechowująca wartości funkcji przystosowania wszystkich permutacji
    let mut permutation_evaluation_values: Vec<i32> = Vec::new();
    // Wyliczenie wartości funkcji przystosowania dla wszystkich osobników populacji
    // Zwiększenie sumy całkowitej funkcji przystosowania
    for i in 0..population_size.clone() {
        permutation_evaluation_values.push(permutation_evaluation_value(&matrix, &population[i as usize]));
        permutations_evaluation_sum = permutations_evaluation_sum + permutation_evaluation_values[i as usize] as i64;
    }
    // Zmienna przechowująca losowy współczynnik określający funkcję celu
    let mut random_target_value: i64;

    // Iteracja wybierająca elementy do populacji rodziców
    for i in 0..parents_population_size.clone() {
        // Określenie losowe funkcji celu dla wygenerowanych wartości
        random_target_value = generate_randomized_target_value(&permutations_evaluation_sum);
        // Zmienne przechowujące aktualną i poprzednią wartość
        // Wykorzystywaną przez pętlę sprawdzającą populację początkową
        let mut current_value: i64 = 0;
        let mut previous_value: i64 = 0;
        // Pętla sprawdzająca wszystkie osobniki populacji
        // Wybiera te, które spełniają funkcję celu i umieszcza jes w tablicy rodziców
        for i in 0..population_size.clone() {
            // Zwiększenie aktualnej wartośći o wartość funkcji przystosowania dla aktualnie sprawdzanego osobnika
            current_value = current_value + permutation_evaluation_values[i as usize] as i64;

            // Jeżeli poprzednia i aktualna wartość jest mniejsza od wartości funkcji celu
            // Aktualnie sprawdzany element populacji nadaje się na rodzica
            // W przeciwnym wypadku należy zwiększyć wartość ostatniej funkcji
            // O wartość funkcji ewaluacji ostatniego osobnika
            if (previous_value <= random_target_value) && (random_target_value <= current_value) {
                let permutation_to_add_as_parent: Vec<i32> = population[i as usize].clone();
                selected_parents.push(permutation_to_add_as_parent);
                break;
            } else {
                previous_value = previous_value + permutation_evaluation_values[i as usize] as i64;
            }
        }
    }

    println!("Wybranych rodziców: {}", &selected_parents.len());

    // Zwracana wartość jest tablicą zawierającą osobniki spełniające
    // Kryteria do bycia rodzicem
    return selected_parents;
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

// Funkcja wybiera dwoje osobników z populacji rodziców
// Następnie generuje z nich parę osobników kolejnego pokolenia
fn generate_children_pair(parents_population: &Vec<Vec<i32>>) {
    // Tablica przechowująca dwie permutacje, odpowiadające
    // Parze dzieci (osobników kolejnej populacji)
    let mut children_pair: Vec<Vec<i32>> = Vec::new();
    // Losowy wybór osobników z populacji pierwotnej
    // Będą oni rodzicami pary osobników nowej populacji
    let mut parents_pair: Vec<Vec<i32>> = generate_parents_pair(&parents_population);

    //TODO: Koniec na chwilę
}

// Funkcja generuje losową parę rodziców z populacji
// Zapobiega wylosowaniu dwukrotnie tego samego rodzica
fn generate_parents_pair(parents_population: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut parents_pair: Vec<Vec<i32>> = Vec::new();
    let mut father_index: usize = 0;
    let mut mother_index: usize = 0;
    // Pętla zapobiega wylosowaniu dwa razy tego samego osobnika
    // Ponieważ w takim wypadku algorytm nie ma sensu
    while father_index == mother_index {
        father_index = rand::thread_rng().gen_range(0, parents_population.len()) as usize;
        mother_index = rand::thread_rng().gen_range(0, parents_population.len()) as usize;
    }
    // Po określeniu indeksów osobników, sa one dodawane do tablicy
    parents_pair.push(parents_population[father_index].clone());
    parents_pair.push(parents_population[mother_index].clone());

    println!("Wylosowany Ojciec: {:?}", &parents_pair[0]);
    println!("Wylosowana Matka: {:?}", &parents_pair[1]);

    return parents_pair;
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

// Funkcja generująca losową wartość celu
// Wykorzystywana przy wyborze rodziców do kolejnej populacji
fn generate_randomized_target_value(permutation_evaluation_sum: &i64) -> i64 {

    // Losowy float w zakresie 0..1
    let random_float: f64 = rand::thread_rng().next_f64();
    // Obliczenie kryterium celu jako float
    let target_as_float: f64 = random_float * (permutation_evaluation_sum.clone() as f64);
    // Konwersja kryterium celu na i64
    let target: i64 = target_as_float as i64;
    // Zwrot wyliczonego kryterium celu
    return target;
}