use criterion::{criterion_group, criterion_main, Criterion};


fn criterion_benchmark(c: &mut Criterion) {
    use ark_std::{UniformRand, ops::Mul};

    use ark_secp256k1::Fr as ScalarField; 
    use ntat::client::*;
    use ntat::server::*;
    use ntat::util::*;

    let mut rng = ark_std::rand::thread_rng();
    let pp = setup(&mut rng);

    // Client KeyGen
    let sk_c = ScalarField::rand(&mut rng);
    let pk_c = pp.g1.mul(sk_c);

    // Server KeyGen
    let sk_s = ScalarField::rand(&mut rng);
    let pk_s = pp.g2.mul(sk_s);

    let rand_state = ScalarField::rand(&mut rng);
    let rand_server_state = ScalarField::rand(&mut rng);
    let mut client = Client::new(&pp, pk_s, rand_state);
    let mut server = Server::new(&pp, pk_c, rand_server_state);

    c.bench_function("Client Query", |b| b.iter(|| client.client_query(&mut rng, &pp, sk_c, pk_s)));

    let query = client.client_query(&mut rng, &pp, sk_c, pk_s);

    c.bench_function("Server Issue", |b| b.iter(|| server.server_issue(&mut rng, &pp, sk_s, pk_c, &query)));
    let response = server.server_issue(&mut rng, &pp, sk_s, pk_c, &query);

    let r = response.unwrap();
    c.bench_function("Client Final", |b| b.iter(|| client.client_final(&r)));
    let token = client.client_final(&r);
    let extracted_token = token.unwrap();

    c.bench_function("Client Prove Redemption Part 1", |b| b.iter(|| client.client_prove_redemption1(&mut rng, &extracted_token, sk_c, pk_s)));
    let proof1 = client.client_prove_redemption1(&mut rng, &extracted_token, sk_c, pk_s);

    c.bench_function("Server Verify Redemption Part 1", |b| b.iter(|| server.server_verify_redemption1(&mut rng, &extracted_token, sk_s, &proof1)));
    let c_ = server.server_verify_redemption1(&mut rng, &extracted_token, sk_s, &proof1);

    c.bench_function("Client Prove Redemption Part 2", |b| b.iter(|| client.client_prove_redemption2(&mut rng, &extracted_token, sk_c, c_.unwrap())));
    let proof2 = client.client_prove_redemption2(&mut rng, &extracted_token, sk_c, c_.unwrap());

    c.bench_function("Server Verify Redemption Part 2", |b| b.iter(|| server.server_verify_redemption2(&extracted_token, sk_s, &proof2)));
    let verified = server.server_verify_redemption2(&extracted_token, sk_s, &proof2);

}

