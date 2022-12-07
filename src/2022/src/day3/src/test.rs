fn main() {
    let input = include_str!("./test.txt");

    let rucksack = input.split("\n")
        .map(|row|
            row.chars().collect::<Vec<char>>()
        ).collect::<Vec<Vec<char>>>();
    
    println!("rucksack: {:?}", &rucksack);
    
    let cool1 = rucksack.iter()
        .step_by(2)
        .collect::<Vec<_>>()
        .iter()
        .map(|x| *x)
        .collect::<Vec<_>>();
    
    println!("cool1: {:?}", &cool1);
    
    let cool2 = rucksack.iter()
        .skip(1)
        .step_by(2)
        .collect::<Vec<_>>()
        .iter()
        .map(|x| *x)
        .collect::<Vec<_>>();
    
    println!("cool2: {:?}", &cool2);
    
    let cool = cool1.iter().zip(cool2.iter())
        .map(|pair|
            (*pair.0, *pair.1)
        )
        .collect::<Vec<(&Vec<char>, &Vec<char>)>>();
    
    println!("cool: {:?}", &cool);
}