extern crate utilities;
use std::collections::{HashMap, HashSet};

fn main() {
    let caves: HashMap<String, Vec<String>> = utilities::lines_from_file("input.txt").unwrap()
        .into_iter()
        .map(|x| x.split('-')
            .map(|s| s.to_string())
            .collect::<Vec<String>>())
        .fold(HashMap::<String, Vec<String>>::new(), |mut map, v| {
            insert_path_nodes(&mut map, &v[0], &v[1]);
            insert_path_nodes(&mut map, &v[1], &v[0]);
            map
        });
    
    let p1 = count_paths(&caves, "start", "end", 
        &|child, path| !path.contains(&child) || (path.contains(&child) && !utilities::is_all_lowercase(&child)) );
    println!("Part one solution: {}", p1);

    let p2 = count_paths(&caves, "start", "end", 
        &|child, path| {
            child != "start" 
            && (!path.contains(&child) 
                || (path.contains(&child) && !utilities::is_all_lowercase(&child))
                || (path.contains(&child) && !contains_duplicated_lowercase(path)))
        } );
    println!("Part two solution: {}", p2);
}

fn count_paths<F>(map: &HashMap<String, Vec<String>>, start: &str, end: &str, predicate: &F) -> i32
where F: Fn(&String, &Vec<String>) -> bool
{
    let children = map.get(start).unwrap();
    let mut total_paths = 0;
    for child in children {
        let path = vec![start.to_string()];
        total_paths += recurse_path(map, child, end, path, predicate);
    }
    total_paths
}

fn recurse_path<F>(map: &HashMap<String, Vec<String>>, current: &str, end: &str, mut path: Vec<String>, predicate: &F) -> i32
where F: Fn(&String, &Vec<String>) -> bool
{    
    let children = map.get(current).unwrap();
    path.push(current.to_string());
    let mut total_paths = 0;
    for child in children {
        if child == end { 
            total_paths += 1;
        } else if predicate(child, &path) {
            let p = path.clone();   
            total_paths += recurse_path(map, &child, end, p, predicate);
        }
    }
    total_paths
}

fn insert_path_nodes(map: &mut HashMap<String, Vec<String>>, parent: &str, child: &str) {
    if map.contains_key(parent) {
        map.get_mut(parent).unwrap().push(child.to_string());
    } else {
        map.insert(parent.to_string(), vec![child.to_string()]);
    }
}

fn contains_duplicated_lowercase(path: &Vec<String>) -> bool {
    let mut set: HashSet<&String> = HashSet::new();
    path.iter()
        .filter(|x| utilities::is_all_lowercase(x))
        .fold(false, |result, x| result | !set.insert(x))
}