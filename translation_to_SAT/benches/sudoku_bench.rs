use criterion::{black_box, criterion_group, criterion_main, Criterion};
use translation_to_SAT::{create_dimacs_string, solve_dimacs};

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

    /*vec![
        vec![0,0,0,0,0,0,0,1,0,1,0,1], vec![0,0,0,0,0,1,0,0,0,1,0,1], vec![0,0,0,0,0,1,0,1,0,1,0,2], vec![0,0,0,0,0,1,0,0,0,2,0,0], 
        vec![0,0,0,1,0,0,0,2,0,0,1,0], vec![0,0,0,1,0,1,0,1,0,2,0,1], vec![0,1,0,1,0,0,0,1,0,0,0,2], 
        vec![0,0,0,2,0,0,1,0,0,0,1,1], vec![0,0,0,2,0,1,0,2,0,2,0,2], vec![0,0,1,0,0,0,0,2,0,1,0,2], vec![0,1,0,1,0,0,0,2,0,1,0,2], vec![0,1,0,0,0,1,0,1,0,0,0,2], 
        vec![0,0,1,0,0,1,0,2,0,2,0,1], vec![0,0,1,0,0,1,0,2,0,2,1,0], vec![0,0,1,0,0,1,0,2,0,1,1,0], vec![0,1,0,2,0,0,1,0,0,1,1,1], vec![0,0,1,0,0,1,1,0,0,0,1,1], vec![0,0,1,0,0,1,1,1,0,0,1,1], vec![0,0,1,0,0,1,1,1,0,1,1,2], vec![0,0,1,0,0,1,1,1,0,2,1,2], 
        vec![0,0,1,1,0,1,1,0,0,2,1,1], vec![0,0,1,1,0,1,1,0,0,2,0,2], vec![0,1,1,0,0,0,1,1,0,0,1,2], vec![0,0,1,1,0,1,1,1,0,2,1,1], vec![0,1,1,1,0,0,1,1,0,0,1,2], vec![0,0,1,1,0,0,1,2,0,1,2,0], 
(up to 0,0,1,1)
    ]*/
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

fn sudoku_rules_four_ones(base_struc : structure::Structure) -> cnf_builder::rel_cnf {
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &cnf_builder::pred_funs::One(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,1,2,3,4,5])]), &base_struc.relations.clone());
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &cnf_builder::pred_funs::One(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,1,4,5,2,3])]), &base_struc.relations.clone());
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &cnf_builder::pred_funs::One(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![4,5,0,1,2,3])]), &base_struc.relations.clone());
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &cnf_builder::pred_funs::One(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,4,1,5,2,3])]), &base_struc.relations.clone());

    sud_rules
}

fn sudoku_rules_two_ones_one_lone_some(base_struc : structure::Structure) -> cnf_builder::rel_cnf {
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &cnf_builder::pred_funs::Lone(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,1,2,3,4,5])]), &base_struc.relations.clone());
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &cnf_builder::pred_funs::Some(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,1,4,5,2,3])]), &base_struc.relations.clone());
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &cnf_builder::pred_funs::One(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![4,5,0,1,2,3])]), &base_struc.relations.clone());
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &cnf_builder::pred_funs::One(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,4,1,5,2,3])]), &base_struc.relations.clone());

    sud_rules
}

fn sudoku_rules_two_somes_one_lone_one(base_struc : structure::Structure) -> cnf_builder::rel_cnf {
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &cnf_builder::pred_funs::Lone(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,1,2,3,4,5])]), &base_struc.relations.clone());
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &cnf_builder::pred_funs::Some(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,1,4,5,2,3])]), &base_struc.relations.clone());
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &cnf_builder::pred_funs::Some(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![4,5,0,1,2,3])]), &base_struc.relations.clone());
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &cnf_builder::pred_funs::One(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,4,1,5,2,3])]), &base_struc.relations.clone());

    sud_rules
}

