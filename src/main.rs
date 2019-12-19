

//use accumulator::Witness;
use accumulator::Witness;
use std::convert::TryInto;
use accumulator::Accumulator;
use accumulator::group::Rsa2048;
use rand::Rng;

use indicatif::ProgressBar;
use indicatif::ProgressStyle;

fn random_vector (lenth:usize) -> Vec<usize>{
    let mut answer :Vec<usize>= Vec::new();
    let mut rng = rand::thread_rng();
    for _item in 0..lenth {
        answer.push(rng.gen::<usize>())
    }
    answer
}

fn add_delete (acc: Accumulator::<Rsa2048, usize>,set: &[usize], target: usize ){
    println!("Creating subset witness");
    let spinner = ProgressBar::new_spinner();
    spinner.set_prefix("Creating Witness:");
    spinner
        .set_style(ProgressStyle::default_bar().template("{prefix} {spinner} {elapsed_precise}"));
    spinner.enable_steady_tick(100);
    let witness = Witness(Accumulator::<Rsa2048, usize>::empty()).compute_subset_witness(&set,&[target]).unwrap();
    spinner.finish();


    println!("Deleteing target");
    let acc_with_delete = acc.clone().delete(&[(target,witness.clone())]).unwrap();


    println!("Creating proof");
    let (acc_new, proof)  = acc_with_delete.clone().add_with_proof(&[target]);

    println!("Sending proof {:?}", proof);
    let answer = acc_new.verify_membership(&target,&proof);
    println!("Verification is {}",answer);
}

fn add_to_subset(acc: Accumulator::<Rsa2048, usize>,set: &[usize], target: usize ){
    println!("Creating vector without target");
    let set2 : Vec<usize> = set.into_iter().filter (|&x| *x!=target).cloned().collect();
    let mut acc_2 = Accumulator::<Rsa2048, usize>::empty();

    let progress_bar = ProgressBar::new(set2.len().try_into().unwrap());
    progress_bar.set_prefix("Creating accumlator without target :");
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{prefix} {wide_bar} {pos}/{len} {elapsed_precise} {eta_precise}"),
    );
    for item in set2{
        acc_2 = acc_2.add(&[item]);
        progress_bar.inc(1);
    }
    progress_bar.finish();

    println!("Creating proof");
    let (acc_new, proof)  = acc_2.clone().add_with_proof(&[target]);
    println!("Publish full set accumlator ACC_NEW {:?}", acc_new);
    println!("Sending proof #1 {:?}", proof);
    let answer = acc.verify_membership(&target,&proof);
    println!("Verification is {}",answer);
}

const SET_SIZE : usize = 1000;
fn main() {
    let mut rng = rand::thread_rng();
    println!("Createing Vector of size {}", SET_SIZE);
    let set = random_vector(SET_SIZE);
    let target = set[rng.gen_range(0, set.len())];
    println!("Target is {}",target);
    let mut acc = Accumulator::<Rsa2048, usize>::empty();
    let progress_bar = ProgressBar::new(set.len().try_into().unwrap());
    progress_bar.set_prefix("Creating full accumlator :");
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{prefix} {wide_bar} {pos}/{len} {elapsed_precise} {eta_precise}"),
    );
    for item in &set{
        acc = acc.add(&[*item]);
        progress_bar.inc(1);
    }
    progress_bar.finish();
    println!("Publish full set accumlator ACC {:?}", acc);

    add_to_subset(acc,&set,target)






}
