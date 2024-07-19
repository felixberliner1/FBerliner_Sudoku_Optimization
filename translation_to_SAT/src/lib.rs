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
    use crate::cnf_builder::all_pred;
    use crate::cnf_builder::all_in_relation_pred;
    use crate::cnf_builder::pred_funs;
    use crate::cnf_builder::rel_cnf;
    use crate::cnf_builder::rel_literal;
    use crate::solve_dimacs;
    use crate::create_conversion_table;
    use crate::create_dimacs_string;
    use crate::convert_to_readable;
    use crate::HashMap;
    
    fn relations_sudoku() -> Vec<Vec<usize>> {
        vec![vec![0,0,0,0,0,1], vec![0,0,0,1,1,2], vec![0,0,0,2,2,1], 
            vec![0,0,1,1,2,2], vec![0,0,2,2,0,0], vec![0,1,0,2,2,2], vec![0,1,2,1,2,0], vec![0,2,0,2,1,0], vec![1,0,0,0,1,1], 
            vec![1,0,0,2,0,0], vec![1,0,1,0,0,2], vec![1,0,2,0,1,0], vec![1,0,2,2,2,0], vec![1,1,1,0,0,0], vec![1,1,1,2,2,1], 
            vec![1,1,2,0,1,2], vec![1,1,2,2,0,1], vec![1,2,0,1,2,1], vec![1,2,1,0,1,0], vec![1,2,1,2,2,2], vec![1,2,2,0,1,1], 
            vec![2,0,0,1,0,1], vec![2,0,0,2,1,2], vec![2,0,1,0,2,2], vec![2,1,0,0,2,1], vec![2,1,0,1,0,2], vec![2,1,1,1,0,1], 
            vec![2,2,0,1,1,0], vec![2,2,0,2,2,0], vec![2,2,2,0,0,0], vec![2,0,1,2,2,0], vec![1,1,2,1,2,2], vec![0,2,0,0,2,0], 
            vec![1,1,0,1,2,0], vec![0,0,2,0,0,2], vec![2,0,0,0,0,0], vec![1,0,1,1,1,2], vec![2,1,2,1,1,0], vec![2,1,2,2,2,2], 
            vec![0,0,1,0,2,0], vec![0,2,1,2,0,2], vec![2,2,1,0,2,1], vec![0,2,2,1,1,2], vec![1,1,0,0,1,0], vec![2,2,1,2,1,1], 
            vec![0,0,1,2,1,0], vec![0,0,2,1,1,1], vec![0,1,0,0,0,2], vec![0,1,0,1,0,0], vec![0,1,1,0,1,1], vec![0,1,1,1,2,1], 
            vec![0,1,1,2,1,2], vec![0,1,2,0,0,1], vec![0,1,2,2,1,0], vec![0,2,0,1,1,1], vec![0,2,1,0,0,1], vec![0,2,1,1,0,0], 
            vec![0,2,2,0,2,2], vec![0,2,2,2,2,1], vec![1,0,0,1,2,2], vec![1,0,1,2,0,1], vec![1,0,2,1,2,1], vec![1,1,0,2,0,2], 
            vec![1,1,1,1,1,1], vec![1,2,0,0,1,2], vec![1,2,0,2,0,1], vec![1,2,1,1,2,0], vec![1,2,2,1,0,0], vec![1,2,2,2,0,2], 
            vec![2,0,1,1,1,0], vec![2,0,2,0,2,1], vec![2,0,2,1,0,2], vec![2,0,2,2,1,1], vec![2,1,0,2,1,1], vec![2,1,1,0,1,2], 
            vec![2,1,1,2,0,0], vec![2,1,2,0,2,0], vec![2,2,0,0,2,2], vec![2,2,1,1,0,2], vec![2,2,2,1,0,1], vec![2,2,2,2,1,2]]
    }
    
    fn sudoku_x_clues(i : usize) -> structure::Structure {
        use rand::seq::SliceRandom;
        
        let mut rand_relations = Vec::new();
        let base_sudoku = relations_sudoku();
    
        let mut indexes = Vec::new();
        for x in 0..base_sudoku.len() {
            indexes.push(x);
        }
    
        indexes.shuffle(&mut rand::thread_rng());
    
        for x in 0..i {
            rand_relations.push(base_sudoku[indexes[x]].clone());
        }

        println!("{:?}", rand_relations);
    
        structure::Structure {
            description: "sudoku 45 clues".to_string(),
            size: 3,
            constants: std::collections::HashMap::from([]),
            relations: std::collections::HashMap::from([("Cell".to_string(), rand_relations)])
        }
    }
    fn relations_modular() -> Vec<Vec<usize>> {
        vec![vec![0,0,0,2,0,1,0,1,0,1,0,2], vec![0,1,0,1,0,1,0,2,0,1,1,0], vec![0,1,0,2,0,1,1,0,0,2,1,1], vec![0,1,1,0,0,2,1,1,1,0,1,1], 
        vec![0,2,1,1,1,0,1,1,1,1,1,1], vec![1,0,1,1,1,1,1,1,1,1,1,0], vec![1,1,1,1,1,1,1,0,1,1,0,2], vec![1,1,1,0,1,1,0,2,1,2,0,2], 
        vec![1,0,0,2,1,0,0,1,1,0,0,0], vec![1,0,0,1,1,0,0,0,1,1,0,0], vec![1,0,0,0,1,1,0,0,1,2,0,0], vec![1,1,0,0,1,2,0,0,1,2,0,1], 
        vec![2,0,0,0,2,1,0,0,2,1,0,1], vec![2,1,0,0,2,1,0,1,2,2,0,1], vec![0,1,2,1,0,1,2,0,0,2,2,0], vec![0,1,2,0,0,2,2,0,1,0,2,0], 
        vec![0,2,2,0,1,0,2,0,1,0,2,1], vec![1,0,2,0,1,0,2,1,1,1,2,1], vec![1,2,1,1,1,1,1,2,1,2,1,2], vec![1,1,1,2,1,2,1,2,2,0,1,2], 
        vec![1,2,1,2,2,0,1,2,2,0,2,0], vec![2,0,1,2,2,0,2,0,2,0,2,1], vec![2,0,2,0,2,0,2,1,1,2,2,1], vec![2,0,2,1,1,2,2,1,1,2,2,0]]
    }

    fn mod_sudoku_xy_clues(x: usize, y: usize) -> structure::Structure {
        use rand::seq::SliceRandom;
        
        let mut rand_clues = Vec::new();
        let mut rand_linesegs = Vec::new();
        let base_clues = relations_sudoku();
        let base_linesegs = relations_modular();

        let mut indexes_clue = Vec::new();
        for i in 0..base_clues.len() {
            indexes_clue.push(i);
        }
        indexes_clue.shuffle(&mut rand::thread_rng());

        let mut indexes_ls = Vec::new();
        
        for i in 0..base_linesegs.len() {
            indexes_ls.push(i);
        }
        indexes_ls.shuffle(&mut rand::thread_rng());

        for i in 0..x {
            rand_clues.push(base_clues[indexes_clue[i]].clone());
        }

        for i in 0..y {
            rand_linesegs.push(base_linesegs[indexes_ls[i]].clone());
        } 
    
        structure::Structure {
            description: "sudoku 45 clues".to_string(),
            size: 3,
            constants: std::collections::HashMap::from([]),
            relations: std::collections::HashMap::from([("Cell".to_string(), rand_clues), ("Mod Line".to_string(), rand_linesegs)])
        }
    }

    fn sudoku_rules_four_ones(base_struc : structure::Structure) -> rel_cnf {
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &pred_funs::One(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![0,1,2,3,4,5])]), &base_struc.relations.clone());
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &pred_funs::One(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![0,1,4,5,2,3])]), &base_struc.relations.clone());
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &pred_funs::One(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![4,5,0,1,2,3])]), &base_struc.relations.clone());
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &pred_funs::One(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![0,4,1,5,2,3])]), &base_struc.relations.clone());

        sud_rules
    }
    
    fn sudoku_rules_two_ones_one_lone_some(base_struc : structure::Structure) -> rel_cnf {
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &pred_funs::Lone(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![0,1,2,3,4,5])]), &base_struc.relations.clone());
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &pred_funs::Some(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![0,1,4,5,2,3])]), &base_struc.relations.clone());
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &pred_funs::One(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![4,5,0,1,2,3])]), &base_struc.relations.clone());
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &pred_funs::One(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![0,4,1,5,2,3])]), &base_struc.relations.clone());

        sud_rules
    }
    
    fn sudoku_rules_two_somes_one_lone_one(base_struc : structure::Structure) -> rel_cnf {
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &pred_funs::Lone(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![0,1,2,3,4,5])]), &base_struc.relations.clone());
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &pred_funs::Some(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![0,1,4,5,2,3])]), &base_struc.relations.clone());
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &pred_funs::Some(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![4,5,0,1,2,3])]), &base_struc.relations.clone());
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &pred_funs::One(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![0,4,1,5,2,3])]), &base_struc.relations.clone());

        sud_rules
    }

    fn sudoku_rules_three_somes_one_lone(base_struc : structure::Structure) -> rel_cnf {
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &pred_funs::Lone(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![0,1,2,3,4,5])]), &base_struc.relations.clone());
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &pred_funs::Some(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![0,1,4,5,2,3])]), &base_struc.relations.clone());
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &pred_funs::Some(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![4,5,0,1,2,3])]), &base_struc.relations.clone());
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &pred_funs::Some(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![0,4,1,5,2,3])]), &base_struc.relations.clone());

        sud_rules
    }

    fn modular_line_rules(base_struc : structure::Structure) -> rel_cnf {
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &pred_funs::Lone(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![0,1,2,3,4,5])]), &base_struc.relations.clone());
        sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &pred_funs::Some(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![0,1,4,5,2,3])]), &base_struc.relations.clone());
        sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &pred_funs::Some(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![4,5,0,1,2,3])]), &base_struc.relations.clone());
        sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &pred_funs::Some(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![0,4,1,5,2,3])]), &base_struc.relations.clone());
        sud_rules = all_in_relation_pred(sud_rules, "Mod Line".to_string(), Vec::new(), &pred_funs::All(vec![3], Box::new(pred_funs::Some(vec![base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![0,1,2,3,13,12]), rel_literal::Pos("Cell".to_string(), vec![4,5,6,7,13,12]), rel_literal::Pos("Cell".to_string(), vec![8,9,10,11,13,12])]))), &base_struc.relations);

        sud_rules
    }

    fn modular_line_neg_rules(base_struc : structure::Structure) -> cnf_builder::rel_cnf {
        let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &cnf_builder::pred_funs::Lone(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,1,2,3,4,5])]), &base_struc.relations.clone());
        sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &cnf_builder::pred_funs::Some(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,1,4,5,2,3])]), &base_struc.relations.clone());
        sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &cnf_builder::pred_funs::Some(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![4,5,0,1,2,3])]), &base_struc.relations.clone());
        sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &cnf_builder::pred_funs::Some(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,4,1,5,2,3])]), &base_struc.relations.clone());
    
        sud_rules = all_in_relation_pred(sud_rules, "Mod Line".to_string(), Vec::new(), 
            &cnf_builder::pred_funs::All(vec![base_struc.size, base_struc.size, base_struc.size], 
            Box::new(cnf_builder::pred_funs::End(
            vec![cnf_builder::rel_literal::Neg("Cell".to_string(), vec![0,1,2,3,12,14]), 
            cnf_builder::rel_literal::Neg("Cell".to_string(), vec![4,5,6,7,13,14])]))), &base_struc.relations);
        
        sud_rules = all_in_relation_pred(sud_rules, "Mod Line".to_string(), Vec::new(), 
            &cnf_builder::pred_funs::All(vec![base_struc.size, base_struc.size, base_struc.size], 
            Box::new(cnf_builder::pred_funs::End(
            vec![cnf_builder::rel_literal::Neg("Cell".to_string(), vec![0,1,2,3,12,14]), 
            cnf_builder::rel_literal::Neg("Cell".to_string(), vec![8,9,10,11,13,14])]))), &base_struc.relations);
    
        sud_rules = all_in_relation_pred(sud_rules, "Mod Line".to_string(), Vec::new(), 
            &cnf_builder::pred_funs::All(vec![base_struc.size, base_struc.size, base_struc.size], 
            Box::new(cnf_builder::pred_funs::End(
            vec![cnf_builder::rel_literal::Neg("Cell".to_string(), vec![4,5,6,7,12,14]), 
            cnf_builder::rel_literal::Neg("Cell".to_string(), vec![8,9,10,11,13,14])]))), &base_struc.relations);
        
        sud_rules
    }

    fn solve_cnf(start_cnf : cnf_builder::rel_cnf, base_relations : HashMap<String, Vec<Vec<usize>>>) -> Vec<(String, Vec<usize>)> {
        let (conversion, lit_count, clause_count) = create_conversion_table(&start_cnf);
        let solution = solve_dimacs(create_dimacs_string(start_cnf, &conversion, lit_count, clause_count)).unwrap();

        convert_to_readable(solution, conversion, base_relations)
    }
    
    
    #[test]
    fn solve_sud_0_0Some() {
        let test_sudoku = sudoku_x_clues(15);

        let solution_1 = solve_cnf(sudoku_rules_four_ones(test_sudoku.clone()), test_sudoku.relations);

        println!("{:?}", solution_1)
    }

    #[test]
    fn test_sud_0_clues_1l1s20() {
        let test_sudoku = sudoku_x_clues(15);

        let solution_2 = solve_cnf(sudoku_rules_two_ones_one_lone_some(test_sudoku.clone()), test_sudoku.relations);

        println!("{:?}", solution_2)
    }

    #[test]
    fn test_sud_0_clues_2l1s1o() {
        let test_sudoku = sudoku_x_clues(15);

        let solution_3 = solve_cnf(sudoku_rules_two_somes_one_lone_one(test_sudoku.clone()), test_sudoku.relations);

        println!("{:?}", solution_3)
    }

    #[test]
    fn test_sud_0_clues_3l1s() {
        let test_sudoku = sudoku_x_clues(15);

        let solution_4 = solve_cnf(sudoku_rules_three_somes_one_lone(test_sudoku.clone()), test_sudoku.relations);

        println!("{:?}", solution_4)
    }

    #[test]
    fn test_modline_correctness() {
        let test_sudoku = mod_sudoku_xy_clues(0, 24);

        let solution_4 = solve_cnf(modular_line_rules(test_sudoku.clone()), test_sudoku.relations);

        println!("{:?}", solution_4)
    }
}
