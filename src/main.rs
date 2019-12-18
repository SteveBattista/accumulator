

use accumulator::Witness;
use std::convert::TryInto;
use accumulator::Accumulator;
use accumulator::group::Rsa2048;
use rand::Rng;

use indicatif::ProgressBar;
use indicatif::ProgressStyle;

fn random_vector (lenght:usize) -> Vec<usize>{
    let mut answer :Vec<usize>= Vec::new();
    let mut rng = rand::thread_rng();
    for _item in 0..lenght {
        answer.push(rng.gen::<usize>())
    }
    answer
}

const SET_SIZE : usize = 10000;
fn main() {
    let mut rng = rand::thread_rng();
    println!("Createing Vector of size {}", SET_SIZE);
    let set = random_vector(SET_SIZE);
    let target = set[rng.gen_range(0, set.len())];
    println!("Target is {}",target);
    println!("Creating vector without target");
    let set2 : Vec<usize> = set.clone().into_iter().filter (|&x| x!=target).collect();
    let mut acc = Accumulator::<Rsa2048, usize>::empty();
    let mut acc_2 = Accumulator::<Rsa2048, usize>::empty();
    let progress_bar = ProgressBar::new(set.len().try_into().unwrap());
    progress_bar.set_prefix("Creating full accumlator");
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
    progress_bar.set_prefix("Creating accumlator without target");
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{prefix} {wide_bar} {pos}/{len} {elapsed_precise} {eta_precise}"),
    );
    for item in set2{
        acc_2 = acc_2.clone().add(&[item]);
        progress_bar.inc(1);
    }
    progress_bar.finish();
    println!("Creating proof");
    let (_acc_new, proof)  = acc_2.add_with_proof(&[target]);
    println!("Sending proof {:?}", proof);
    let answer = acc.verify_membership(&target,&proof);
    println!("Answer {}",answer);


   let witness_multiple = Witness(acc.clone());
   for  (elem,target_witness) in &witness_multiple.compute_individual_witnesses(&[target]){
       if *elem == target {
           println!("Witness_multiple {:?}",witness_multiple);
           println!("Witness: {:?}", target_witness);
           println!("Deleteing target");
           acc = acc.clone().delete(&[(target,target_witness.clone())]).unwrap();
       }
    }
   println!("Creating proof");
   let (_acc_new, proof)  = acc.clone().add_with_proof(&[target]);
   println!("Sending proof {:?}", proof);
   let answer = acc.verify_membership(&target,&proof);
   println!("Answer {}",answer);

}



//println!("Acc: {:?}\nProof:{:?}",acc,proof);

// A network participant who sees (acc, proof, and ["dog", "cat"]) can verify that the update
// was formed correctly ...
//assert!(acc.verify_membership_batch(&set, &proof));

// ... and trying to verify something that has not been accumulated will fail.
//assert!(!acc.verify_membership(&"cow", &proof));=