fn criterion_benchmark_dalek(c: &mut Criterion) {
    use curve25519_dalek_ng::ristretto::RistrettoPoint;
    use curve25519_dalek_ng::scalar::Scalar as ScalarField;
    use rand::rngs::ThreadRng;

    use ntat::server_dalek::*;
    use ntat::client_dalek::*;
    use ntat::util_dalek::*;

    let mut rng = ark_std::rand::thread_rng();
    let pp = setup(&mut rng);

    // Client KeyGen
    let sk_c = ScalarField::random(&mut rng);
    let pk_c = pp.g1*sk_c;

    // Server KeyGen
    let sk_s = ScalarField::random(&mut rng);
    let pk_s = pp.g2*sk_s;

    let rand_state = ScalarField::random(&mut rng);
    let rand_server_state = ScalarField::random(&mut rng);
    let mut client = Client::new(&pp, pk_s, rand_state);
    let mut server = Server::new(&pp, pk_c, rand_server_state);

    c.bench_function("Client Query Dalek", |b| b.iter(|| client.client_query(&mut rng, &pp, sk_c, pk_s)));

    let query = client.client_query(&mut rng, &pp, sk_c, pk_s);

    c.bench_function("Server Issue Dalek", |b| b.iter(|| server.server_issue(&mut rng, &pp, sk_s, pk_c, &query)));
    let response = server.server_issue(&mut rng, &pp, sk_s, pk_c, &query);

    let r = response.unwrap();
    c.bench_function("Client Final Dalek", |b| b.iter(|| client.client_final(&r)));
    let token = client.client_final(&r);
    let extracted_token = token.unwrap();

    c.bench_function("Client Prove Redemption Part 1 Dalek", |b| b.iter(|| client.client_prove_redemption1(&mut rng, &extracted_token, sk_c, pk_s)));
    let proof1 = client.client_prove_redemption1(&mut rng, &extracted_token, sk_c, pk_s);

    c.bench_function("Server Verify Redemption Part 1 Dalek", |b| b.iter(|| server.server_verify_redemption1(&mut rng, &extracted_token, sk_s, &proof1)));
    let c_ = server.server_verify_redemption1(&mut rng, &extracted_token, sk_s, &proof1);

    c.bench_function("Client Prove Redemption Part 2 Dalek", |b| b.iter(|| client.client_prove_redemption2(&mut rng, &extracted_token, sk_c, c_.unwrap())));
    let proof2 = client.client_prove_redemption2(&mut rng, &extracted_token, sk_c, c_.unwrap());

    c.bench_function("Server Verify Redemption Part 2 Dalek", |b| b.iter(|| server.server_verify_redemption2(&extracted_token, sk_s, &proof2)));
    let verified = server.server_verify_redemption2(&extracted_token, sk_s, &proof2);
}


fn criterion_benchmark_pairing(c: &mut Criterion) {
    use ark_std::{UniformRand, ops::Mul};

    use ark_bls12_381::{Fr as ScalarField};


    use ntat::client_pairing::*;
    use ntat::server_pairing::*;
    use ntat::util_pairing::*;

    let mut rng = ark_std::rand::thread_rng();
    let pp = setup(&mut rng);

    // Client KeyGen
    let sk_c = ScalarField::rand(&mut rng);
    let pk_c = pp.g1.mul(sk_c);

    // Server KeyGen
    let sk_s = ScalarField::rand(&mut rng);
    let pk_s = pp.g2.mul(sk_s);

    let rand_state = ScalarField::rand(&mut rng);
    let mut client = Client::new(&pp, pk_s, rand_state);
    let mut server = Server::new(&pp, pk_c, rand_state);

    c.bench_function("Client Query w/Pairing", |b| b.iter(|| client.client_query(&mut rng, &pp, sk_c, pk_s)));

    let query = client.client_query(&mut rng, &pp, sk_c, pk_s);

    c.bench_function("Server Issue w/Pairing", |b| b.iter(|| server_issue(&mut rng, &pp, sk_s, pk_c, &query)));
    let response = server.server_issue(&mut rng, &pp, sk_s, pk_c, &query);

    let r = response.unwrap();
    c.bench_function("Client Final w/Pairing", |b| b.iter(|| client.client_final(&r)));
    let token = client.client_final(&r);
    let extracted_token = token.unwrap();

    c.bench_function("Client Prove Redemption Part 1 w/Pairing", |b| b.iter(|| client.client_prove_redemption1(&mut rng, &extracted_token, sk_c, pk_s)));
    let proof1 = client.client_prove_redemption1(&mut rng, &extracted_token, sk_c, pk_s);

    c.bench_function("Server Verify Redemption Part 1 w/Pairing", |b| b.iter(|| server.server_verify_redemption1(&mut rng, &extracted_token, pk_s, &proof1)));
    let c_ = server.server_verify_redemption1(&mut rng, &extracted_token, pk_s, &proof1);

    c.bench_function("Client Prove Redemption Part 2 w/Pairing", |b| b.iter(|| client.client_prove_redemption2(&mut rng, &extracted_token, sk_c, c_.unwrap())));
    let proof2 = client.client_prove_redemption2(&mut rng, &extracted_token, sk_c, c_.unwrap());

    c.bench_function("Server Verify Redemption Part 2 w/Pairing", |b| b.iter(|| server.server_verify_redemption2(&extracted_token, sk_s, &proof2)));
    let verified = server.server_verify_redemption2(&extracted_token, sk_s, &proof2);

}

