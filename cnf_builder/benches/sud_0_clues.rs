use criterion::{black_box, criterion_group, criterion_main, Criterion};

/*fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
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

fn sudoku_rules_four_ones(base_struc : structure::Structure) -> cnf_builder::rel_cnf {
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &cnf_builder::pred_funs::One(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,1,2,3,4,5])]), &base_struc.relations.clone());
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &cnf_builder::pred_funs::One(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,1,4,5,2,3])]), &base_struc.relations.clone());
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &cnf_builder::pred_funs::One(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![4,5,0,1,2,3])]), &base_struc.relations.clone());
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &cnf_builder::pred_funs::One(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,4,1,5,2,3])]), &base_struc.relations.clone());

    sud_rules
}

fn sudoku_rules_two_ones_one_lone_some(base_struc : structure::Structure) -> cnf_builder::rel_cnf {
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &cnf_builder::pred_funs::Lone(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,1,2,3,4,5])]), &base_struc.relations.clone());
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &cnf_builder::pred_funs::Some(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,1,4,5,2,3])]), &base_struc.relations.clone());
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &cnf_builder::pred_funs::One(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![4,5,0,1,2,3])]), &base_struc.relations.clone());
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &cnf_builder::pred_funs::One(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,4,1,5,2,3])]), &base_struc.relations.clone());

    sud_rules
}

fn sudoku_rules_two_somes_one_lone_one(base_struc : structure::Structure) -> cnf_builder::rel_cnf {
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &cnf_builder::pred_funs::Lone(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,1,2,3,4,5])]), &base_struc.relations.clone());
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &cnf_builder::pred_funs::Some(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,1,4,5,2,3])]), &base_struc.relations.clone());
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &cnf_builder::pred_funs::Some(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![4,5,0,1,2,3])]), &base_struc.relations.clone());
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &cnf_builder::pred_funs::One(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,4,1,5,2,3])]), &base_struc.relations.clone());

    sud_rules
}

fn sudoku_rules_three_somes_one_lone(base_struc : structure::Structure) -> cnf_builder::rel_cnf {
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &cnf_builder::pred_funs::Lone(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,1,2,3,4,5])]), &base_struc.relations.clone());
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &cnf_builder::pred_funs::Some(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,1,4,5,2,3])]), &base_struc.relations.clone());
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &cnf_builder::pred_funs::Some(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![4,5,0,1,2,3])]), &base_struc.relations.clone());
    let mut sud_rules = cnf_builder::all_pred(vec![base_struc.size, base_struc.size, base_struc.size, base_struc.size], Vec::new(), Vec::new(), &cnf_builder::pred_funs::Some(vec![base_struc.size, base_struc.size], vec![cnf_builder::rel_literal::Pos("Cell".to_string(), vec![0,4,1,5,2,3])]), &base_struc.relations.clone());

    sud_rules
}

fn test_sud_0_clues_4s() {
    let test_cnf = sudoku_rules_four_ones(structure_sudoku_0_clues());

    println!("{:?}", test_cnf.len())
}

fn test_sud_0_clues_1l1s20() {
    let test_cnf = sudoku_rules_two_ones_one_lone_some(structure_sudoku_0_clues());

    println!("{:?}", test_cnf.len())
}

fn test_sud_0_clues_2l1s1o() {
    let test_cnf = sudoku_rules_two_somes_one_lone_one(structure_sudoku_0_clues());

    println!("{:?}", test_cnf.len())
}

fn test_sud_0_clues_3l1s() {
    let test_cnf = sudoku_rules_three_somes_one_lone(structure_sudoku_0_clues());

    println!("{:?}", test_cnf.len())
}

fn bench_sudoku(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sudoku 0 Clues");
    group.bench_function("Four Ones", |b| b.iter(|| sudoku_rules_four_ones(black_box(structure_sudoku_0_clues()))));
    group.bench_function("One Some", |b| b.iter(|| sudoku_rules_two_ones_one_lone_some(black_box(structure_sudoku_0_clues()))));
    group.bench_function("Two Somes", |b| b.iter(|| sudoku_rules_two_somes_one_lone_one(black_box(structure_sudoku_0_clues()))));
    group.bench_function("Three Somes", |b| b.iter(|| sudoku_rules_three_somes_one_lone(black_box(structure_sudoku_0_clues()))));

    group.finish();
}
criterion_group!(benches, bench_sudoku);
criterion_main!(benches);
