//Felix Berliner
use std::{collections::HashMap, ptr::read_volatile};
use serde::{Serialize, Deserialize};
use structure::Structure;

#[derive(Clone, Debug)]
pub enum rel_literal {
    Pos(String, Vec<usize>),
    Neg(String, Vec<usize>)
}

pub type rel_clause = Vec<rel_literal>;

pub type rel_cnf = Vec<rel_clause>;

pub type pred_functions = fn (usize, rel_cnf, usize, Vec<usize>, &pred_funs) -> rel_cnf;
#[derive(Clone)]
pub enum pred_funs {
    /* 
        The End part of the enum refers to the last part of a predicate, which all predicates must have. The only field is an example clause, the details of which are specified in the "end_pred" function.
    */

    End(rel_clause),
    
    /* 
            The following enums call the functions corresponding to the next part of the predicate for whatever function it's used in, using these fields as inputs.
            Fields for these enums:
            1) Vector of sizes for all variables you want to iterate on
            2) The next step of the predicate after this part, works the same as the "nxt_fun" field in the functions where this enum is used.

            All is for the "all_pred" function
            Some is for the "some_pred" function
            Lone is for the "lone_pred" function
            one is for the "one_pred" function
    */

    All(Vec<usize>, Box<pred_funs>),
    Some(Vec<usize>, rel_clause),
    Lone(Vec<usize>, rel_clause),
    One(Vec<usize>, rel_clause)
}


/*
    This function translates the "for all" notation for a predicate for iterating on the members of a pre-specified relation.
    It will add groups of clauses to the base cnf for every member of the input relation.

    Fields for this function:
    1) The base cnf which you want to add clauses to (this should be the same cnf for every predicate you want to add to a structure).
    2) The symbol of the relation you want the "for all" to iterate on.
    3) The vector of any variables you want to take into account for the end predicate. If you have any constants, list them here, otherwise just use an empty vector.
    4) The next part of the predicate that comes after this one. All predicates must have an "End" as the last part, so if this is the only specification for this predicate just use an "End" here.
        (The fields for each specific enum are specified inside the enum definition above)
    5) The relations from the structure this predicate is for.

    This function, like all predicate functions, outputs the input cnf with added clauses depending on which function you called.
*/

pub fn all_in_relation_pred (mut base_cnf : rel_cnf, relation_symbol : String, mut pre_req_vars : Vec<usize>, nxt_fun : &pred_funs, relations : &HashMap<String, Vec<Vec<usize>>>) -> rel_cnf {
    let start_pred_reqs = pre_req_vars;
        match nxt_fun {
            pred_funs::All(x, y) => {
                for i in relations.get(&relation_symbol).unwrap().iter() {
                    pre_req_vars = start_pred_reqs.clone();
                    for a in i {
                        pre_req_vars.push(*a);
                    }
                    base_cnf = all_pred(x.clone(), base_cnf, pre_req_vars, y, &relations);
                }
            }
            pred_funs::Some(a, b) => {
                for i in relations.get(&relation_symbol).unwrap().iter() {
                    pre_req_vars = start_pred_reqs.clone();
                    for x in i {
                        pre_req_vars.push(*x);
                    }
                    base_cnf = some_pred(a.clone(), base_cnf,  pre_req_vars, &pred_funs::End(b.clone()), relations);
                }
            }
            pred_funs::Lone(a, b) => {
                for i in relations.get(&relation_symbol).unwrap().iter() {
                    pre_req_vars = start_pred_reqs.clone();
                    for x in i {
                        pre_req_vars.push(*x);
                    }
                    base_cnf = lone_pred(a.clone(), base_cnf, pre_req_vars, &pred_funs::End(b.clone()), relations);
                }
            }
            pred_funs::One(a, b) => {
                for i in relations.get(&relation_symbol).unwrap().iter() {
                    pre_req_vars = start_pred_reqs.clone();
                    for x in i {
                        pre_req_vars.push(*x);
                    }
                    base_cnf = one_pred(a.clone(), base_cnf, pre_req_vars, &pred_funs::End(b.clone()), relations);
                }
            }
            pred_funs::End(a) => {
                for i in relations.get(&relation_symbol).unwrap().iter() {
                    pre_req_vars = start_pred_reqs.clone();
                    for x in i {
                        pre_req_vars.push(*x);
                    }
                    let new_clause = end_pred(Vec::new(), a, &pre_req_vars, relations);
                    match new_clause {
                        Some(x) => {
                            base_cnf.push(x);
                        }
                        _ => {}
                    }
                }
            }
        }
        base_cnf
}