fn criterion_benchmark_u_prove(c: &mut Criterion) {

    use curve25519_dalek_ng::ristretto::RistrettoPoint;
    use curve25519_dalek_ng::scalar::Scalar as ScalarField;
    use rand::rngs::ThreadRng;

    use ntat::server_u_prove::*;
    use ntat::client_u_prove::*;
    use ntat::util_u_prove::*;

    let mut rng = ark_std::rand::thread_rng();
    let pp = setup(&mut rng);

    // Client KeyGen
    let sk_c = ScalarField::random(&mut rng);
    let pk_c = pp.gd * sk_c;

    // Server KeyGen
    let sk_s = ScalarField::random(&mut rng);
    let pk_s = pp.g0 * sk_s.invert();

    let pi = ScalarField::random(&mut rng);
    let rand_state = ScalarField::random(&mut rng);
    let mut client = Client::new(&pp, sk_c, pk_c, pi, rand_state);
    let mut server = Server::new(&pp, pk_c, sk_s, pk_s, rand_state);

    c.bench_function("Server Initiate Dalek", |b| b.iter(|| server.server_initiate(&mut rng, &pp)));
    let message = server.server_initiate(&mut rng, &pp);

    c.bench_function("Client Query Dalek", |b| b.iter(|| client.client_query(&mut rng, &pp, pk_s, &message)));
    let sigma_c = client.client_query(&mut rng, &pp, pk_s, &message);

    c.bench_function("Server Issue Dalek", |b| b.iter(|| server.server_issue(sigma_c)));
    let sigma_r = server.server_issue(sigma_c);

    c.bench_function("Client Final Dalek", |b| b.iter(|| server.server_initiate(&mut rng, &pp)));
    let token = client.client_final(&pp, pk_s, sigma_r);

    let (token, witness) = token.unwrap();

    c.bench_function("Client Prove Redemption1 U-Prove", |b| b.iter(|| client.client_prove_redemption1(&mut rng, &pp, &token)));
    let proof1 = client.client_prove_redemption1(&mut rng, &pp, &token);

    c.bench_function("Server Verify Redemption1 U-Prove", |b| b.iter(|| server.server_verify_redemption1(&mut rng, &pp, &proof1)));
    let a = server.server_verify_redemption1(&mut rng, &pp, &proof1);
    let a = a.unwrap();

    c.bench_function("Client Prove Redemption2 U-Prove", |b| b.iter(|| client.client_prove_redemption2(&token, a)));
    let proof2 = client.client_prove_redemption2(&token, a);

    c.bench_function("Server Verify Redemption2 U-Prove", |b| b.iter(|| server.server_verify_redemption2(&token, &pp, &proof2)));
    let verified = server.server_verify_redemption2(&token, &pp, &proof2);

    assert_eq!(verified, true);

}


fn criterion_benchmark_scalar_dalek(c: &mut Criterion) {
    use curve25519_dalek_ng::ristretto::RistrettoPoint;
    use curve25519_dalek_ng::scalar::Scalar as ScalarField;
    use rand::rngs::ThreadRng;

    let mut rng = ark_std::rand::thread_rng();
    // Client KeyGen
    let sk_c = ScalarField::random(&mut rng);
    let sk2 = ScalarField::random(&mut rng);
    let g = RistrettoPoint::random(&mut rng);
    let g2 = RistrettoPoint::random(&mut rng);

    c.bench_function("Scalar Dalek", |b| b.iter(|| g*sk_c));

    c.bench_function("Add Dalek", |b| b.iter(|| g + g2));

    c.bench_function("Scalar Mult  Dalek", |b| b.iter(|| sk_c*sk2));

    c.bench_function("Scalar Inv  Dalek", |b| b.iter(|| sk_c.invert()));
}


criterion_group!(benches, criterion_benchmark_dalek, criterion_benchmark_u_prove);
//criterion_group!(benches, criterion_benchmark_u_prove);
criterion_main!(benches);