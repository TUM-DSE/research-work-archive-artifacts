From Coq Require Import Bool.Bool.
From Coq Require Import Arith.Arith.
From Coq Require Import NArith.
From Coq Require Import Lists.List.
From Coq Require Import Lia.

Import ListNotations.



(* Profile Constants (§6.1) *)
Parameter  PROFILE_COUNT_PCR : nat.     (* number of consecutive PCR implemented on this MARS *)
Parameter  PROFILE_COUNT_TSR : nat.     (* number of consecutive TSR implemented on this MARS *)
Definition PROFILE_COUNT_REG : nat := PROFILE_COUNT_PCR + PROFILE_COUNT_TSR.

Parameter  PROFILE_LEN_DIGEST : nat.    (* length of a digest that can be processed or produced *)
Parameter  PROFILE_LEN_SIGN : nat.      (* length of signature produced by CryptSign() *)
Parameter  PROFILE_LEN_KSYM : nat.      (* length of symmetric key produced by CryptSkdf() if implemented, otherwise 0 *)
Parameter  PROFILE_LEN_KPUB : nat.      (* length of public asymmetric key returned by MARS_PublicRead() if implemented, otherwise 0 *)
Parameter  PROFILE_LEN_KPRV : nat.      (* length of asymmetric key produced by CryptAkdf() if implemented, otherwise 0 *)
Parameter  PROFILE_LEN_XKDF : nat.      (* PROFILE_LEN_KPRV if defined, else PROFILE_LEN_KSYM *)

Parameter  PROFILE_ALG_HASH : nat.      (* TCG-registered algorithm for hashing by CryptHash functions *)
Parameter  PROFILE_ALG_SIGN : nat.      (* TCG-registered algorithm for signing by CryptSign() *)
Parameter  PROFILE_ALG_SKDF : nat.      (* TCG-registered algorithm for symmetric key derivation by CryptSkdf() *)
Parameter  PROFILE_ALG_AKDF : nat.      (* TCG-registered algorithm for asymmetric key derivation by CryptAkdf() *)


(* Property Tags (§8.1.2) *)
Inductive PT : Type :=
| MARS_PT_PCR 
| MARS_PT_TSR
| MARS_PT_LEN_DIGEST 
| MARS_PT_LEN_SIGN 
| MARS_PT_LEN_KSYM 
| MARS_PT_LEN_KPUB 
| MARS_PT_LEN_KPRV
| MARS_PT_ALG_HASH 
| MARS_PT_ALG_SIGN 
| MARS_PT_ALG_SKDF 
| MARS_PT_ALG_AKDF.



(* Response Code (§6.2) *)
Inductive MARS_RC : Type :=
| MARS_RC_SUCCESS           (* Command executed as expected *)
| MARS_RC_IO                (* Input / Output or parsing error, will not be used for this model *)
| MARS_RC_FAILURE           (* Self-testing placed MARS in failure mode or MARS is otherwise inaccessible *)
| MARS_RC_BUFFER            (* Buffer pointer (null or misaligned) or length invalid. It is memory-level, will not be used for this model *) 
| MARS_RC_COMMAND           (* Command not supported *)
| MARS_RC_VALUE             (* Value out of range or incorrect for command *)
| MARS_RC_REG               (* Invalid register index specified *)
| MARS_RC_SEQ.              (* Sequence not started *)



(* Cryptographic Key Labels (§5.5) *)
Inductive Label : Type :=
| MARS_LX   (* eXternal *)
| MARS_LD   (* Derivation Parent *)
| MARS_LU   (* Unrestricted signing *)
| MARS_LR.  (* Restricted attestation *)


Parameter Digest : Type.
Parameter DIGEST_ZERO : Digest.
(* Profile-specified values for TSR at init (§5.4) *)
Parameter TSR_INIT : nat -> Digest.
Parameter TSR_SAMPLE : nat -> Digest.

Parameter Secret : Type. (* PS, DP, keys... *)
Parameter PS_INIT: Secret.

Parameter Context : Type. (* ctx, nonce... *)

Parameter Signature : Type.

Parameter PublicKey : Type.
Parameter ExtractPublicKey : Secret -> PublicKey.

Inductive HashContext : Type :=
| HC_RegSelect : N -> HashContext
| HC_Digest    : Digest -> HashContext
| HC_Context   : Context -> HashContext.

Inductive KdfContext : Type :=
| KC_Digest    : Digest -> KdfContext
| KC_Context   : Context -> KdfContext.