/*
    This function translates the "for all" notation for a predicate that iterates on the universe of a structure. 
    It will add groups of clauses to the base cnf for every combination of assignments from the input universes.

    Fields for this function:
    1) The vector of sizes representing the universes for each element
    2) The base cnf which you want to add clauses to (this should be the same cnf for every predicate you want to add to a structure).
    3) The vector of any variables you want to take into account for the end predicate. If you have any constants, list them here, otherwise just use an empty vector.
    4) The next part of the predicate that comes after this one. All predicates must have an "End" as the last part, so if this is the only specification for this predicate just use an "End" here.
        If the next function is an all then it will simply return the base_cnf with no changes. Please consolidate every "all" element into one function.
        (The fields for each specific enum are specified inside the enum definition above)
    5) The relations from the structure this predicate is for.

    This function, like all predicate functions, outputs the input cnf with added clauses depending on which function you called.
*/

pub fn all_pred (size : Vec<usize>, mut base_cnf : rel_cnf, mut pre_req_vars : Vec<usize>, nxt_fun : &pred_funs, relations : &HashMap<String, Vec<Vec<usize>>>) -> rel_cnf {
    let starting_ind: usize = pre_req_vars.len();

    for i in 0..size.len() {
        pre_req_vars.push(0);
    }

    match nxt_fun {
        pred_funs::End(x) => {
            'all_loop : loop {
                let mut new_clause = Vec::new();
                match end_pred(new_clause, &x, &pre_req_vars, &relations) {
                    None => {}
                    Some(x) => {
                        new_clause = x;
                        base_cnf.push(new_clause);
                    }
                }

                let mut add_one = true;
                for i in 0..size.len() {
                    if add_one && pre_req_vars[i + starting_ind] + 1 < size[i] {
                        pre_req_vars[i + starting_ind] += 1;

                        add_one = false;

                        for i2 in 0..i {
                            pre_req_vars[i2 + starting_ind] = 0;
                        }
                    } else if add_one && pre_req_vars[i + starting_ind] + 1 == size[i] && i == size.len() - 1 {
                        break 'all_loop
                    }
                }
            }
            base_cnf
        }
        pred_funs::Some(x,y) => {
            'all_loop : loop {
                base_cnf = some_pred(x.clone(), base_cnf, pre_req_vars.clone(), &pred_funs::End(y.clone()), &relations);
                
                let mut add_one = true;
                for i in 0..size.len() {
                    if add_one && pre_req_vars[i+starting_ind] + 1 < size[i] {
                        pre_req_vars[i+starting_ind] += 1;
        
                        add_one = false;
        
                        for i2 in 0..i {
                            pre_req_vars[i2+starting_ind] = 0;
                        }
                    } else if add_one && pre_req_vars[i+starting_ind] + 1 == size[i] && i == size.len() - 1 {
                        break 'all_loop
                    }
                }
            }
            base_cnf
        }
        pred_funs::Lone(x, y) => {
            'all_loop : loop {
                base_cnf = lone_pred(x.clone(), base_cnf, pre_req_vars.clone(), &pred_funs::End(y.clone()), &relations);
                
                let mut add_one = true;
                for i in 0..size.len() {
                    if add_one && pre_req_vars[i+starting_ind] + 1 < size[i] {
                        pre_req_vars[i+starting_ind] += 1;
        
                        add_one = false;
        
                        for i2 in 0..i {
                            pre_req_vars[i2+starting_ind] = 0;
                        }
                    } else if add_one && pre_req_vars[i+starting_ind] + 1 == size[i] && i == size.len() - 1 {
                        break 'all_loop
                    }
                }
            }
            base_cnf
        }
        pred_funs::One(x,y) => {
            'all_loop : loop {
                base_cnf = one_pred(x.clone(), base_cnf, pre_req_vars.clone(), &pred_funs::End(y.clone()), &relations);
                
                let mut add_one = true;
                for i in 0..size.len() {
                    if add_one && pre_req_vars[i+starting_ind] + 1 < size[i] {
                        pre_req_vars[i+starting_ind] += 1;
        
                        add_one = false;
        
                        for i2 in 0..i {
                            pre_req_vars[i2+starting_ind] = 0;
                        }
                    } else if add_one && pre_req_vars[i+starting_ind] + 1 == size[i] && i == size.len() - 1 {
                        break 'all_loop
                    }
                }
            }
            base_cnf
        }
        _ => {
            println!("Only an End, Some. Lone, or One may come after an All");
            base_cnf
        }
    }
}

