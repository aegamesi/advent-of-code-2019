use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;


fn read_lines(filename: &str) -> impl Iterator<Item=String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|line| line.unwrap())
}

struct Chemical {
    quantity: i64,
    name: String,
}

impl Chemical {
    fn parse(str: &str) -> Chemical {
        let parts: Vec<&str> = str.split(" ").collect();
        let quantity: i64 = parts[0].parse().unwrap();
        Chemical {
            quantity,
            name: parts[1].to_string(),
        }
    }
}

struct Reaction {
    input: Vec<Chemical>,
    output: Chemical
}

fn parse_reaction(s: String) -> Reaction {
    let mut chems: Vec<Chemical> = s.split(" => ").flat_map(|x| x.split(", ")).map(Chemical::parse).collect();
    let to = chems.pop().unwrap();
    Reaction {
        input: chems,
        output: to,
    }
}

fn process(reactions: &HashMap<String, Reaction>, haves: &mut HashMap<String, i64>) {
    loop {
        let (product, amount) = {
            let needed = haves.iter().filter(|(product, amount)| **amount < 0 && *product != "ORE").next();
            if needed.is_none() {
                break;
            }
            needed.unwrap()
        };
        let needed = -*amount;
        let product = product.to_string();
        // println!("{} {}:", needed, product);

        let reaction = reactions.get(&product).unwrap();
        let multiply = ((needed as f32) / (reaction.output.quantity as f32)).ceil() as i64;

        for input in &reaction.input {
            let stock = *haves.get(&input.name).unwrap_or(&0);
            // println!(" with {} {} (have {})", input.quantity * multiply, &input.name, stock);
            haves.insert(input.name.to_string(), stock - (input.quantity * multiply));
        }

        let produced = multiply * reaction.output.quantity;
        haves.insert(product, produced - needed);
    }
}

fn main() {
    let lines = read_lines("input.in");
    let reactions: HashMap<String, Reaction> = lines.map(parse_reaction).map(|x| (x.output.name.to_string(), x)).collect();

    // Part 1.
    let mut haves: HashMap<String, i64> = HashMap::new();
    haves.insert("ORE".to_string(), 0);
    haves.insert("FUEL".to_string(), -1);
    process(&reactions, &mut haves);
    println!("Part 1.    ORE Required: {}", -*haves.get("ORE").unwrap());

    // Part 2.
    let mut fuel = 0;
    let mut haves: HashMap<String, i64> = HashMap::new();
    haves.insert("ORE".to_string(), 0);
    loop {
        haves.insert("FUEL".to_string(), -1);
        process(&reactions, &mut haves);

        let ore = -*haves.get("ORE").unwrap();
        if ore < 1_000_000_000_000 {
            fuel += 1;
        } else {
            break;
        }
    }
    println!("Part 2 Fuel Generated: {}", fuel);
}
