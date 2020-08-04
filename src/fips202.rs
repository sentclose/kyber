

const SHAKE128_RATE: usize = 168;
const SHAKE256_RATE: usize = 136;
const SHA3_256_RATE: usize = 136;
const SHA3_512_RATE: usize =  72;

const NROUNDS: usize = 24;

fn rol(a: u64, offset: u64) -> u64 
{
  (a << offset) ^ (a >> (64-offset))
}


/*************************************************
* Name:        load64
*
* Description: Load 8 bytes into uint64_t in little-endian order
*
* Arguments:   - const unsigned char *x: pointer to input byte array
*
* Returns the loaded 64-bit unsigned integer
**************************************************/
pub fn load64(x: &[u8]) -> u64
{
  let mut r = 0u64;
  for i in x.iter().take(8) {
    r |= (*i as u64) << (8 * i);
  }
  r
}


/*************************************************
* Name:        store64
*
* Description: Store a 64-bit integer to a byte array in little-endian order
*
* Arguments:   - uint8_t *x: pointer to the output byte array
*              - uint64_t u: input 64-bit unsigned integer
**************************************************/
pub fn store64(x: &mut[u8], mut u: u64)
{
  for i in 0..8 {
    x[i] = u as u8;
    u >>= 8;
  }
}

/* Keccak round constants */

const KECCAKF_ROUNDCONSTANTS: [u64; NROUNDS] = [
  0x0000000000000001u64,
  0x0000000000008082u64,
  0x800000000000808au64,
  0x8000000080008000u64,
  0x000000000000808bu64,
  0x0000000080000001u64,
  0x8000000080008081u64,
  0x8000000000008009u64,
  0x000000000000008au64,
  0x0000000000000088u64,
  0x0000000080008009u64,
  0x000000008000000au64,
  0x000000008000808bu64,
  0x800000000000008bu64,
  0x8000000000008089u64,
  0x8000000000008003u64,
  0x8000000000008002u64,
  0x8000000000000080u64,
  0x000000000000800au64,
  0x800000008000000au64,
  0x8000000080008081u64,
  0x8000000000008080u64,
  0x0000000080000001u64,
  0x8000000080008008u64
];