fn sudoku_rules_three_somes_one_lone(base_struc : structure::Structure) -> cnf_builder::rel_cnf {
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), 
        &cnf_builder::pred_funs::Lone(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,1,2,3,4,5])]), &base_struc.relations);
    sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), 
        &cnf_builder::pred_funs::Some(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,1,4,5,2,3])]), &base_struc.relations);
    sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), 
        &cnf_builder::pred_funs::Some(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![4,5,0,1,2,3])]), &base_struc.relations);
    sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), 
        &cnf_builder::pred_funs::Some(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,4,1,5,2,3])]), &base_struc.relations);

    sud_rules
}

fn modular_line_rules(base_struc : structure::Structure) -> cnf_builder::rel_cnf {
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &cnf_builder::pred_funs::Lone(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,1,2,3,4,5])]), &base_struc.relations.clone());
    sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &cnf_builder::pred_funs::Some(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,1,4,5,2,3])]), &base_struc.relations.clone());
    sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &cnf_builder::pred_funs::Some(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![4,5,0,1,2,3])]), &base_struc.relations.clone());
    sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &cnf_builder::pred_funs::Some(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,4,1,5,2,3])]), &base_struc.relations.clone());
    sud_rules = cnf_builder::all_in_relation_pred(sud_rules, "Mod Line".to_string(), Vec::new(), &cnf_builder::pred_funs::All(vec![3], Box::new(cnf_builder::pred_funs::Some(vec![base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,1,2,3,13,12]), cnf_builder::rel_literal::Pos("Cell".to_string(), vec![4,5,6,7,13,12]), cnf_builder::rel_literal::Pos("Cell".to_string(), vec![8,9,10,11,13,12])]))), &base_struc.relations);

    sud_rules
}

fn modular_line_neg_rules(base_struc : structure::Structure) -> cnf_builder::rel_cnf {
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &cnf_builder::pred_funs::Lone(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,1,2,3,4,5])]), &base_struc.relations.clone());
    sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &cnf_builder::pred_funs::Some(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,1,4,5,2,3])]), &base_struc.relations.clone());
    sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &cnf_builder::pred_funs::Some(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![4,5,0,1,2,3])]), &base_struc.relations.clone());
    sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &cnf_builder::pred_funs::Some(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,4,1,5,2,3])]), &base_struc.relations.clone());

    sud_rules = cnf_builder::all_in_relation_pred(sud_rules, "Mod Line".to_string(), Vec::new(), 
        &cnf_builder::pred_funs::All(vec![base_struc.size, base_struc.size, base_struc.size], 
        Box::new(cnf_builder::pred_funs::End(
        vec![cnf_builder::rel_literal::Neg("Cell".to_string(), vec![0,1,2,3,12,14]), 
        cnf_builder::rel_literal::Neg("Cell".to_string(), vec![4,5,6,7,13,14])]))), &base_struc.relations);
    
    sud_rules = cnf_builder::all_in_relation_pred(sud_rules, "Mod Line".to_string(), Vec::new(), 
        &cnf_builder::pred_funs::All(vec![base_struc.size, base_struc.size, base_struc.size], 
        Box::new(cnf_builder::pred_funs::End(
        vec![cnf_builder::rel_literal::Neg("Cell".to_string(), vec![0,1,2,3,12,14]), 
        cnf_builder::rel_literal::Neg("Cell".to_string(), vec![8,9,10,11,13,14])]))), &base_struc.relations);

    sud_rules = cnf_builder::all_in_relation_pred(sud_rules, "Mod Line".to_string(), Vec::new(), 
        &cnf_builder::pred_funs::All(vec![base_struc.size, base_struc.size, base_struc.size], 
        Box::new(cnf_builder::pred_funs::End(
        vec![cnf_builder::rel_literal::Neg("Cell".to_string(), vec![4,5,6,7,12,14]), 
        cnf_builder::rel_literal::Neg("Cell".to_string(), vec![8,9,10,11,13,14])]))), &base_struc.relations);
    
    sud_rules
}

