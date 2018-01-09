extern crate time;
extern crate rand;

use self::rand::Rng as random_number_generator;

// Podstawowa metoda zawierająca całość algorytmu
pub fn solve(
    matrix: &mut Vec<Vec<i32>>,
    iterations: i32,
    population_size: i32,
    children_pairs_size: i32,
    mutation_probability: f32,
    max_time_in_seconds: i32,
) {

    // Początkowe rozwiązanie ma maksymalną wartość
    let mut best_solution: i32 = <i32>::max_value();

    // Liczba miast
    let mut number_of_cities: i32 = matrix.len() as i32;

    // Tablica zawierająca uszeregowaną listę wszystkich wierzchołków w postaci tablicy
    let mut nodes: Vec<i32> = (0..number_of_cities).collect();

    // Populacja jest dwuwymiarową tablicą permutacji
    // Populacja startowa jest generowana losowo
    let mut population: Vec<Vec<i32>> =
        create_starting_population(&number_of_cities, &population_size, &nodes);

    // Podbnie zbiór rodziców jest tablicą dwuwymiarową osobników wybranych z populacji
    // Zakładamy, że jest on połową całej populacji
    let mut parents_population_size: i32 = population_size / 2;
    let mut parents_population: Vec<Vec<i32>> = Vec::new();

    // Kolejny zbiór będzie przechowywał dzieci otrzymane w wyniku mutacji
    // Oraz krzyżowania osobników populacji
    let mut children_population: Vec<Vec<i32>> = Vec::new();

    // Pętla wykonująca całość algorytmu
    // Napisana wg kroków podanych na wikipedii XD
    for iteration in 0..iterations {
        // Określenie populacji, która będzie rodzicami dla kolejnego pokolenia
        parents_population = find_parents_in_population(
            &population,
            &population_size,
            &parents_population_size,
            &matrix,
        );

        // Wyznaczenie dzieci jako populacji tworzonej z populacji rodziców
        for i in 0..children_pairs_size {
            let children_pair = generate_children_pair(&parents_population, mutation_probability);
            children_population.push(children_pair[0].clone());
            children_population.push(children_pair[1].clone());
        }

        println!(
            "Populacja dzieci ma rozmiar: {}",
            &children_population.len()
        );

        // Wyznaczenie nowej populacji, wybierając najlepsze osobniki z populacji dzieci i rodziców
        population = regenerate_population(
            &matrix,
            &population,
            population_size as usize,
            &children_population,
        );
        // Wyczyszczenie populacji dzieci
        children_population = Vec::new();
        println!(
            "Finałowa populacja w danej iteracji ma rozmiar: {}",
            &population.len()
        );
    }
}

