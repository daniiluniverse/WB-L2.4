// Задача
// Написать функцию поиска всех множеств анаграмм по словарю.
//
//
// Например:
//
// 'пятак', 'пятка' и 'тяпка' — принадлежат одному множеству,
//
// 'листок', 'слиток' и 'столик' — другому.
//
// Требования:
// Входные данные для функции: ссылка на массив, каждый элемент которого — слово на русском языке в кодировке utf8.
//
// Выходные данные: ссылка на мапу множеств анаграмм
//
// Ключ — первое встретившееся в словаре слово из множества. Значение — ссылка на массив, каждый элемент которого — слово из множества.
//
// Массив должен быть отсортирован по возрастанию.
//
// Множества из одного элемента не должны попасть в результат.
//
// Все слова должны быть приведены к нижнему регистру.
//
// В результате каждое слово должно встречаться только один раз.


use std::collections::{HashMap, HashSet};

fn anagrams(words: HashSet<&str>) -> HashMap<String, Vec<String>> {

    // Создаем временную мапу для хранения анаграмм
    let mut anagram_map: HashMap<String, Vec<String>> = HashMap::new();

    for word in words{
         // Приводим слово к нижнему регистру
        let lower_word = word.to_lowercase();
        // Сортируем буквы в слове
        let mut sort_chars: Vec<char> = word.chars().collect();
        sort_chars.sort_unstable();
        let sorted_word = sort_chars.iter().collect();
        // Добавляем слово в соответствующую группу анаграмм
        anagram_map.entry(sorted_word).or_insert(Vec::new()).push(lower_word.clone());
    }

    // Фильтруем множества, содержащие только одно слово, и сортируем каждый массив анаграмм
    let mut sors_anagrams: HashMap<String, Vec<String>> = HashMap::new();

    for (_, mut anagrams) in anagram_map{
        if anagrams.len() > 1{
            // Сортируем массив анаграмм по возрастанию
            anagrams.sort();
            // В качестве ключа берем первое слово из отсортированного массива
            sors_anagrams.insert(anagrams[0].clone(), anagrams);
        }
    }
    sors_anagrams
}


fn main() {
    let words =["пятак", "пятка", "тяпка", "листок", "слиток", "столик", "кот", "ток", "кто", ];

    let set: HashSet<&str> = HashSet::from(words);
    let anagrams = anagrams(set);

    for (key, value) in anagrams{
        println!("{}: {:?}", key, value)
    }


}