/*
    This function translates the "existence" notation for a predicate that iterates on the universe of a structure. 
    It will add a single clause made up of variations of the end clause to the base cnf for every combination of assignments from the input universes.

    Fields for this function:
    1) The vector of sizes representing the universes for each element
    2) The base cnf which you want to add clauses to (this should be the same cnf for every predicate you want to add to a structure).
    3) The vector of any variables you want to take into account for the end predicate. If you have any constants, list them here, otherwise just use an empty vector.
    4) The next part of the predicate that comes after this one. Since the formula must follow CNF format and "some" is at the or level, this must be an End.
        If the next function called is not an End, it will just return the base cnf with no changes.
        (The fields for each specific enum are specified inside the enum definition above)
    5) The relations from the structure this predicate is for.

    This function, like all predicate functions, outputs the input cnf with added clauses depending on which function you called.
*/

pub fn some_pred (size : Vec<usize>, mut base_cnf : rel_cnf, mut pre_req_vars : Vec<usize>, nxt_fun : &pred_funs, relations : &HashMap<String, Vec<Vec<usize>>>) -> rel_cnf {
    let starting_ind = pre_req_vars.len();

    for i in 0..size.len() {
        pre_req_vars.push(0);
    }

    match nxt_fun {
        pred_funs::End(x) => {
            let mut some_clause : rel_clause = Vec::new();

            let mut pos_atom_found = false;
            'some_loop : loop { 
                match end_pred(some_clause.clone(), &x, &pre_req_vars, &relations) {
                    None => {
                        pos_atom_found = true;
                    }
                    Some(x) => {
                        some_clause = x;
                    }
                }

                let mut add_one = true;
                for i in 0..size.len() {
                    if add_one && pre_req_vars[i + starting_ind] + 1 < size[i] {
                        pre_req_vars[i + starting_ind] += 1;

                        add_one = false;

                        for i2 in 0..i {
                            pre_req_vars[i2 + starting_ind] = 0;
                        }
                    } else if add_one && pre_req_vars[i + starting_ind] + 1 == size[i] && i == size.len() - 1 {
                        break 'some_loop
                    }
                }
            }

            if !pos_atom_found {

                base_cnf.push(some_clause);
            }
            base_cnf
        }
        _ => {
            println!("Error: Should be an end fun after a some call");
            base_cnf
        }
    }
}

/*
    This function ensures that there can't be two or more assignments that fulfill the end predicate.
    It will add an and clause connecting negations of all pairs of combinations of assignments from the input universes.
    Because an end clause will cause significant blowup inside a lone, the end clause must only have 1 literal.

    Fields for this function:
    1) The vector of sizes representing the universes for each element
    2) The base cnf which you want to add clauses to (this should be the same cnf for every predicate you want to add to a structure).
    3) The vector of any variables you want to take into account for the end predicate. If you have any constants, list them here, otherwise just use an empty vector.
    4) The next part of the predicate that comes after this one. Since the formula must follow CNF format and "lone" is at the or level, this must be an End.
        If the next function called is not an End, it will just return the base cnf with no changes.
        (The fields for each specific enum are specified inside the enum definition above)
    5) The relations from the structure this predicate is for.

    This function, like all predicate functions, outputs the input cnf with added clauses depending on which function you called.
*/

