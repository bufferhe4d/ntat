
use ark_std::{UniformRand, ops::Mul};

use ark_bls12_381::{Bls12_381, G1Projective as G, G1Affine as GAffine, G2Projective as G2, Fr as ScalarField, Fq as F};
use ark_ec::short_weierstrass::Projective;
use ark_bls12_381::g1::Config as g1config;
use ark_bls12_381::g2::Config as g2config;
use rand::rngs::ThreadRng;
use sha256::digest;
use ark_ff::fields::{PrimeField, Field};

pub mod client;
pub mod util;

use crate::client::*;
use crate::util::*;

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }
fn main() {
    let mut rng = ark_std::rand::thread_rng();
    let pp = util::setup(&mut rng);
    println!("G1: {:?}", pp.g1);
    println!("G2: {:?}", pp.g2);
    println!("G3: {:?}", pp.g3);
    println!("G4: {:?}", pp.g4);

    // Client KeyGen
    let sk_c = ScalarField::rand(&mut rng);
    let pk_c = pp.g1.mul(sk_c);

    // Server KeyGen
    let sk_s = ScalarField::rand(&mut rng);
    let pk_s = pp.g2.mul(sk_s);


    let rand_state = ScalarField::rand(&mut rng);
    let mut client = Client::new(&pp, pk_s, rand_state);
    let query = client.client_query(&mut rng, &pp, sk_c, pk_s);
    println!("Query: {:?}", query);
    

    let response = server_issue(&mut rng, &pp, sk_s, pk_c, &query);

    match response {
        None => { panic!("Server Issue Failed"); }
        Some(resp) =>  { 
            
            println!("Received response: {:?}", resp); 
            let token = client.client_final(&resp);
            match token {
                None => {panic!("Client Final Failed!");}
                Some(tok) => {println!("Received Token: {:?}", tok);}
            }
        } 
    };

    
}
