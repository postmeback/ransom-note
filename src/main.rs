use std::io::stdin;

fn main() {

    let mut iskidnapping: bool = true;

    println!("Write down the words in magazine");

    let mut magazine_words = String::new();

    stdin().read_line(&mut magazine_words).expect("Invalid Input").to_string();

    println!("Write down the ransom note");

    let mut ransom_note = String::new();

    stdin().read_line(&mut ransom_note).expect("Invalid Input").to_string();

    let r_words: Vec<&str> = ransom_note.split_whitespace().collect();

    for word in r_words  {
           if !(magazine_words.contains(word))
           {
            iskidnapping = false;
            break;
           }
    }

    if iskidnapping
    {
        println!( "The kidnapping is going to happen")
    }
    else {
        println!( "The kidnapping is not going to happen")
    }
}