// Funckja generująca w losowy sposób populację początkową
// Nie korzystam z algorytmu zachłannego
// Lubię jak jest losowo
fn create_starting_population(
    number_of_cities: &i32,
    population_size: &i32,
    nodes: &Vec<i32>,
) -> Vec<Vec<i32>> {
    println!(
        "Generowanie populacji początkowej, rozmiar: {}",
        &population_size
    );

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

// Metoda generuje nową populację
// Używana jest po wygenerowaniu kolejnego pokolenia
fn regenerate_population(
    matrix: &Vec<Vec<i32>>,
    population: &Vec<Vec<i32>>,
    population_size: usize,
    population_children: &Vec<Vec<i32>>,
) -> Vec<Vec<i32>> {

    // Zmienna będzie przechowywała elementy nowej populacji
    let mut new_population: Vec<Vec<i32>> = Vec::new();

    // Zmienne przechowujące aktualne stany populacji wejściowej i jej dzieci
    let mut current_population: Vec<Vec<i32>> = population.clone();
    let mut current_population_children: Vec<Vec<i32>> = population_children.clone();

    // Populacje wejściowe należy posortować wg wartości funkcji przystosowania
    current_population.sort_by(|x, y| {
        permutation_evaluation_value(matrix, x).cmp(&permutation_evaluation_value(matrix, y))
    });

    current_population_children.sort_by(|x, y| {
        permutation_evaluation_value(matrix, x).cmp(&permutation_evaluation_value(matrix, y))
    });


    // Iteracja po całym docelowym rozmiarze populacji
    for i in 0..population_size {

        // Jeżeli pusty jest zbiór dzieci, ale zbiór rodziców ma jeszcze elementy
        // Dodajemy do nowej populacji alement zbioru rodziców
        if current_population_children.is_empty() && !current_population.is_empty() {
            new_population.push(current_population[current_population.len() - 1].clone());
            current_population.pop();
            continue;
        }

        // Jeżeli pusty jest zbiór rodziców, ale zbiór dzieci ma jescze elementy
        // Dodajemy do nowej populacji alement zbioru dzieci
        if !current_population_children.is_empty() && current_population.is_empty() {
            new_population.push(
                current_population_children[current_population_children.len() - 1].clone(),
            );
            current_population_children.pop();
            continue;
        }

        // Jeżeli obie populacje zawierają jeszcze elementy
        // Wybieramy ten, o korzystniejszej wartości funkcji przystosowania
        // Można sprawdzać po indeksach tablicy, bo wcześniej je sortowaliśmy
        if permutation_evaluation_value(
            matrix,
            &current_population_children[current_population_children.len() - 1],
        ) <
            permutation_evaluation_value(
                matrix,
                &current_population[current_population.len() - 1],
            )
            {
            new_population.push(current_population[current_population.len() - 1].clone());
            current_population.pop();
        } else {
            new_population.push(
                current_population_children[current_population_children.len() - 1]
                    .clone(),
            );
            current_population_children.pop();
        }
    }

    // Zwracamy nową populację wygenerowaną z dzieci i rodziców
    return new_population;
}

// Funkcja wybierająca rodziców spośród populacji
// Przy użyciu kryterium celu i funkcji ewaluacji wartości osobników
fn find_parents_in_population(
    population: &Vec<Vec<i32>>,
    population_size: &i32,
    parents_population_size: &i32,
    matrix: &Vec<Vec<i32>>,
) -> Vec<Vec<i32>> {
    println!(
        "Wyszukiwanie rodziców w populacji o rozmiarze {}",
        &population_size
    );
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
        permutation_evaluation_values.push(permutation_evaluation_value(
            &matrix,
            &population[i as usize],
        ));
        permutations_evaluation_sum = permutations_evaluation_sum +
            permutation_evaluation_values[i as usize] as i64;
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
                     permutation: &Vec<i32>
) -> i32 {

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
                                permutation: &Vec<i32>
) -> i32 {

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
fn generate_children_pair(
    parents_population: &Vec<Vec<i32>>,
    mutation_probability: f32,
) -> Vec<Vec<i32>> {
    // Tablica przechowująca dwie permutacje, odpowiadające
    // Parze dzieci (osobników kolejnej populacji)
    let mut children_pair: Vec<Vec<i32>> = Vec::new();
    // Losowy wybór osobników z populacji pierwotnej
    // Będą oni rodzicami pary osobników nowej populacji
    let mut parents_pair: Vec<Vec<i32>> = generate_parents_pair(&parents_population);
    println!("Ojciec: {:?}", &parents_pair[0]);
    println!("Matka: {:?}", &parents_pair[1]);
    // Następnie następuje krzyżowanie osobników
    children_pair.push(cross_single_child_pmx(&parents_pair));
    children_pair.push(cross_single_child_pmx(&parents_pair));
    // I próba mutacji otrzymanych dzieci
    children_pair[0] = attempt_child_mutation(children_pair[0].clone(), mutation_probability);
    children_pair[1] = attempt_child_mutation(children_pair[1].clone(), mutation_probability);
    println!("  Syn: {:?}", &children_pair[0]);
    println!("  Córka: {:?}", &children_pair[1]);

    return children_pair;
}

// Zwraca parę dzieci po krzyżowaniu
// Metodą Partially Matched Cross (PMX)
fn cross_single_child_pmx(parents_pair: &Vec<Vec<i32>>
) -> Vec<i32> {
    let child_size: usize = parents_pair[0].len() as usize;
    // Nowa para dzieci
    let mut child: Vec<i32> = vec![0; child_size];
    let mut swapped: Vec<i32> = vec![0; child_size];
    // Obliczenie ilości elementów w permutacji
    // Wyznaczenie dwóch punktów krzyżowania
    let mut first_cross_point: usize = 0;
    let mut second_cross_point: usize = 0;
    // Pętla zapobiegająca wylosowaniu dwóch takich samych punktów krzyżowania
    while first_cross_point == second_cross_point {
        first_cross_point = rand::thread_rng().gen_range(0, child_size);
        second_cross_point = rand::thread_rng().gen_range(0, child_size);
    }

    // Sortowanie punktów krzyżowania
    // Jeżeli pierwszy nastepuje po drugim
    // Należy zamienić je miejscami
    if first_cross_point > second_cross_point {
        let temp_cross_point: usize = second_cross_point.clone();
        second_cross_point = first_cross_point.clone();
        first_cross_point = temp_cross_point;
    }

    // Pierwsza pętla krzyżująca metodą PMX
    // Zamienia elementy dzieci w zakresie punktów krzyżowania
    for i in (first_cross_point)..(second_cross_point) {
        child[i] = parents_pair[0][i].clone();
        swapped[parents_pair[0][i] as usize] = 1;
    }

    // Druga pętla krzyżująca metodą PMX
    // Zamienia pozostałe elementy, aby uniknąć niespójnych permutacji
    let mut swap_index = 0;
    for i in 0..child_size {
        if swapped[parents_pair[1][i] as usize] != 1 {
            if swap_index == first_cross_point {
                swap_index = second_cross_point.clone();
            }

            child[swap_index] = parents_pair[1][i].clone();
            swap_index = swap_index + 1;
        }
    }

    return child;
}

// Funkcja generuje losową parę rodziców z populacji
// Zapobiega wylosowaniu dwukrotnie tego samego rodzica
fn generate_parents_pair(parents_population: &Vec<Vec<i32>>
) -> Vec<Vec<i32>> {
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

    return parents_pair;
}

// Metoda zamienia dwa wybrane elementy w populacji
// Zwraca permutację, jako wektor z zamienionymi elementami
fn swap_elements_in_permutation(
    permutation: &Vec<i32>,
    first_element_index: usize,
    second_element_index: usize,
) -> Vec<i32> {

    // Nowa populacja, będąca klonem starej
    let mut new_population: Vec<i32> = permutation.clone();
    // Swap elementów w nowej populacji
    let mut saved_element: i32 = permutation[first_element_index];
    new_population[first_element_index] = permutation[second_element_index];
    new_population[second_element_index] = saved_element;
    // Populacja po zamianie zwracana jako wynik
    return new_population;
}

// Metoda wykonuje mutację danego osobnika
// Zamieniając elementu, jeżeli spełniony zostanie
// Warunek określony przez prawdopodobieństwo
fn attempt_child_mutation(permutation: Vec<i32>,
                          mutation_probability: f32
) -> Vec<i32> {

    // Losowa zmienna z zakresu 0..1
    let random_float: f32 = rand::thread_rng().next_f32();

    // Sprawdzenie czy wylosowana liczba mieści się w zakresie podobieństwa
    // Określonym przez użytkownika
    if random_float <= mutation_probability {
        println!("Nastąpiła mutacja dziecka {:?}", &permutation);

        // Zmienne przechowujące indeksy elementów do zamiany
        let mut first_element_index: usize = 0;
        let mut second_element_index: usize = 0;
        // Losowanie elementów do momentu otrzymania dwóch róznych
        // Zapobiega niepoprawnej mutacji
        while first_element_index == second_element_index {
            first_element_index = rand::thread_rng().gen_range(0, permutation.len());
            second_element_index = rand::thread_rng().gen_range(0, permutation.len());
        }

        // Zwracana jest permutacja po zamianie elementów
        // Na określonych wcześniej indeksach
        return swap_elements_in_permutation(
            &permutation,
            first_element_index,
            second_element_index,
        );
    } else {
        // Jeżeli warunek prawdopodobieństwa nie został spełniony
        // Zwracana jest oryginalnie sprawdzana permutacja
        return permutation;
    }
}

// Funkcja generująca losową wartość celu
// Wykorzystywana przy wyborze rodziców do kolejnej populacji
fn generate_randomized_target_value(permutation_evaluation_sum: &i64
) -> i64 {

    // Losowy float w zakresie 0..1
    let random_float: f64 = rand::thread_rng().next_f64();
    // Obliczenie kryterium celu jako float
    let target_as_float: f64 = random_float * (permutation_evaluation_sum.clone() as f64);
    // Konwersja kryterium celu na i64
    let target: i64 = target_as_float as i64;
    // Zwrot wyliczonego kryterium celu
    return target;
}