pub fn lone_pred (size : Vec<usize>, mut base_cnf : rel_cnf, mut pre_req_vars : Vec<usize>, nxt_fun : &pred_funs, relations : &HashMap<String, Vec<Vec<usize>>>) -> rel_cnf {
    let strarting_ind = pre_req_vars.len();
    let mut compare_vars = pre_req_vars;
    let mut compare_vars_prime = compare_vars.clone();

    for i in 0..size.len() {
        compare_vars.push(0);
        compare_vars_prime.push(0);
    }

    match nxt_fun {
        pred_funs::End(x) => {
            'lone_loop_outer : loop {
                compare_vars_prime = compare_vars.clone();

                'lone_loop_inner : loop {
                    let mut add_one = true;

                    for i in 0..size.len() {
                        if add_one && compare_vars_prime[i + strarting_ind] + 1 < size[i] {
                            compare_vars_prime[i + strarting_ind] += 1;

                            add_one = false;

                            for i2 in 0..i {
                                compare_vars_prime[i2 + strarting_ind] = 0;
                            }
                        } else if add_one && compare_vars_prime[i + strarting_ind] + 1 == size[i] && i == size.len() - 1 {
                            break 'lone_loop_inner
                        }
                    }
                    match (end_pred(Vec::new(), &x, &compare_vars, &relations), end_pred(Vec::new(), &x, &compare_vars_prime, &relations)) {
                        (None, None) => {
                            base_cnf.push(Vec::new());
                        }
                        (None, Some(x)) => {
                            base_cnf = lone_aux_one(base_cnf, x);
                        }
                        (Some(x), None) => {
                            base_cnf = lone_aux_one(base_cnf, x);
                        }
                        (Some(x), Some(y)) => {
                            base_cnf = lone_aux_two(base_cnf, x, y)
                        }
                    }
                }

                let mut add_one = true;

                for i in 0..size.len() {
                    if add_one && compare_vars[i + strarting_ind] + 1 < size[i] {
                        compare_vars[i + strarting_ind] += 1;

                        add_one = false;

                        for i2 in 0..i {
                            compare_vars[i2 + strarting_ind] = 0;
                        }
                    } else if add_one && compare_vars[i + strarting_ind] + 1 == size[i] && i == size.len() - 1 {
                        break 'lone_loop_outer
                    }
                }
            }
            base_cnf
        }
        _ => {
            println!("Error: Should be an end fun after a lone call");
            base_cnf
        }
    }
}
fn lone_aux_one (mut base_cnf : rel_cnf, compare_clause : rel_clause) -> rel_cnf {
    for i in compare_clause.iter() {
        match i {
            rel_literal::Pos(x, y) => {
                let new_clause = vec![rel_literal::Neg(x.clone(),y.clone())];
                base_cnf.push(new_clause);
            }
            rel_literal::Neg(x,y) => {
                let new_clause = vec![rel_literal::Pos(x.clone(),y.clone())];
                base_cnf.push(new_clause);
            }
        }
    }

    base_cnf
}

