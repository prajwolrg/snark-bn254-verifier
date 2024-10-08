use bn::{pairing, pairing_batch, AffineG1, AffineG2, Fr, Gt, G1, G2};

use super::error::Groth16Error;

#[derive(Clone, PartialEq)]
pub struct Groth16G1 {
    pub alpha: AffineG1,
    pub beta: AffineG1,
    pub delta: AffineG1,
    pub k: Vec<AffineG1>,
}

#[derive(Clone, PartialEq)]
pub struct Groth16G2 {
    pub beta: AffineG2,
    pub delta: AffineG2,
    pub gamma: AffineG2,
}

#[derive(Clone, PartialEq)]
pub struct PedersenVerifyingKey {
    pub g: AffineG2,
    pub g_root_sigma_neg: AffineG2,
}

#[derive(Clone, PartialEq)]
pub struct Groth16VerifyingKey {
    pub g1: Groth16G1,
    pub g2: Groth16G2,
    pub commitment_key: PedersenVerifyingKey,
    pub public_and_commitment_committed: Vec<Vec<u32>>,
}

#[allow(dead_code)]
pub struct Groth16Proof {
    pub ar: AffineG1,
    pub krs: AffineG1,
    pub bs: AffineG2,
    pub commitments: Vec<AffineG1>,
    pub commitment_pok: AffineG1,
}

#[derive(Clone, PartialEq)]
pub struct PreparedVerifyingKey {
    pub vk: Groth16VerifyingKey,
    pub alpha_g1_beta_g2: Gt,
    pub gamma_g2_neg_pc: G2,
    pub delta_g2_neg_pc: G2,
}

fn process_vk(vk: &Groth16VerifyingKey) -> Result<PreparedVerifyingKey, Groth16Error> {
    Ok(PreparedVerifyingKey {
        vk: vk.clone(),
        alpha_g1_beta_g2: pairing(vk.g1.alpha.into(), vk.g2.beta.into()),
        gamma_g2_neg_pc: (-vk.g2.gamma).into(),
        delta_g2_neg_pc: (-vk.g2.delta).into(),
    })
}

fn prepare_inputs(pvk: PreparedVerifyingKey, public_inputs: &[Fr]) -> Result<G1, Groth16Error> {
    if (public_inputs.len() + 1) != pvk.vk.g1.k.len() {
        return Err(Groth16Error::PrepareInputsFailed);
    }

    Ok(public_inputs
        .iter()
        .zip(pvk.vk.g1.k.iter().skip(1))
        .fold(pvk.vk.g1.k[0], |acc, (i, b)| acc + (*b * *i))
        .into())
}

pub fn verify_groth16(
    vk: &Groth16VerifyingKey,
    proof: &Groth16Proof,
    public_inputs: &[Fr],
) -> Result<bool, Groth16Error> {
    let pvk = process_vk(vk)?;
    let qap = pairing_batch(&[
        (proof.ar.into(), proof.bs.into()),
        (
            prepare_inputs(pvk.clone(), public_inputs)?,
            pvk.gamma_g2_neg_pc,
        ),
        (proof.krs.into(), pvk.delta_g2_neg_pc),
    ]);

    let exp = qap
        .final_exponentiation()
        .ok_or(Groth16Error::UnexpectedIdentity)?;

    Ok(exp == pvk.alpha_g1_beta_g2)
}