/*************************************************
* Name:        KeccakF1600_StatePermute
*
* Description: The Keccak F1600 Permutation
*
* Arguments:   - uint64_t * state: pointer to in/output Keccak state
**************************************************/
pub fn keccakf1600_statepermute(state: &mut[u64])
{
  let (mut Aba, mut Abe, mut Abi, mut Abo, mut Abu) = (0u64,0u64, 0u64,0u64,0u64,);
  let (mut Aga, mut Age, mut Agi, mut Ago, mut Agu) = (0u64,0u64, 0u64,0u64,0u64,);
  let (mut Aka, mut Ake, mut Aki, mut Ako, mut Aku) = (0u64,0u64, 0u64,0u64,0u64,);
  let (mut Ama, mut Ame, mut Ami, mut Amo, mut Amu) = (0u64,0u64, 0u64,0u64,0u64,);
  let (mut Asa, mut Ase, mut Asi, mut Aso, mut Asu) = (0u64,0u64, 0u64,0u64,0u64,);
  let (mut BCa, mut BCe, mut BCi, mut BCo, mut BCu) = (0u64,0u64, 0u64,0u64,0u64,);
  let (mut Da, mut De, mut Di, mut Do, mut Du) = (0u64,0u64, 0u64,0u64,0u64,);
  let (mut Eba, mut Ebe, mut Ebi, mut Ebo, mut Ebu) = (0u64,0u64, 0u64,0u64,0u64,);
  let (mut Ega, mut Ege, mut Egi, mut Ego, mut Egu) = (0u64,0u64, 0u64,0u64,0u64,);
  let (mut Eka, mut Eke, mut Eki, mut Eko, mut Eku) = (0u64,0u64, 0u64,0u64,0u64,);
  let (mut Ema, mut Eme, mut Emi, mut Emo, mut Emu) = (0u64,0u64, 0u64,0u64,0u64,);
  let (mut Esa, mut Ese, mut Esi, mut Eso, mut Esu) = (0u64,0u64, 0u64,0u64,0u64,);

  //copyFromState(A, state)
 let mut Aba = state[ 0];
 let mut Abe = state[ 1];
 let mut Abi = state[ 2];
 let mut Abo = state[ 3];
 let mut Abu = state[ 4];
 let mut Aga = state[ 5];
 let mut Age = state[ 6];
 let mut Agi = state[ 7];
 let mut Ago = state[ 8];
 let mut Agu = state[ 9];
 let mut Aka = state[10];
 let mut Ake = state[11];
 let mut Aki = state[12];
 let mut Ako = state[13];
 let mut Aku = state[14];
 let mut Ama = state[15];
 let mut Ame = state[16];
 let mut Ami = state[17];
 let mut Amo = state[18];
 let mut Amu = state[19];
 let mut Asa = state[20];
 let mut Ase = state[21];
 let mut Asi = state[22];
 let mut Aso = state[23];
 let mut Asu = state[24];

  for round in (0..NROUNDS).step_by(2) {
    //    prepareTheta
    BCa = Aba^Aga^Aka^Ama^Asa;
    BCe = Abe^Age^Ake^Ame^Ase;
    BCi = Abi^Agi^Aki^Ami^Asi;
    BCo = Abo^Ago^Ako^Amo^Aso;
    BCu = Abu^Agu^Aku^Amu^Asu;

    //thetaRhoPiChiIotaPrepareTheta(round  , A, E)
    Da = BCu^rol(BCe, 1);
    De = BCa^rol(BCi, 1);
    Di = BCe^rol(BCo, 1);
    Do = BCi^rol(BCu, 1);
    Du = BCo^rol(BCa, 1);

    Aba ^= Da;
    BCa = Aba;
    Age ^= De;
    BCe = rol(Age, 44);
    Aki ^= Di;
    BCi = rol(Aki, 43);
    Amo ^= Do;
    BCo = rol(Amo, 21);
    Asu ^= Du;
    BCu = rol(Asu, 14);
    Eba =   BCa ^((!BCe)&  BCi );
    Eba ^= KECCAKF_ROUNDCONSTANTS[round];
    Ebe =   BCe ^((!BCi)&  BCo );
    Ebi =   BCi ^((!BCo)&  BCu );
    Ebo =   BCo ^((!BCu)&  BCa );
    Ebu =   BCu ^((!BCa)&  BCe );

    Abo ^= Do;
    BCa = rol(Abo, 28);
    Agu ^= Du;
    BCe = rol(Agu, 20);
    Aka ^= Da;
    BCi = rol(Aka,  3);
    Ame ^= De;
    BCo = rol(Ame, 45);
    Asi ^= Di;
    BCu = rol(Asi, 61);
    Ega =   BCa ^((!BCe)&  BCi );
    Ege =   BCe ^((!BCi)&  BCo );
    Egi =   BCi ^((!BCo)&  BCu );
    Ego =   BCo ^((!BCu)&  BCa );
    Egu =   BCu ^((!BCa)&  BCe );

    Abe ^= De;
    BCa = rol(Abe,  1);
    Agi ^= Di;
    BCe = rol(Agi,  6);
    Ako ^= Do;
    BCi = rol(Ako, 25);
    Amu ^= Du;
    BCo = rol(Amu,  8);
    Asa ^= Da;
    BCu = rol(Asa, 18);
    Eka =   BCa ^((!BCe)&  BCi );
    Eke =   BCe ^((!BCi)&  BCo );
    Eki =   BCi ^((!BCo)&  BCu );
    Eko =   BCo ^((!BCu)&  BCa );
    Eku =   BCu ^((!BCa)&  BCe );

    Abu ^= Du;
    BCa = rol(Abu, 27);
    Aga ^= Da;
    BCe = rol(Aga, 36);
    Ake ^= De;
    BCi = rol(Ake, 10);
    Ami ^= Di;
    BCo = rol(Ami, 15);
    Aso ^= Do;
    BCu = rol(Aso, 56);
    Ema =   BCa ^((!BCe)&  BCi );
    Eme =   BCe ^((!BCi)&  BCo );
    Emi =   BCi ^((!BCo)&  BCu );
    Emo =   BCo ^((!BCu)&  BCa );
    Emu =   BCu ^((!BCa)&  BCe );

    Abi ^= Di;
    BCa = rol(Abi, 62);
    Ago ^= Do;
    BCe = rol(Ago, 55);
    Aku ^= Du;
    BCi = rol(Aku, 39);
    Ama ^= Da;
    BCo = rol(Ama, 41);
    Ase ^= De;
    BCu = rol(Ase,  2);
    Esa =   BCa ^((!BCe)&  BCi );
    Ese =   BCe ^((!BCi)&  BCo );
    Esi =   BCi ^((!BCo)&  BCu );
    Eso =   BCo ^((!BCu)&  BCa );
    Esu =   BCu ^((!BCa)&  BCe );

    //    prepareTheta
    BCa = Eba^Ega^Eka^Ema^Esa;
    BCe = Ebe^Ege^Eke^Eme^Ese;
    BCi = Ebi^Egi^Eki^Emi^Esi;
    BCo = Ebo^Ego^Eko^Emo^Eso;
    BCu = Ebu^Egu^Eku^Emu^Esu;

    //thetaRhoPiChiIotaPrepareTheta(round+1, E, A)
    Da = BCu^rol(BCe, 1);
    De = BCa^rol(BCi, 1);
    Di = BCe^rol(BCo, 1);
    Do = BCi^rol(BCu, 1);
    Du = BCo^rol(BCa, 1);

    Eba ^= Da;
    BCa = Eba;
    Ege ^= De;
    BCe = rol(Ege, 44);
    Eki ^= Di;
    BCi = rol(Eki, 43);
    Emo ^= Do;
    BCo = rol(Emo, 21);
    Esu ^= Du;
    BCu = rol(Esu, 14);
    Aba =   BCa ^((!BCe)&  BCi );
    Aba ^= KECCAKF_ROUNDCONSTANTS[round+1];
    Abe =   BCe ^((!BCi)&  BCo );
    Abi =   BCi ^((!BCo)&  BCu );
    Abo =   BCo ^((!BCu)&  BCa );
    Abu =   BCu ^((!BCa)&  BCe );

    Ebo ^= Do;
    BCa = rol(Ebo, 28);
    Egu ^= Du;
    BCe = rol(Egu, 20);
    Eka ^= Da;
    BCi = rol(Eka, 3);
    Eme ^= De;
    BCo = rol(Eme, 45);
    Esi ^= Di;
    BCu = rol(Esi, 61);
    Aga =   BCa ^((!BCe)&  BCi );
    Age =   BCe ^((!BCi)&  BCo );
    Agi =   BCi ^((!BCo)&  BCu );
    Ago =   BCo ^((!BCu)&  BCa );
    Agu =   BCu ^((!BCa)&  BCe );

    Ebe ^= De;
    BCa = rol(Ebe, 1);
    Egi ^= Di;
    BCe = rol(Egi, 6);
    Eko ^= Do;
    BCi = rol(Eko, 25);
    Emu ^= Du;
    BCo = rol(Emu, 8);
    Esa ^= Da;
    BCu = rol(Esa, 18);
    Aka =   BCa ^((!BCe)&  BCi );
    Ake =   BCe ^((!BCi)&  BCo );
    Aki =   BCi ^((!BCo)&  BCu );
    Ako =   BCo ^((!BCu)&  BCa );
    Aku =   BCu ^((!BCa)&  BCe );

    Ebu ^= Du;
    BCa = rol(Ebu, 27);
    Ega ^= Da;
    BCe = rol(Ega, 36);
    Eke ^= De;
    BCi = rol(Eke, 10);
    Emi ^= Di;
    BCo = rol(Emi, 15);
    Eso ^= Do;
    BCu = rol(Eso, 56);
    Ama =   BCa ^((!BCe)&  BCi );
    Ame =   BCe ^((!BCi)&  BCo );
    Ami =   BCi ^((!BCo)&  BCu );
    Amo =   BCo ^((!BCu)&  BCa );
    Amu =   BCu ^((!BCa)&  BCe );

    Ebi ^= Di;
    BCa = rol(Ebi, 62);
    Ego ^= Do;
    BCe = rol(Ego, 55);
    Eku ^= Du;
    BCi = rol(Eku, 39);
    Ema ^= Da;
    BCo = rol(Ema, 41);
    Ese ^= De;
    BCu = rol(Ese, 2);
    Asa =   BCa ^((!BCe)&  BCi );
    Ase =   BCe ^((!BCi)&  BCo );
    Asi =   BCi ^((!BCo)&  BCu );
    Aso =   BCo ^((!BCu)&  BCa );
    Asu =   BCu ^((!BCa)&  BCe );
  } 

  state[ 0] = Aba;
  state[ 1] = Abe;
  state[ 2] = Abi;
  state[ 3] = Abo;
  state[ 4] = Abu;
  state[ 5] = Aga;
  state[ 6] = Age;
  state[ 7] = Agi;
  state[ 8] = Ago;
  state[ 9] = Agu;
  state[10] = Aka;
  state[11] = Ake;
  state[12] = Aki;
  state[13] = Ako;
  state[14] = Aku;
  state[15] = Ama;
  state[16] = Ame;
  state[17] = Ami;
  state[18] = Amo;
  state[19] = Amu;
  state[20] = Asa;
  state[21] = Ase;
  state[22] = Asi;
  state[23] = Aso;
  state[24] = Asu;
}