fn lone_aux_two (mut base_cnf : rel_cnf, compare_clause_1 : rel_clause, compare_clause_2 : rel_clause) -> rel_cnf {
    if compare_clause_1.len() > 1 || compare_clause_2.len() > 1 {
        println!("When using a Lone on 2 clauses, neither can be more than a length of 1");
    }

    match(&compare_clause_1[0], &compare_clause_2[0]) {
        (rel_literal::Pos(name_1, vars_1), rel_literal::Pos(name_2, vars_2)) => {
            let mut new_clause = Vec::new();
            if name_1 != name_2 || vars_1 != vars_2 {
                new_clause.push(rel_literal::Neg(name_1.clone(), vars_1.clone()));
                new_clause.push(rel_literal::Neg(name_2.clone(), vars_2.clone()));
            } else {
                new_clause.push(rel_literal::Neg(name_1.clone(), vars_1.clone()));
            }
            base_cnf.push(new_clause);
        }
        (rel_literal::Pos(name_1, vars_1), rel_literal::Neg(name_2, vars_2)) => {
                if name_1 != name_2 || vars_1 != vars_2 {
                    let mut new_clause = Vec::new();
                    new_clause.push(rel_literal::Neg(name_1.clone(), vars_1.clone()));
                    new_clause.push(rel_literal::Pos(name_2.clone(), vars_2.clone()));
                    base_cnf.push(new_clause);
                }
        }
        (rel_literal::Neg(name_1, vars_1), rel_literal::Neg(name_2, vars_2)) => {
                let mut new_clause = Vec::new();
                if name_1 != name_2 || vars_1 != vars_2 {
                    new_clause.push(rel_literal::Pos(name_1.clone(), vars_1.clone()));
                    new_clause.push(rel_literal::Pos(name_2.clone(), vars_2.clone()));
                } else {
                    new_clause.push(rel_literal::Pos(name_1.clone(), vars_1.clone()));
                }
                base_cnf.push(new_clause);
        }
        (rel_literal::Neg(name_1, vars_1), rel_literal::Pos(name_2, vars_2)) => {
                if name_1 != name_2 || vars_1 != vars_2 {
                    let mut new_clause = Vec::new();
                    new_clause.push(rel_literal::Pos(name_1.clone(), vars_1.clone()));
                    new_clause.push(rel_literal::Neg(name_2.clone(), vars_2.clone()));
                    base_cnf.push(new_clause);
                }
        }
    }

    base_cnf
}

/*
    This function adds both the some_pred and lone_pred specifications to a predicate. 
    It will run both the some_pred and lone_pred functions for the same set of inputs.

    Fields for this function:
    1) The vector of sizes representing the universes for each element
    2) The base cnf which you want to add clauses to (this should be the same cnf for every predicate you want to add to a structure).
    3) The vector of any variables you want to take into account for the end predicate. If you have any constants, list them here, otherwise just use an empty vector.
    4) The next part of the predicate that comes after this one. Since the formula must follow CNF format and "one" is at the or level, this must be an End.
        If the next function called is not an End, it will just return the base cnf with no changes.
        (The fields for each specific enum are specified inside the enum definition above)
    5) The relations from the structure this predicate is for.

    This function, like all predicate functions, outputs the input cnf with added clauses depending on which function you called.
*/

pub fn one_pred (size : Vec<usize>, mut base_cnf : rel_cnf, mut pre_req_vars : Vec<usize>, nxt_fun : &pred_funs, relations : &HashMap<String, Vec<Vec<usize>>>) -> rel_cnf {
    match nxt_fun {
        pred_funs::End(x) => {
            let new_pre_reqs = pre_req_vars.clone();
            base_cnf = some_pred(size.clone(), base_cnf, pre_req_vars, &pred_funs::End(x.clone()), &relations);
            base_cnf = lone_pred(size, base_cnf, new_pre_reqs, &pred_funs::End(x.clone()), &relations);
            base_cnf
        }
        _ => {
            println!("Error: Should be an end fun after a one call");
            base_cnf
        }
    }
}

