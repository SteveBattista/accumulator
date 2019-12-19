mod uuid;

//use accumulator::Witness;
use accumulator::MembershipProof;
use data_encoding::HEXUPPER;
use crate::uuid::Uuid;
use accumulator::NonmembershipProof;
use accumulator::Witness;
use std::convert::TryInto;
use accumulator::Accumulator;
use accumulator::group::Rsa2048;
use rand::Rng;

use indicatif::ProgressBar;
use indicatif::ProgressStyle;

use pretty_bytes::converter::convert;

const SPINNER_INTERVAL : u64 = 1000;
const SET_SIZE : usize = 10_000;

fn acc_to_string(acc:&Accumulator::<Rsa2048, Uuid> )-> String{
    let string = format!("{:?}",*acc);
    HEXUPPER.encode(string.as_bytes())

}


fn membership_to_string(membership: &MembershipProof::<Rsa2048, Uuid> )-> String{
    let string = format!("{:?}",*membership);
    HEXUPPER.encode(string.as_bytes())

}

fn non_membership_to_string(membership: &NonmembershipProof::<Rsa2048, Uuid> )-> String{
    let string = format!("{:?}",*membership);
    HEXUPPER.encode(string.as_bytes())

}

fn random_vector (length:usize) -> Vec<Uuid>{
    let mut answer :Vec<Uuid>= Vec::new();
    let rng = rand::thread_rng();
    for _item in 0..length {
        answer.push(Uuid::random_uuid(rng));
    }
    answer
}

fn create_accumulator_from_set (set: &Vec<Uuid>) -> Accumulator::<Rsa2048, Uuid>{
    let mut acc = Accumulator::<Rsa2048, Uuid>::empty();
    let progress_bar = ProgressBar::new(set.len().try_into().unwrap());
    progress_bar.set_prefix("Creating full accumlator :");
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{prefix} {wide_bar} {pos}/{len} {elapsed_precise} {eta_precise}"),
    );
    for item in set{
        acc = acc.add(&[*item]);
        progress_bar.inc(1);
    }
    progress_bar.finish();
    acc
}


fn select_random_item_from_set ( set: &Vec<Uuid>) -> Uuid {
    let mut rng = rand::thread_rng();
    set[rng.gen_range(0, set.len())]
}

fn select_item_not_in_set( set: &Vec<Uuid>) -> Uuid {
    let mut non_target : Uuid;
    let rng = rand::thread_rng();
    loop {
        non_target = Uuid::random_uuid(rng);
        if !set.contains(&non_target) {
            break;
        }
    }
    non_target
}

fn create_witness_for_target_in_set(set: &[Uuid], target: Uuid) ->Witness<Rsa2048, Uuid> {
    println!("Creating subset witness");
    let spinner = ProgressBar::new_spinner();
    spinner.set_prefix("Creating Witness:");
    spinner
        .set_style(ProgressStyle::default_bar().template("{prefix} {spinner} {elapsed_precise}"));
    spinner.enable_steady_tick(SPINNER_INTERVAL);
    let witness = Witness(Accumulator::<Rsa2048, Uuid>::empty()).compute_subset_witness(&set,&[target]).unwrap();
    spinner.finish();
witness
}

#[allow(dead_code)]
fn create_witness_for_target_in_accumlator(acc: Accumulator::<Rsa2048, Uuid>, target: Uuid) ->Witness<Rsa2048, Uuid> {

    println!("Creating subset witness");
    let spinner = ProgressBar::new_spinner();
    spinner.set_prefix("Creating Witness:");
    spinner
        .set_style(ProgressStyle::default_bar().template("{prefix} {spinner} {elapsed_precise}"));
    spinner.enable_steady_tick(SPINNER_INTERVAL);
    let touple_vec = Witness(acc).compute_individual_witnesses(&[target]);
    spinner.finish();
    let (_elem,witness) = &touple_vec[0];
    witness.clone()
}

fn prove_membership(acc: Accumulator::<Rsa2048, Uuid>,set: &[Uuid], target: Uuid){

    let witness = create_witness_for_target_in_set(set, target);
    // Previous line takes too long. It is faster to use the add to subset it is 3-4 times faster
    //let witness = create_witness_for_target_in_accumlator(acc.clone(),target); This is invalid
    let membership_proof = acc.prove_membership(&[(target,witness)]).unwrap();
    println!("Publishing membership_proof size {}", convert((membership_to_string(&membership_proof).len()*2) as f64));
    let answer = acc.verify_membership(&target,&membership_proof);
    println!("Verification of membership proof is {}",answer);
}

fn create_non_membership_poof(acc: Accumulator::<Rsa2048, Uuid>,set: &[Uuid], target: Uuid)-> NonmembershipProof<Rsa2048, Uuid>{
    let spinner = ProgressBar::new_spinner();
    spinner.set_prefix("Creating Non Memberhsip Proof:");
    spinner
        .set_style(ProgressStyle::default_bar().template("{prefix} {spinner} {elapsed_precise}"));
    spinner.enable_steady_tick(SPINNER_INTERVAL);
    let non_membership_proof = acc.prove_nonmembership(set,&[target]).unwrap();
    spinner.finish();
    non_membership_proof
}

fn prove_nonmembership(acc: Accumulator::<Rsa2048, Uuid>,set: &[Uuid], target: Uuid){

    let non_membership_proof = create_non_membership_poof(acc.clone(),set,target);
    println!("Publishing non_membership_proof size {}",  convert((non_membership_to_string(&non_membership_proof).len()*2) as f64));
    let answer = acc.verify_nonmembership(&[target],&non_membership_proof);
    println!("Verification of non_membership proof is {}",answer);
}

#[allow(dead_code)]
fn add_delete (acc: Accumulator::<Rsa2048, Uuid>,set: &[Uuid], target: Uuid ){
    println!("Creating subset witness");

    let witness = create_witness_for_target_in_set(set, target);

    println!("Deleteing target");
    let acc_with_delete = acc.clone().delete(&[(target,witness.clone())]).unwrap();


    println!("Creating proof");
    let (acc_new, proof)  = acc_with_delete.clone().add_with_proof(&[target]);

    println!("Sending proof {}", membership_to_string(&proof));
    let answer = acc_new.verify_membership(&target,&proof);
    println!("Verification is {}",answer);
}

#[allow(dead_code)]
fn add_to_subset(acc: Accumulator::<Rsa2048, Uuid>,set: &[Uuid], target: Uuid ){

    println!("Creating set without target");

    let set2 : Vec<Uuid> = set.into_iter().filter (|&x| *x!=target).cloned().collect();
    let acc_2 = create_accumulator_from_set(&set2);

    println!("Creating proof");
    let (acc_new, proof)  = acc_2.clone().add_with_proof(&[target]);
    println!("Publish full set accumlator ACC_NEW size {}", convert((acc_to_string(&acc_new).len()*2) as f64));
    println!("Sending proof #1 size {}", convert((membership_to_string(&proof).len()*2) as f64));
    let answer = acc.verify_membership(&target,&proof);
    println!("Verification is {}",answer);
}



fn main() {

    println!("Createing Vector of size {}", SET_SIZE);
    let set = random_vector(SET_SIZE);

    let acc = create_accumulator_from_set (&set);
    println!("Publish full set accumlator ACC size {}", convert((acc_to_string(&acc).len()*2) as f64));

    let target = select_random_item_from_set(&set);
    println!("Target is {}",target);

    prove_membership(acc.clone(),&set, target);

    let non_target = select_item_not_in_set(&set);
    println!("Item not in set: {}",non_target);
    prove_nonmembership(acc.clone(),&set, non_target);







}
