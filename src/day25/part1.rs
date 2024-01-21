use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;


pub fn main(filename: &str) -> String {
    let mut file = File::open(filename).expect("Failed to open file");
	let mut file_content: String = String::new();

	file.read_to_string(&mut file_content).expect("Failed to read file content");

    let wiring_diagram: Vec<(String, Vec<String>)> = file_content
        .lines()
        .map(|line| {
            let (node, connections) = line.split_once(": ").unwrap();
            (node.to_string(), connections.split(" ").map(String::from).collect::<Vec<String>>())
        })
        .collect();

    solve(&wiring_diagram).to_string()
}


fn solve(wiring_diagram: &Vec<(String, Vec<String>)>) -> usize {
    let wiring_diagram_formatted: HashMap<String, HashSet<String>> = get_wiring_diagram_formatted(&wiring_diagram);
    let unique_connections: Vec<(String, String)> = get_unique_connections(&wiring_diagram);

    for i in 0..unique_connections.len() {
        println!("i: {i}");
        for j in i+1..unique_connections.len() {
            println!("j: {j}");
            for k in j+1..unique_connections.len() {
                let separated = get_number_of_components_in_separated_groups(
                    &wiring_diagram_formatted, 
                    vec![
                        unique_connections[i].clone(),
                        unique_connections[j].clone(),
                        unique_connections[k].clone()
                    ]
                );

                if separated != 0 {
                    return separated;
                }
            }
        }
    }

    0
}


fn get_wiring_diagram_formatted(wiring_diagram: &Vec<(String, Vec<String>)>) -> HashMap<String, HashSet<String>> {
    let mut wiring_diagram_formated: HashMap<String, HashSet<String>> = HashMap::new();

    for (wire, connections) in wiring_diagram {
        for connection in connections {
            wiring_diagram_formated.entry(wire.clone()).or_insert(HashSet::new()).insert(connection.clone());
            wiring_diagram_formated.entry(connection.clone()).or_insert(HashSet::new()).insert(wire.clone());
        }
    }

    wiring_diagram_formated
}


fn get_unique_connections(wiring_diagram: &Vec<(String, Vec<String>)>) -> Vec<(String, String)> {
    let mut wiring_diagram_formated: Vec<(String, String)> = Vec::new();

    for (wire, connections) in wiring_diagram {
        for connection in connections {
            wiring_diagram_formated.push((wire.clone(), connection.clone()));
        }
    }

    wiring_diagram_formated
}


fn get_number_of_components_in_separated_groups(
    wiring_diagram: &HashMap<String, HashSet<String>>, 
    wires_removed: Vec<(String, String)>
) -> usize {
    let mut visited: HashSet<String> = HashSet::new();

    let mut stack: Vec<String> = Vec::new();

    for (wire, connections) in wiring_diagram {
        for connection in connections {
            if wires_removed.contains(&(wire.clone(), connection.clone())) || wires_removed.contains(&(connection.clone(), wire.clone())) {
                continue;
            }
            stack.push(connection.clone());
        }

        if !stack.is_empty() {
            visited.insert(wire.clone());
            break
        }
    }

    while !stack.is_empty() {
        let wire: String = stack.pop().unwrap();

        for wire_to in wiring_diagram.get(&wire).unwrap() {
            if visited.contains(wire_to) {
                continue;
            }

            if wires_removed.contains(&(wire.clone(), wire_to.clone())) || wires_removed.contains(&(wire_to.clone(), wire.clone())) {
                continue;
            }

            visited.insert(wire_to.clone());

            stack.push(wire_to.clone());
        }
    }

    let components: usize = visited.len();

    if components >= wiring_diagram.len() - 1 {
        return 0;
    }

    components * (wiring_diagram.len() - components)
}