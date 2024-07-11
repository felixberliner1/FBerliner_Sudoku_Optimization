//Felix Berliner

use std::{collections::HashMap, io::LineWriter};

pub type input_relations = HashMap<(String, Vec<usize>), bool>;

pub type dimacs_conversion_table = HashMap<(String, Vec<usize>), u16>;

use cnf_builder;

use varisat::Lit;
use varisat::Solver; 

/*
    This function takes a cnf as input and creates a hashmap that assigns each unique member of a relation a number for a DIMACS string.
    It also outputs the number of literals in the cnf and then the number of clauses in the cnf in that order. 

    If you want to use these values to create a DIMACS string, call the create_dimacs_string function with the input cnf for this function as well as it's corresponding 3 outputs in order.
*/

pub fn create_conversion_table (base_cnf : &cnf_builder::rel_cnf) -> (dimacs_conversion_table, u16, usize) {
    let mut conv_table : dimacs_conversion_table = HashMap::new();
    
    let mut lit_count = 1;

    let clause_count = base_cnf.len();

    for i in 0..clause_count {
        for x in base_cnf[i].iter() {
            match x {
                cnf_builder::rel_literal::Pos(name, vars) => {
                    let new_lit = (name.clone(), vars.clone());
                    if conv_table.get(&new_lit) == None {
                        conv_table.insert(new_lit, lit_count);
                        lit_count += 1;
                    }
                }
                cnf_builder::rel_literal::Neg(name, vars) => {
                    let new_lit = (name.clone(), vars.clone());
                    if conv_table.get(&new_lit) == None {
                        conv_table.insert(new_lit, lit_count);
                        lit_count += 1;
                    }
                }
            }
        }
    }
    (conv_table, lit_count - 1, clause_count)
}

/* 
    This function takes in a cnf, along with it's conversion table, literal count, and clause count, and then creates a DIMACS string that Varisat can solve.
*/

pub fn create_dimacs_string (base_cnf : cnf_builder::rel_cnf, conv_table : &dimacs_conversion_table, lit_count : u16, clause_count : usize) -> String{
    let mut dimacs_string = String::from("p cnf ");
    dimacs_string.push_str(&format!("{} {}\n", lit_count.to_string(), clause_count.to_string()));

    for i in 0..clause_count {
        for x in base_cnf[i].iter() {
            match x {
                cnf_builder::rel_literal::Pos(name, vars) => {
                    let curr_lit = (name.clone(), vars.clone());
                    match conv_table.get(&curr_lit) {
                        None => {
                            println!("Error: literal missing from conversion table.");
                        }
                        Some (&y) => {
                            dimacs_string.push_str(&format!("{} ", y));
                        }
                    }
                }
                cnf_builder::rel_literal::Neg(name, vars) => {
                    let curr_lit = (name.clone(), vars.clone());
                    match conv_table.get(&curr_lit) {
                        None => {
                            println!("Error: literal missing from conversion table.");
                        }
                        Some (&y) => {
                            dimacs_string.push_str(&format!("-{} ", y));
                        }
                    }
                }   
            }
        }
        dimacs_string.push_str(&format!("0\n"));
    }

    dimacs_string
}

/* 
    This function takes in a String in DIMACS format (created with the above functions) and then uses Varisat to solve that string. 
    If the function returns a None then the CNF is unsatisfiable, and if it returns a Some, the associated vector is the positive variables in the satisfying assignment it found (in Varisat Lit form).
*/

pub fn solve_dimacs (dimacs_string : String) -> Option<Vec<Lit>> {
    let mut solver : Solver = Solver::new();
    solver.add_dimacs_cnf(dimacs_string.as_bytes()).expect("Parse Error");
    let solution = solver.solve().unwrap();

    if solution {
        Some(solver.model().unwrap())
    } else {
        None
    }
}

/*
    This function takes a Varisat model for a satisfying assignment (the Some output from the above function), as well as that model's associated conversion table (from the first function) as input. 
    The function then converts those literals using the conversion table, adds the determined relations from the Structure's relation hashmap, and adds them together into a single Vector containing all the relations for a satisfying assignment for the CNF.
*/

pub fn convert_to_readable (varisat_output : Vec<Lit>, conv_table : dimacs_conversion_table, base_relations : HashMap<String, Vec<Vec<usize>>>) -> Vec<(String, Vec<usize>)> {
    let mut return_clause : Vec<(String, Vec<usize>)> = Vec::new();

    for ((name,vars),v) in conv_table {
        if varisat_output.contains(&Lit::from_dimacs(v.try_into().unwrap())) {
            return_clause.push((name,vars));
        }
    }

    for (symbol, elements) in base_relations {
        for i in 0..elements.len() {
            return_clause.push((symbol.clone(), elements[i].clone()));
        }
    }

    return_clause
}



#[cfg(test)]
mod tests {
    /*use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }*/
}
