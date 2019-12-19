

//use accumulator::Witness;
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

const SET_SIZE : usize = 100000;
fn main() {
    let mut rng = rand::thread_rng();
    println!("Createing Vector of size {}", SET_SIZE);
    let set = random_vector(SET_SIZE);
    let target = set[rng.gen_range(0, set.len())];
    println!("Target is {}",target);
    let mut acc = Accumulator::<Rsa2048, usize>::empty();
    //println!("Set: {:?}",set);
    println!("Creating vector without target");
    let set2 : Vec<usize> = set.clone().into_iter().filter (|&x| x!=target).collect();
    let mut acc_2 = Accumulator::<Rsa2048, usize>::empty();
    let progress_bar = ProgressBar::new(set.len().try_into().unwrap());
    progress_bar.set_prefix("Creating full accumlator :");
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{prefix} {wide_bar} {pos}/{len} {elapsed_precise} {eta_precise}"),
    );
    for item in &set{
        acc = acc.clone().add(&[*item]);
        progress_bar.inc(1);
    }
    progress_bar.finish();
    println!("Publish full set accumlator {:?}", acc);
    let progress_bar = ProgressBar::new(set2.len().try_into().unwrap());
    progress_bar.set_prefix("Creating accumlator without target :");
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{prefix} {wide_bar} {pos}/{len} {elapsed_precise} {eta_precise}"),
    );
    for item in set2{
        acc_2 = acc_2.clone().add(&[item]);
        progress_bar.inc(1);
    }
    progress_bar.finish();
    //println!("Publish partial set accumlator {:?}", acc_2);
    println!("Creating proof");
    let (_acc_new, proof)  = acc_2.add_with_proof(&[target]);
    println!("Sending proof {:?}", proof);
    let answer = acc.verify_membership(&target,&proof);
    println!("Verification is {}",answer);


    /*println!("Creating subset witness");
    let spinner = ProgressBar::new_spinner();
    spinner.set_prefix("Creating Witness:");
    spinner
        .set_style(ProgressStyle::default_bar().template("{prefix} {spinner} {elapsed_precise}"));
    spinner.enable_steady_tick(100);
    let witness = Witness(Accumulator::<Rsa2048, usize>::empty()).compute_subset_witness(&set,&[target]).unwrap();
    spinner.finish();


    println!("Deleteing target");
    acc = acc.clone().delete(&[(target,witness.clone())]).unwrap();


    println!("Creating proof");
    let (acc_new, proof)  = acc.clone().add_with_proof(&[target]);

    println!("Sending proof {:?}", proof);
    let answer = acc_new.verify_membership(&target,&proof);
    println!("Verification is {}",answer); */

}
