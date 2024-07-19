use cnf_builder::all_in_relation_pred;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

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
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &cnf_builder::pred_funs::Lone(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,1,2,3,4,5])]), &base_struc.relations.clone());
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &cnf_builder::pred_funs::Some(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,1,4,5,2,3])]), &base_struc.relations.clone());
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &cnf_builder::pred_funs::Some(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![4,5,0,1,2,3])]), &base_struc.relations.clone());
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], sud_rules, Vec::new(), &cnf_builder::pred_funs::Some(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,4,1,5,2,3])]), &base_struc.relations.clone());

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

fn build_mod_line(clue_num : usize, segment_num : usize) -> cnf_builder::rel_cnf {
    let test_sudoku = mod_sudoku_xy_clues(clue_num, segment_num);
    modular_line_rules(test_sudoku.clone())
}

fn build_neg_mod_line(clue_num : usize, segment_num : usize) -> cnf_builder::rel_cnf {
    let test_sudoku = mod_sudoku_xy_clues(clue_num, segment_num);
    modular_line_neg_rules(test_sudoku.clone())
}

fn bench_sudoku(c: &mut Criterion) {

    let mut group = c.benchmark_group("Mod Sudoku Build");

    /*for x in 0..5 {
        for y in 0..9 {
            group.bench_function(criterion::BenchmarkId::new("Mod line build test", format!("{} Clues, {} Line segments", x*10,y*3)), |b| b.iter(|| build_neg_mod_line(x*10, y*3)));

        }
    }*/

    /*for x in 0..46 {
        group.bench_function(criterion::BenchmarkId::new("Vanilla Sudoku Build Test", format!("Clues : {}", x)), |b| b.iter(|| sudoku_rules_three_somes_one_lone(sudoku_x_clues(x))));
    }*/
    
    group.finish();
}
criterion_group!(benches, bench_sudoku);
criterion_main!(benches);