/*
    This function is the last part of a predicate, where the iterated variables are assigned to a set of literals.

    The first field of this function is the base_clause, which the end_pred will add literals on to. Unless you want constant literals in the clause, this can be an empty vector.
    
    The second field is the example clause. This is the instructions that the function uses to assign elements of pre_req_vars to the literal.
    The example clause is formatted like a normal clause, but instead of plugging values into the vectors of integers, the values should instead be the index of the pre_req_vars vector you want to reference.
    Pre_req_vars is formatted with the constants the user inputs first, followed by the elements each predicate part iterates on in order.
    For example, if the predicate has 2 user inputted constants, then an all function iterating on 4 elements, then a some function iterating on 2 elements:
    0, 1 would refer to the two constants,
    2,3,4,5 would refer to the variables that the all iterates on,
    6,7 would refer to the variables that the some iterates on.
    The example clause must use indexes that are less than the size of pre_req_vars, otherwise it will cause an error.

    The third field is the vector of all variables and constants you want the predicate to take into account, if you have no constants you want to input, this can just be an empty vector.

    The fourth field is the relations from this predicate's structure. This function will automatically remove any already determined relations from the CNF according to this hashmap.

    This function returns an option for a clause. If the option is None, then the clause it would have added would be automatically true. Otherwise, the clause inside the Some is the clause made from the pre_req_vars.
*/
pub fn end_pred (mut base_clause : rel_clause, example_clause : &rel_clause, pre_req_vars : &Vec<usize>, relations : &HashMap<String, Vec<Vec<usize>>>) -> Option<rel_clause> {
    for x in 0..example_clause.len() {
        match &example_clause[x] {
            rel_literal::Pos(name, vars) => {
                let mut new_vars = Vec::new();
                for i in 0..vars.len() {
                    new_vars.push(pre_req_vars[vars[i]]);
                    
                }

                match relations.get(&name.clone()) {
                    None => {         
                    base_clause.push(rel_literal::Pos(name.clone(), new_vars));
                    }
                    Some(x) => {
                        let mut atom_found = false;
                        for i in x.iter() {
                            if *i == new_vars {
                                atom_found = true;
                            }
                        }

                        if !atom_found {  
                            base_clause.push(rel_literal::Pos(name.clone(), new_vars));
                        } else {
                            return None;
                        }
                    }
                }
            }
            rel_literal::Neg(name, vars) => {
                let mut new_vars = Vec::new();
                for i in 0..vars.len() {
                    new_vars.push(pre_req_vars[vars[i]]);
                    
                }

                match relations.get(&name.clone()) {
                    None => {
                        base_clause.push(rel_literal::Neg(name.clone(), new_vars));
                    }
                    Some(x) => {
                        let mut atom_found = false;
                        for i in x.iter() {
                            if *i == new_vars {
                                atom_found = true;
                            }
                        }

                        if !atom_found {
                            base_clause.push(rel_literal::Neg(name.clone(), new_vars));
                        }
                    }
                }
            }
        }
    }

    Some(base_clause)
}


#[cfg(test)]
mod tests {
    /*use crate::all_pred;
    use crate::pred_funs;
    use crate::rel_cnf;
    use crate::rel_literal;
    /*use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }*/
    fn structure_sudoku_0_clues() -> structure::Structure {
        structure::Structure {
            description: "sudoku 0 clues".to_string(),
            size: 3,
            constants: std::collections::HashMap::from([]),
            relations: std::collections::HashMap::from([])
        }
    }

    fn structure_sudoku_5_clues() -> structure::Structure {
        structure::Structure {
            description: "sudoku 5 clues".to_string(),
            size: 3,
            constants: std::collections::HashMap::from([]),
            relations: std::collections::HashMap::from([("Cell".to_string(), vec![vec![0,0,0,0,0,1], vec![0,0,0,1,1,2], vec![0,0,0,2,2,1], 
            vec![0,0,1,1,2,2], vec![0,0,2,2,0,0]])])
        }
    }

    fn structure_sudoku_10_clues() -> structure::Structure {
        structure::Structure {
            description: "sudoku 10 clues".to_string(),
            size: 3,
            constants: std::collections::HashMap::from([]),
            relations: std::collections::HashMap::from([("Cell".to_string(), vec![vec![0,0,0,0,0,1], vec![0,0,0,1,1,2], vec![0,0,0,2,2,1], 
            vec![0,0,1,1,2,2], vec![0,0,2,2,0,0], vec![0,1,0,2,2,2], vec![0,1,2,1,2,0], vec![0,2,0,2,1,0], vec![1,0,0,0,1,1], 
            vec![1,0,0,2,0,0]])])
        }
    }

    fn structure_sudoku_15_clues() -> structure::Structure {
        structure::Structure {
            description: "sudoku 15 clues".to_string(),
            size: 3,
            constants: std::collections::HashMap::from([]),
            relations: std::collections::HashMap::from([("Cell".to_string(), vec![vec![0,0,0,0,0,1], vec![0,0,0,1,1,2], vec![0,0,0,2,2,1], 
            vec![0,0,1,1,2,2], vec![0,0,2,2,0,0], vec![0,1,0,2,2,2], vec![0,1,2,1,2,0], vec![0,2,0,2,1,0], vec![1,0,0,0,1,1], 
            vec![1,0,0,2,0,0], vec![1,0,1,0,0,2], vec![1,0,2,0,1,0], vec![1,0,2,2,2,0], vec![1,1,1,0,0,0], vec![1,1,1,2,2,1]])])
        }
    }