fn solve_cnf(start_cnf : cnf_builder::rel_cnf, base_relations : std::collections::HashMap<String, Vec<Vec<usize>>>) -> Vec<(String, Vec<usize>)> {
    let (conversion, lit_count, clause_count) = translation_to_SAT::create_conversion_table(&start_cnf);
    let solution = solve_dimacs(create_dimacs_string(start_cnf, &conversion, lit_count, clause_count)).unwrap();

    translation_to_SAT::convert_to_readable(solution, conversion, base_relations)
}

fn build_and_solve_1_lone(clue_num : usize) -> Vec<(String, Vec<usize>)> {
    let test_sudoku = sudoku_x_clues(clue_num);
    solve_cnf(sudoku_rules_three_somes_one_lone(test_sudoku.clone()), test_sudoku.relations)
}

fn build_and_solve_2_lone(clue_num : usize) -> Vec<(String, Vec<usize>)> {
    let test_sudoku = sudoku_x_clues(clue_num);
    solve_cnf(sudoku_rules_two_somes_one_lone_one(test_sudoku.clone()), test_sudoku.relations)
}

fn build_and_solve_3_lone(clue_num : usize) -> Vec<(String, Vec<usize>)> {
    let test_sudoku = sudoku_x_clues(clue_num);
    solve_cnf(sudoku_rules_two_ones_one_lone_some(test_sudoku.clone()), test_sudoku.relations)
}

fn build_and_solve_4_lone(clue_num : usize) -> Vec<(String, Vec<usize>)> {
    let test_sudoku = sudoku_x_clues(clue_num);
    solve_cnf(sudoku_rules_four_ones(test_sudoku.clone()), test_sudoku.relations)
}

fn build_and_solve_mod_line(clue_num : usize, segment_num : usize) -> Vec<(String, Vec<usize>)> {
    let test_sudoku = mod_sudoku_xy_clues(clue_num, segment_num);
    solve_cnf(modular_line_rules(test_sudoku.clone()), test_sudoku.relations)
}

fn b_s_neg_mod_line(clue_num : usize, segment_num : usize) -> Vec<(String, Vec<usize>)> {
    let test_sudoku = mod_sudoku_xy_clues(clue_num, segment_num);
    solve_cnf(modular_line_neg_rules(test_sudoku.clone()), test_sudoku.relations)
}

fn bench_sudoku(c: &mut Criterion) {
    
    let mut group = c.benchmark_group("Mod Sudoku - Solving");

    for x in 0..9 {
        for y in 0..25 {
            group.bench_function(criterion::BenchmarkId::new("Mod line solve test", format!("{} Clues, {} Line segments", x*5,y)), |b| b.iter(|| build_and_solve_mod_line(x*5, y)));
        }
    }

    /*for x in 0..46 {
        group.bench_function(criterion::BenchmarkId::new("Vanilla Sudoku Solve Test", format!("Clues : {}", x)), |b| b.iter(|| build_and_solve_1_lone(x)));
    }*/

    /*for x in 0..9 {
        group.bench_function(criterion::BenchmarkId::new("1 Lone test", format!("{} Clues", x*10)), |b| b.iter(|| build_and_solve_1_lone(x*5)));
        group.bench_function(criterion::BenchmarkId::new("2 lone test", format!("{} Clues", x*10)), |b| b.iter(|| build_and_solve_2_lone(x*5)));
        group.bench_function(criterion::BenchmarkId::new("3 lone test", format!("{} Clues", x*10)), |b| b.iter(|| build_and_solve_3_lone(x*5)));
        group.bench_function(criterion::BenchmarkId::new("4 lone test", format!("{} Clues", x*10)), |b| b.iter(|| build_and_solve_4_lone(x*5)));

    }*/

    group.finish();
}
criterion_group!(benches, bench_sudoku);
criterion_main!(benches);