fn MIN(a: u64, b: u64) -> u64 {
  std::cmp::min(a, b)
}


/*************************************************
* Name:        keccak_absorb
*
* Description: Absorb step of Keccak;
*              non-incremental, starts by zeroeing the state.
*
* Arguments:   - uint64_t *s:             pointer to (uninitialized) output Keccak state
*              - unsigned int r:          rate in bytes (e.g., 168 for SHAKE128)
*              - const unsigned char *m:  pointer to input to be absorbed into s
*              - unsigned long long mlen: length of input in bytes
*              - unsigned char p:         domain-separation byte for different Keccak-derived functions
**************************************************/
pub fn keccak_absorb(s: &mut[u64], mut r: usize, m: &[u8], mut mlen: u64, p: u8)
{
  let mut t = [0u8; 200];

  // Zero State
  for i in 0..25 {
    s[i] = 0;
  }

  let mut idx = 0usize;
  while mlen >= r as u64 {
    for i in 0..(r/8) {
      s[i] ^= load64(&m[idx + 8 * i..]);
    }
    keccakf1600_statepermute(s);
    mlen -= r as u64;
    idx += r;
  }

  for i in 0..r {
    t[i] = 0;
  }
  for i in 0..mlen as usize {
    t[i] = m[i];
  }
  t[mlen as usize] = p;
  t[r - 1] |= 128;
  for i in 0..(r/8) {
    s[i] ^= load64(&t[8 * i..]);
  }
}