    fn structure_sudoku_25_clues() -> structure::Structure {
        structure::Structure {
            description: "sudoku 25 clues".to_string(),
            size: 3,
            constants: std::collections::HashMap::from([]),
            relations: std::collections::HashMap::from([("Cell".to_string(), vec![vec![0,0,0,0,0,1], vec![0,0,0,1,1,2], vec![0,0,0,2,2,1], 
            vec![0,0,1,1,2,2], vec![0,0,2,2,0,0], vec![0,1,0,2,2,2], vec![0,1,2,1,2,0], vec![0,2,0,2,1,0], vec![1,0,0,0,1,1], 
            vec![1,0,0,2,0,0], vec![1,0,1,0,0,2], vec![1,0,2,0,1,0], vec![1,0,2,2,2,0], vec![1,1,1,0,0,0], vec![1,1,1,2,2,1], 
            vec![1,1,2,0,1,2], vec![1,1,2,2,0,1], vec![1,2,0,1,2,1], vec![1,2,1,0,1,0], vec![1,2,1,2,2,2], vec![1,2,2,0,1,1], 
            vec![2,0,0,1,0,1], vec![2,0,0,2,1,2], vec![2,0,1,0,2,2], vec![2,1,0,0,2,1]])])
        }
    }

    fn structure_sudoku_35_clues() -> structure::Structure {
        structure::Structure {
            description: "sudoku 35 clues".to_string(),
            size: 3,
            constants: std::collections::HashMap::from([]),
            relations: std::collections::HashMap::from([("Cell".to_string(), vec![vec![0,0,0,0,0,1], vec![0,0,0,1,1,2], vec![0,0,0,2,2,1], 
            vec![0,0,1,1,2,2], vec![0,0,2,2,0,0], vec![0,1,0,2,2,2], vec![0,1,2,1,2,0], vec![0,2,0,2,1,0], vec![1,0,0,0,1,1], 
            vec![1,0,0,2,0,0], vec![1,0,1,0,0,2], vec![1,0,2,0,1,0], vec![1,0,2,2,2,0], vec![1,1,1,0,0,0], vec![1,1,1,2,2,1], 
            vec![1,1,2,0,1,2], vec![1,1,2,2,0,1], vec![1,2,0,1,2,1], vec![1,2,1,0,1,0], vec![1,2,1,2,2,2], vec![1,2,2,0,1,1], 
            vec![2,0,0,1,0,1], vec![2,0,0,2,1,2], vec![2,0,1,0,2,2], vec![2,1,0,0,2,1], vec![2,1,0,1,0,2], vec![2,1,1,1,0,1], 
            vec![2,2,0,1,1,0], vec![2,2,0,2,2,0], vec![2,2,2,0,0,0], vec![2,0,1,2,2,0], vec![1,1,2,1,2,2], vec![0,2,0,0,2,0], 
            vec![1,1,0,1,2,0], vec![0,0,0,2,2,0]])])
        }
    }

    fn structure_sudoku_45_clues() -> structure::Structure {
        structure::Structure {
            description: "sudoku 45 clues".to_string(),
            size: 3,
            constants: std::collections::HashMap::from([]),
            relations: std::collections::HashMap::from([("Cell".to_string(), vec![vec![0,0,0,0,0,1], vec![0,0,0,1,1,2], vec![0,0,0,2,2,1], 
            vec![0,0,1,1,2,2], vec![0,0,2,2,0,0], vec![0,1,0,2,2,2], vec![0,1,2,1,2,0], vec![0,2,0,2,1,0], vec![1,0,0,0,1,1], 
            vec![1,0,0,2,0,0], vec![1,0,1,0,0,2], vec![1,0,2,0,1,0], vec![1,0,2,2,2,0], vec![1,1,1,0,0,0], vec![1,1,1,2,2,1], 
            vec![1,1,2,0,1,2], vec![1,1,2,2,0,1], vec![1,2,0,1,2,1], vec![1,2,1,0,1,0], vec![1,2,1,2,2,2], vec![1,2,2,0,1,1], 
            vec![2,0,0,1,0,1], vec![2,0,0,2,1,2], vec![2,0,1,0,2,2], vec![2,1,0,0,2,1], vec![2,1,0,1,0,2], vec![2,1,1,1,0,1], 
            vec![2,2,0,1,1,0], vec![2,2,0,2,2,0], vec![2,2,2,0,0,0], vec![2,0,1,2,2,0], vec![1,1,2,1,2,2], vec![0,2,0,0,2,0], 
            vec![1,1,0,1,2,0], vec![0,0,0,2,2,0], vec![2,0,0,0,0,0], vec![1,0,1,1,1,2], vec![2,1,2,1,1,0], vec![2,1,2,2,2,2], 
            vec![0,0,1,0,2,0], vec![0,2,1,2,0,2], vec![2,2,1,0,2,1], vec![0,2,2,1,1,2], vec![1,1,0,0,1,0], vec![2,2,1,2,1,1]])])
        }
    }