(* ========================================================================= *)
(* State Machine and State Update Functions                                  *)
(* ========================================================================= *)

(* MARS State Machine (§5.3 )
    failure : failure mode flag (§5.3.1)
    PS      : Primary Seed (§5.3.2)
              invariant: PS is persistent and constant.
    DP      : Derivation Parent, volatile (§5.3.3)
    regs    : PCR 0...PROFILE_COUNT_PCR-1; TSR 0...PROFILE_COUNT_TSR-1 (§5.3.4); 
              invariant: length regs = PROFILE_COUNT_REG.
    shc     : hash-sequence context 'shc' (§5.6.2.1), 
              modeled as the list of chunks accumulated so far. *)
Record MARS_STATE : Type := {
  failure : bool;
  PS      : Secret;
  DP      : Secret;
  regs    : list Digest;
  shc     : option (list HashContext) (* none means the sequence has not started*)
}.


Definition set_failure (st : MARS_STATE) (new_failure : bool) : MARS_STATE :=
  {|
    failure := new_failure;
    PS := PS st;
    DP := DP st;
    regs := regs st;
    shc := shc st
  |}.

Definition set_dp (st : MARS_STATE) (new_dp : Secret) : MARS_STATE :=
  {|
    failure := failure st;
    PS := PS st;
    DP := new_dp;
    regs := regs st;
    shc := shc st
  |}.

Fixpoint update (l : list Digest) (i : nat) (x : Digest) : list Digest :=
  match l, i with
  | [], _          => []
  | _ :: r, O      => x :: r
  | h :: r, S i'   => h :: update r i' x
  end.

Definition set_reg (st : MARS_STATE) (i : nat) (x : Digest) : MARS_STATE :=
  {|
    failure := failure st;
    PS := PS st;
    DP := DP st;
    regs := update (regs st) i x;
    shc := shc st
  |}.

Definition set_regs (st : MARS_STATE) (new_regs : list Digest) : MARS_STATE :=
  {|
    failure := failure st;
    PS := PS st;
    DP := DP st;
    regs := new_regs;
    shc := shc st
  |}. 

Definition set_shc (st : MARS_STATE) (new_shc : option (list HashContext)) : MARS_STATE :=
  {|
    failure := failure st;
    PS := PS st;
    DP := DP st;
    regs := regs st;
    shc := new_shc
  |}.