/*************************************************
* Name:        keccak_squeezeblocks
*
* Description: Squeeze step of Keccak. Squeezes full blocks of r bytes each.
*              Modifies the state. Can be called multiple times to keep squeezing,
*              i.e., is incremental.
*
* Arguments:   - unsigned char *h:               pointer to output blocks
*              - unsigned long long int nblocks: number of blocks to be squeezed (written to h)
*              - uint64_t *s:                    pointer to in/output Keccak state
*              - unsigned int r:                 rate in bytes (e.g., 168 for SHAKE128)
**************************************************/
pub fn keccak_squeezeblocks(h: &mut[u8], mut nblocks: u64, s: &mut [u64], r: usize)
{
  let mut idx = 0usize;
  while nblocks > 0 {
    keccakf1600_statepermute(s);
    for i in 0..(r>>3) {
      store64(&mut h[idx+8*i..], s[i])
    }
    idx += r;
    nblocks -= 1;
  }
}


/*************************************************
* Name:        shake128_absorb
*
* Description: Absorb step of the SHAKE128 XOF.
*              non-incremental, starts by zeroeing the state.
*
* Arguments:   - uint64_t *s:                     pointer to (uninitialized) output Keccak state
*              - const unsigned char *input:      pointer to input to be absorbed into s
*              - unsigned long long inputByteLen: length of input in bytes
**************************************************/
pub fn shake128_absorb(s: &mut[u64], input: &[u8], inputByteLen: u64)
{
  keccak_absorb(s, SHAKE128_RATE, input, inputByteLen, 0x1F);
}