    fn sudoku_rules_four_ones(base_struc : structure::Structure) -> rel_cnf {
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &pred_funs::One(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![0,1,2,3,4,5])]), &base_struc.relations.clone());
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &pred_funs::One(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![0,1,4,5,2,3])]), &base_struc.relations.clone());
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &pred_funs::One(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![4,5,0,1,2,3])]), &base_struc.relations.clone());
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &pred_funs::One(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![0,4,1,5,2,3])]), &base_struc.relations.clone());

        sud_rules
    }
    
    fn sudoku_rules_two_ones_one_lone_some(base_struc : structure::Structure) -> rel_cnf {
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &pred_funs::Lone(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![0,1,2,3,4,5])]), &base_struc.relations.clone());
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &pred_funs::Some(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![0,1,4,5,2,3])]), &base_struc.relations.clone());
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &pred_funs::One(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![4,5,0,1,2,3])]), &base_struc.relations.clone());
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &pred_funs::One(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![0,4,1,5,2,3])]), &base_struc.relations.clone());

        sud_rules
    }
    
    fn sudoku_rules_two_somes_one_lone_one(base_struc : structure::Structure) -> rel_cnf {
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &pred_funs::Lone(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![0,1,2,3,4,5])]), &base_struc.relations.clone());
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &pred_funs::Some(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![0,1,4,5,2,3])]), &base_struc.relations.clone());
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &pred_funs::Some(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![4,5,0,1,2,3])]), &base_struc.relations.clone());
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &pred_funs::One(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![0,4,1,5,2,3])]), &base_struc.relations.clone());

        sud_rules
    }

    fn sudoku_rules_three_somes_one_lone(base_struc : structure::Structure) -> rel_cnf {
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &pred_funs::Lone(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![0,1,2,3,4,5])]), &base_struc.relations.clone());
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &pred_funs::Some(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![0,1,4,5,2,3])]), &base_struc.relations.clone());
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &pred_funs::Some(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![4,5,0,1,2,3])]), &base_struc.relations.clone());
        let mut sud_rules = all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &pred_funs::Some(vec![base_struc.size, base_struc.size], vec![rel_literal::Pos("Cell".to_string(), vec![0,4,1,5,2,3])]), &base_struc.relations.clone());

        sud_rules
    }

    #[test]
    fn test_sud_0_clues_4s() {
        let test_cnf = sudoku_rules_four_ones(structure_sudoku_0_clues());

        println!("{:?}", test_cnf.len())
    }

    #[test]
    fn test_sud_0_clues_1l1s20() {
        let test_cnf = sudoku_rules_two_ones_one_lone_some(structure_sudoku_0_clues());

        println!("{:?}", test_cnf.len())
    }

    #[test]
    fn test_sud_0_clues_2l1s1o() {
        let test_cnf = sudoku_rules_two_somes_one_lone_one(structure_sudoku_0_clues());

        println!("{:?}", test_cnf.len())
    }

    #[test]
    fn test_sud_0_clues_3l1s() {
        let test_cnf = sudoku_rules_three_somes_one_lone(structure_sudoku_0_clues());

        println!("{:?}", test_cnf.len())
    }*/
}