(* "The Start/Update/Complete set of sequence commands should not be interleaved with other MARS
commands. If other commands are used, the sequence MUST be terminated by MARS." (§8.2) *)
Definition set_shc_to_none (st : MARS_STATE) : MARS_STATE := set_shc st None.




(* ========================================================================= *)
(* Support Functions (§5.6)                                                  *)
(* ========================================================================= *)



(* CryptSelfTest(fullTest) (§5.6.1) *)
Parameter CryptSelfTest : bool -> bool.



(* CryptHash-related functions (§5.6.2) *)
(* CryptHashInit(shc) (§5.6.2.1)
   input: nothing
   output: initialized shc *)
Parameter CryptHashInit : list HashContext.
(* CryptHashUpdate(shc, data, len) (§5.6.2.2)
   input: shc, data
   output: updated shc
   in this model, length of data or context will not be modeled, because it is meaningless *)
Parameter CryptHashUpdate : list HashContext -> HashContext -> list HashContext.
(* CryptHashFinal(shc, out) (§5.6.2.3)
   input: shc
   output: resulting digest *)
Parameter CryptHashFinal : list HashContext -> Digest.



(* CryptSign(key, digest) (§5.6.3)
   input: key, degest
   output: signature *)
Parameter CryptSign : Secret -> Digest -> Signature.
(* CryptVerify(key, digest, signature) (§5.6.4)
   input: key, degest, signature
   output: result *)
Parameter CryptVerify : Secret -> Digest -> Signature -> bool.




(* CryptSkdf(child, parent, label, ctx, ctxlen) (§5.6.5) 
   input: parent, label, ctx
   output: child *)
Parameter CryptSkdf : Secret -> Label -> KdfContext -> Secret.
(* CryptAkdf(child, parent, label, ctx, ctxlen) (§5.6.6) 
   input: parent, label, ctx
   output: child *)
Parameter CryptAkdf : Secret -> Label -> KdfContext -> Secret.
(* CryptXkdf is CryptAkdf if CryptAkdf is implemented. Otherwise, CryptXkdf is CryptSkdf. (§5.6.7) *)
Parameter CryptXkdf : Secret -> Label -> KdfContext -> Secret.



(* CryptDpInit() (§5.6.8) 
   input: PS
   output: DP0 *)
Parameter CryptDpInit : Secret -> Secret.




Definition valid_select (regSelect : N) : bool :=
  N.eqb (N.shiftr regSelect (N.of_nat PROFILE_COUNT_REG)) 0.

Definition sel_indices (regSelect : N) : list nat :=
  filter (fun i => N.testbit regSelect (N.of_nat i)) (List.seq 0 PROFILE_COUNT_REG).

Definition sel_values (regSelect : N) (regs : list Digest) : list Digest :=
  map (fun i => nth i regs DIGEST_ZERO) (sel_indices regSelect).

Definition sample (regSelect : N) (regs : list Digest) : list Digest :=
  map (fun i =>
         if (PROFILE_COUNT_PCR <=? i) && (i <? PROFILE_COUNT_REG) && N.testbit regSelect (N.of_nat i)
         then TSR_SAMPLE i
         else nth i regs DIGEST_ZERO)
      (List.seq 0 PROFILE_COUNT_REG).


(* snapshot = CryptHash ( regSelect || REG# || ... || REG# || ctx ) *)
Definition snapshot (st : MARS_STATE) (regSelect : N) (ctx : Context) : MARS_STATE * Digest :=
  let new_regs := sample regSelect (regs st) in
  let new_st := set_regs st new_regs in
  let shc1 := CryptHashInit in
  let shc2 := CryptHashUpdate shc1 (HC_RegSelect regSelect) in
  let shc3 := fold_left CryptHashUpdate  (map (fun i => HC_Digest i)(sel_values regSelect (regs new_st))) shc2 in
  let shc4 := CryptHashUpdate shc3 (HC_Context ctx) in
  (new_st, CryptHashFinal shc4).



(* ========================================================================= *)
(* Initialization (§5.4)                                                     *)
(* ========================================================================= *)


(* _MARS_Init: PCR := 0; TSR := Profile-specified values; failure := false, then a
   full self-test may set it; DP := CryptDpInit().                          *)
Definition mars_init : MARS_STATE :=
  {| failure := negb (CryptSelfTest true);
     PS      := PS_INIT;
     DP      := CryptDpInit PS_INIT;
     regs    := map (fun i => if i <? PROFILE_COUNT_PCR
                                 then DIGEST_ZERO
                                 else TSR_INIT i)
                    (List.seq 0 PROFILE_COUNT_REG);
     shc     := None |}.



(* ========================================================================= *)
(* The command interface (§8)                                                *)
(* ========================================================================= *)


Inductive Command : Type :=
| MARS_SelfTest         (fullTest : bool)                                                       (* §8.1.1 *)
| MARS_CapabilityGet    (pt : PT)                                                               (* §8.1.2 *)
| MARS_SequenceHash                                                                             (* §8.2.1 *)
| MARS_SequenceUpdate   (data : HashContext)                                                    (* §8.2.2 *)
| MARS_SequenceComplete                                                                         (* §8.2.3 *)
| MARS_PcrExtend        (pcrIndex : nat) (dig : Digest)                                         (* §8.3.1 *)
| MARS_RegRead          (regIndex : nat)                                                        (* §8.3.2 *)
| MARS_Derive           (regSelect : N) (ctx : Context)                                         (* §8.4.1 *)
| MARS_DpDerive         (regSelect : N) (ctx : option Context)                                  (* §8.4.2; None = NULL ctx *)
| MARS_PublicRead       (restricted : bool) (ctx : Context)                                     (* §8.4.3 *)
| MARS_Quote            (regSelect : N) (nonce : Context) (ctx : Context)                       (* §8.5.1 *)
| MARS_Sign             (ctx : Context) (dig : Digest)                                          (* §8.5.2 *)
| MARS_SignatureVerify  (restricted : bool) (ctx : Context) (dig : Digest) (sig : Signature).   (* §8.5.3 *)

Definition ReturnType (cmd : Command) : Type :=
    match cmd with
    | MARS_SelfTest _ =>  unit
    | MARS_CapabilityGet _ =>  nat
    | MARS_SequenceHash => unit
    | MARS_SequenceUpdate _ => unit
    | MARS_SequenceComplete => Digest
    | MARS_PcrExtend _ _ => unit
    | MARS_RegRead _ => Digest
    | MARS_Derive _ _ => Secret
    | MARS_DpDerive _ _=> unit
    | MARS_PublicRead _ _ => PublicKey
    | MARS_Quote _ _ _ => Signature
    | MARS_Sign _ _ => Signature
    | MARS_SignatureVerify _ _ _ _ => bool
    end.





(* ========================================================================= *)
(* helpers                                                                   *)
(* ========================================================================= *)

Definition is_seq_cmd (cmd : Command) : bool :=
  match cmd with
  | MARS_SequenceHash | MARS_SequenceUpdate _ | MARS_SequenceComplete => true
  | _ => false
  end.



(* MARS_CapabilityGet (§8.1.2). *)
Definition cap_transition (st : MARS_STATE) (pt : PT) : MARS_STATE * MARS_RC * option nat :=
    match pt with
    | MARS_PT_PCR => (st, MARS_RC_SUCCESS, Some PROFILE_COUNT_PCR)
    | MARS_PT_TSR => (st, MARS_RC_SUCCESS, Some PROFILE_COUNT_TSR)
    | MARS_PT_LEN_DIGEST => (st, MARS_RC_SUCCESS, Some PROFILE_LEN_DIGEST)
    | MARS_PT_LEN_SIGN => (st, MARS_RC_SUCCESS, Some PROFILE_LEN_SIGN)
    | MARS_PT_LEN_KSYM => (st, MARS_RC_SUCCESS, Some PROFILE_LEN_KSYM)
    | MARS_PT_LEN_KPUB => (st, MARS_RC_SUCCESS, Some PROFILE_LEN_KPUB)
    | MARS_PT_LEN_KPRV => (st, MARS_RC_SUCCESS, Some PROFILE_LEN_KPRV)
    | MARS_PT_ALG_HASH => (st, MARS_RC_SUCCESS, Some PROFILE_ALG_HASH)
    | MARS_PT_ALG_SIGN => (st, MARS_RC_SUCCESS, Some PROFILE_ALG_SIGN)
    | MARS_PT_ALG_SKDF => (st, MARS_RC_SUCCESS, Some PROFILE_ALG_SKDF)
    | MARS_PT_ALG_AKDF => (st, MARS_RC_SUCCESS, Some PROFILE_ALG_AKDF)
    end.

                                       
Definition transition (st : MARS_STATE) (cmd : Command) : MARS_STATE * MARS_RC * option (ReturnType cmd) :=
  if failure st then 
    match cmd with
    | MARS_CapabilityGet pt => cap_transition st pt
    | _                     => (st, MARS_RC_FAILURE, None)
    end
  else
    let st := if is_seq_cmd cmd then st else set_shc_to_none st in
    match cmd with

    (* MARS_SelfTest (bool fullTest) *)
    | MARS_SelfTest fullTest =>
        let new_failure := (failure st)||negb(CryptSelfTest(fullTest))in
        (set_failure st new_failure, if new_failure then MARS_RC_FAILURE else MARS_RC_SUCCESS, None)

    (* MARS_RC MARS_CapabilityGet (uint16_t pt, void * cap, uint16_t caplen) *)
    | MARS_CapabilityGet pt => cap_transition st pt

    (* MARS_RC MARS_SequenceHash () *)
    | MARS_SequenceHash => 
        let shc1 := CryptHashInit in
        (set_shc st (Some shc1), MARS_RC_SUCCESS, None)

    (* MARS_RC MARS_SequenceUpdate (const void * in, size_t inlen, void * out, size_t * outlen) *)
    | MARS_SequenceUpdate data =>
        match shc st with
        | None   => (st, MARS_RC_SEQ, None)
        | Some shc' => 
            let shc1 := CryptHashUpdate shc' data in
            (set_shc st (Some shc1), MARS_RC_SUCCESS, None)
        end

    (* MARS_RC MARS_SequenceComplete (void * out, size_t * outlen) *)
    | MARS_SequenceComplete =>
        match shc st with
        | None   => (st, MARS_RC_SEQ, None)
        | Some shc' => 
            let out := CryptHashFinal shc' in 
            (set_shc_to_none st, MARS_RC_SUCCESS, Some out)
        end

    (* MARS_RC MARS_PcrExtend (uint16_t pcrIndex, const void * dig) *)
    | MARS_PcrExtend pcrIndex dig =>
        if pcrIndex <? PROFILE_COUNT_PCR
        then 
            let shc1 := CryptHashInit in
            let shc2 := CryptHashUpdate shc1 (HC_Digest (nth pcrIndex (regs st) DIGEST_ZERO)) in
            let shc3 := CryptHashUpdate shc2 (HC_Digest dig) in
            let x := CryptHashFinal shc3 in
            (set_reg st pcrIndex x, MARS_RC_SUCCESS, None)
        else (st, MARS_RC_REG, None)

    (* MARS_RC MARS_RegRead (uint16_t regIndex, void * dig) *)
    | MARS_RegRead regIndex => 
        if regIndex <? PROFILE_COUNT_REG
        then 
            let dig := nth regIndex (regs st) DIGEST_ZERO in
            (st, MARS_RC_SUCCESS, Some dig)
        else (st, MARS_RC_REG, None)

    (* MARS_RC MARS_Derive (uint32_t regSelect, const void * ctx, uint16_t ctxlen, void * out)
       out := CryptSkdf(DP, MARS_LX, CryptSnapshot(regSelect, ctx)) *)
    | MARS_Derive regSelect ctx =>
        if valid_select regSelect
        then
            let (st', snapshot) := snapshot st regSelect ctx in
            let out := CryptSkdf (DP st') MARS_LX (KC_Digest snapshot) in
            (st', MARS_RC_SUCCESS, Some out)
        else (st, MARS_RC_REG, None)

    (* MARS_RC MARS_DpDerive (uint32_t regSelect, const void * ctx, uint16_t ctxlen)
       ctx = Some c : DP := CryptSkdf(DP, MARS_LD, snapshot)
       ctx = None   : DP := CryptDpInit()  (no snapshot, no TSR sampling) *)
    | MARS_DpDerive regSelect ctx =>
        if valid_select regSelect
        then
            match ctx with
            | Some ctx' =>
                let (st', snapshot) := snapshot st regSelect ctx' in
                let DP := CryptSkdf (DP st') MARS_LD (KC_Digest snapshot) in
                (set_dp st' DP, MARS_RC_SUCCESS, None)
            | None =>
                let DP := (CryptDpInit (PS st)) in
                (set_dp st DP, MARS_RC_SUCCESS, None)
            end
        else (st, MARS_RC_REG, None)

    (* MARS_RC MARS_PublicRead (bool restricted, const void * ctx, uint16_t ctxlen, void * pub) *)
    | MARS_PublicRead restricted ctx => 
        let label := if restricted then MARS_LR else MARS_LU in
        let key := CryptAkdf (DP st) label (KC_Context ctx) in
        let pub := ExtractPublicKey key in
        (st, MARS_RC_SUCCESS, Some pub)
    (* MARS_RC MARS_Quote (uint32_t regSelect, const void * nonce, uint16_t nlen, const void * ctx, uint16_t ctxlen, void * sig)
       (st', snapshot) := CryptSnapshot(st, regSelect, nonce)
       AK   := CryptXkdf(DP, MARS_LR, ctx)
       sig  := CryptSign(AK, snapshot) *)
    | MARS_Quote regSelect nonce ctx =>
        if valid_select regSelect
        then
            let (st', snapshot) := snapshot st regSelect nonce in
            let AK := CryptXkdf (DP st') MARS_LR (KC_Context ctx) in
            let sig := CryptSign AK snapshot in
            (st', MARS_RC_SUCCESS, Some sig)
        else (st, MARS_RC_REG, None)

    (* MARS_RC MARS_Sign (const void * ctx, uint16_t ctxlen, const void * dig, void * sig)
       key := CryptXkdf(DP, MARS_LU, ctx)
       sig := CryptSign(key, dig) *)
    | MARS_Sign ctx dig => 
        let key := CryptXkdf (DP st) MARS_LU (KC_Context ctx) in
        let sig := CryptSign key dig in
        (st, MARS_RC_SUCCESS, Some sig)

    (* MARS_RC MARS_SignatureVerify (bool restricted, const void * ctx, uint16_t ctxlen, const void * dig, const void * sig, bool * result)
       key := CryptXkdf(DP, restricted ? MARS_LR : MARS_LU, ctx)
       result := CryptVerify(key, dig, sig) *)   
    | MARS_SignatureVerify restricted ctx dig sig =>
        let label := if restricted then MARS_LR else MARS_LU in 
        let key := CryptXkdf (DP st) (label) (KC_Context ctx) in
        let result := CryptVerify key dig sig in
        (st, MARS_RC_SUCCESS, Some result)
    end.