/*************************************************
* Name:        shake128_squeezeblocks
*
* Description: Squeeze step of SHAKE128 XOF. Squeezes full blocks of SHAKE128_RATE bytes each.
*              Modifies the state. Can be called multiple times to keep squeezing,
*              i.e., is incremental.
*
* Arguments:   - unsigned char *output:      pointer to output blocks
*              - unsigned long long nblocks: number of blocks to be squeezed (written to output)
*              - uint64_t *s:                pointer to in/output Keccak state
**************************************************/
pub fn shake128_squeezeblocks(output: &mut[u8], nblocks: u64, s: &mut[u64])
{
  keccak_squeezeblocks(output, nblocks, s, SHAKE128_RATE);
}


/*************************************************
* Name:        shake256
*
* Description: SHAKE256 XOF with non-incremental API
*
* Arguments:   - unsigned char *output:      pointer to output
*              - unsigned long long outlen:  requested output length in bytes
               - const unsigned char *input: pointer to input
               - unsigned long long inlen:   length of input in bytes
**************************************************/
pub fn shake256(output: &mut[u8], mut outlen: u64, input: &[u8], inlen: u64)
{
  let mut s = [0u64; 25];
  let mut t = [0u8; SHAKE256_RATE];
  let nblocks = outlen/SHAKE256_RATE as u64;
  
    /* Absorb input */
    keccak_absorb(&mut s, SHAKE256_RATE, input, inlen, 0x1F);

    /* Squeeze output */
    keccak_squeezeblocks(output, nblocks, &mut s, SHAKE256_RATE);
    let mut idx =0;
    idx += nblocks as usize *SHAKE256_RATE;
    outlen -= nblocks *SHAKE256_RATE as u64;

    if outlen > 0
    {
      keccak_squeezeblocks(&mut t, 1, &mut s, SHAKE256_RATE);
      for i in 0..outlen as usize {
        output[i] = t[i];
      }
    }
}


/*************************************************
* Name:        sha3_256
*
* Description: SHA3-256 with non-incremental API
*
* Arguments:   - unsigned char *output:      pointer to output (32 bytes)
*              - const unsigned char *input: pointer to input
*              - unsigned long long inlen:   length of input in bytes
**************************************************/
pub fn sha3_256(output: &mut [u8], input: &[u8], inlen: usize)
{
  let mut s =[0u64; 25];
  let mut t = [0u8; SHA3_256_RATE];

    /* Absorb input */
    keccak_absorb(&mut s, SHA3_256_RATE, input, inlen as u64, 0x06);

    /* Squeeze output */
    keccak_squeezeblocks(&mut t, 1, &mut s, SHA3_256_RATE);

    for i in 0..32 {
      output[i] = t[i];
    }
}

/*************************************************
* Name:        sha3_512
*
* Description: SHA3-512 with non-incremental API
*
* Arguments:   - unsigned char *output:      pointer to output (64 bytes)
*              - const unsigned char *input: pointer to input
*              - unsigned long long inlen:   length of input in bytes
**************************************************/
pub fn sha3_512(output: &mut [u8], input: &[u8], inlen: usize) {
  let mut s =[0u64; 25];
  let mut t = [0u8; SHA3_512_RATE];

    /* Absorb input */
    keccak_absorb(&mut s, SHA3_512_RATE, input, inlen as u64, 0x06);

    /* Squeeze output */
    keccak_squeezeblocks(&mut t, 1, &mut s, SHA3_512_RATE);

    for i in 0..64 {
      output[i] = t[i];
    }
}

