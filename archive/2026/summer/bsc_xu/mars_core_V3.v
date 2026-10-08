From Coq Require Import Bool.Bool.
From Coq Require Import Arith.Arith.
From Coq Require Import NArith.
From Coq Require Import Lists.List.
From Coq Require Import Lia.

Import ListNotations.

(* ========================================================================= *)
(* 0. Profile-independent enumerations                                       *)
(* ========================================================================= *)

(* Cryptographic Key Labels (§5.5, Table 2) *)
Inductive Label : Type :=
| MARS_LX   (* eXternal *)
| MARS_LD   (* Derivation Parent *)
| MARS_LU   (* Unrestricted signing *)
| MARS_LR.  (* Restricted attestation *)

Scheme Equality for Label.

(* Response Codes (§6.2, Table 4) *)
Inductive MARS_RC : Type :=
| MARS_RC_SUCCESS
| MARS_RC_IO        (* transport-level; unreachable in this model *)
| MARS_RC_FAILURE
| MARS_RC_BUFFER    (* memory-level; unreachable in this model *)
| MARS_RC_COMMAND
| MARS_RC_VALUE     (* unreachable: PT is a closed inductive *)
| MARS_RC_REG
| MARS_RC_SEQ.

(* Property Tags (§8.1.2, Table 6) *)
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

Section MARS_MODEL.

(* ========================================================================= *)
(* 1. Abstract data and profile constants                                    *)
(* ========================================================================= *)

Variable Digest    : Type.
Variable Secret    : Type.   (* PS, DP, derived keys *)
Variable Context   : Type.   (* application ctx, nonce, sequenced data *)
Variable Signature : Type.
Variable PublicKey : Type.   (* only appears in MARS_PublicRead's type *)

Variable DIGEST_ZERO : Digest.          (* PCR reset value (§5.4) *)
Variable PS_INIT     : Secret.          (* the device's Primary Seed (§5.3.2) *)
Variable TSR_INIT    : nat -> Digest.   (* profile-specified TSR init (§5.4) *)

(* Profile Constants (§6.1, Table 3) *)
Variable  PROFILE_COUNT_PCR : nat.     (* number of consecutive PCR implemented on this MARS *)
Variable  PROFILE_COUNT_TSR : nat.     (* number of consecutive TSR implemented on this MARS *)
Definition PROFILE_COUNT_REG : nat := PROFILE_COUNT_PCR + PROFILE_COUNT_TSR.

Variable  PROFILE_LEN_DIGEST : nat.    (* length of a digest that can be processed or produced *)
Variable  PROFILE_LEN_SIGN : nat.      (* length of signature produced by CryptSign() *)
Variable  PROFILE_LEN_KSYM : nat.      (* length of symmetric key produced by CryptSkdf() if implemented, otherwise 0 *)
Variable  PROFILE_LEN_KPUB : nat.      (* length of public asymmetric key returned by MARS_PublicRead() if implemented, otherwise 0 *)
Variable  PROFILE_LEN_KPRV : nat.      (* length of asymmetric key produced by CryptAkdf() if implemented, otherwise 0 *)
Variable  PROFILE_LEN_XKDF : nat.      (* PROFILE_LEN_KPRV if defined, else PROFILE_LEN_KSYM *)

Variable  PROFILE_ALG_HASH : nat.      (* TCG-registered algorithm for hashing by CryptHash functions *)
Variable  PROFILE_ALG_SIGN : nat.      (* TCG-registered algorithm for signing by CryptSign() *)
Variable  PROFILE_ALG_SKDF : nat.      (* TCG-registered algorithm for symmetric key derivation by CryptSkdf() *)
Variable  PROFILE_ALG_AKDF : nat.      (* TCG-registered algorithm for asymmetric key derivation by CryptAkdf() *)

(* §5.3.4: at least one PCR; at most 32 registers in total.
   (No theorem below needs these -- itself a finding: the bounds matter for
   the 32-bit regSelect encoding, not for the protocol logic.) *)
Hypothesis pcr_at_least_one : 1 <= PROFILE_COUNT_PCR.
Hypothesis reg_at_most_32   : PROFILE_COUNT_REG <= 32.

(* ========================================================================= *)
(* 2. Hash inputs and KDF contexts                                           *)
(* ========================================================================= *)


Inductive HashInput : Type :=
| HI_RegSelect : N -> HashInput
| HI_Digest    : Digest -> HashInput
| HI_Context      : Context -> HashInput.

(* KDF context: a MARS-computed snapshot digest, or raw application ctx.
   The constructors keep the two §5.5 uses of "context" apart by type. *)
Inductive KdfContext : Type :=
| KC_Digest  : Digest -> KdfContext
| KC_Context : Context -> KdfContext.

(* ========================================================================= *)
(* 3. Environment: all non-determinism at the device boundary                *)
(* ========================================================================= *)

(* Sensor readings vary over time; they are inputs
   to each transition, not global constants. *)
Record Env : Type := {
  TSR_SAMPLE   : nat -> Digest    (* TSR sampling values (§5.3.4.3) *)
}.

(* ========================================================================= *)
(* 4. Device state (§5.3) and setters                                        *)
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
  shc     : option (list HashInput) (* none means the sequence has not started*)
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

Definition set_shc (st : MARS_STATE) (new_shc : option (list HashInput)) : MARS_STATE :=
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
(* 5. Support Functions (§5.6)                                               *)
(* ========================================================================= *)

(* CryptSelfTest(fullTest) (§5.6.1) *)
Variable CryptSelfTest : bool -> bool.



(* CryptHash-related functions (§5.6.2) *)
(* CryptHashInit(shc) (§5.6.2.1)
   input: nothing
   output: initialized shc *)
Variable CryptHashInit : list HashInput.
(* CryptHashUpdate(shc, data, len) (§5.6.2.2)
   input: shc, data
   output: updated shc
   in this model, length of data or context will not be modeled, because it is meaningless *)
Variable CryptHashUpdate : list HashInput -> HashInput -> list HashInput.
(* CryptHashFinal(shc, out) (§5.6.2.3)
   input: shc
   output: resulting digest *)
Variable CryptHashFinal : list HashInput -> Digest.



(* CryptSign(key, digest) (§5.6.3)
   input: key, degest
   output: signature *)
Variable CryptSign : Secret -> Digest -> Signature.
(* CryptVerify(key, digest, signature) (§5.6.4)
   input: key, degest, signature
   output: result *)
Variable CryptVerify : Secret -> Digest -> Signature -> bool.



(* CryptSkdf(child, parent, label, ctx, ctxlen) (§5.6.5) 
   input: parent, label, ctx
   output: child *)
Variable CryptSkdf : Secret -> Label -> KdfContext -> Secret.
(* CryptAkdf(child, parent, label, ctx, ctxlen) (§5.6.6) 
   input: parent, label, ctx
   output: child *)
Variable CryptAkdf : Secret -> Label -> KdfContext -> Secret.
(* CryptXkdf is CryptAkdf if CryptAkdf is implemented. Otherwise, CryptXkdf is CryptSkdf. (§5.6.7) *)
(* Symmetric profile (§5.6.7): CryptXkdf IS CryptSkdf. *)
Definition CryptXkdf : Secret -> Label -> KdfContext -> Secret := CryptSkdf.



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

Definition sample (e : Env) (regSelect : N) (regs : list Digest) : list Digest :=
  map (fun i =>
         if (PROFILE_COUNT_PCR <=? i) && (i <? PROFILE_COUNT_REG) && N.testbit regSelect (N.of_nat i)
         then TSR_SAMPLE e i
         else nth i regs DIGEST_ZERO)
      (List.seq 0 PROFILE_COUNT_REG).


(* snapshot = CryptHash ( regSelect || REG# || ... || REG# || ctx ) (§5.6.9) *)
Definition CryptSnapshot (st : MARS_STATE) (e : Env) (regSelect : N) (ctx : Context) : MARS_STATE * Digest :=
  let new_regs := sample e regSelect (regs st) in
  let new_st := set_regs st new_regs in
  let shc1 := CryptHashInit in
  let shc2 := CryptHashUpdate shc1 (HI_RegSelect regSelect) in
  let shc3 := fold_left CryptHashUpdate  (map (fun i => HI_Digest i)(sel_values regSelect (regs new_st))) shc2 in
  let shc4 := CryptHashUpdate shc3 (HI_Context ctx) in
  (new_st, CryptHashFinal shc4).


(* ========================================================================= *)
(* 6. Named security hypotheses                                              *)
(*    Each theorem in Group B cites exactly the hypotheses it uses.          *)
(* ========================================================================= *)

(* Idealized collision resistance: the hash is injective on chunk lists. *)
Hypothesis H_hash_inj :
  forall l1 l2, CryptHashFinal l1 = CryptHashFinal l2 -> l1 = l2.

(* Idealized preservation of the chunk boundary by a hash update.  This is
   stronger than ordinary collision resistance and is used only by the
   symbolic context-binding theorem below. *)
Hypothesis H_hash_update_inj :
  forall l1 x1 l2 x2,
    CryptHashUpdate l1 x1 = CryptHashUpdate l2 x2 ->
    l1 = l2 /\ x1 = x2.

(* The PCR reset value is outside the hash image (extended PCR <> zero). *)
Hypothesis H_hash_nonzero :
  forall l, CryptHashFinal l <> DIGEST_ZERO.

(* Idealized KDF: injective in parent, label, and context jointly
   (label/context domain separation, §5.5). *)
Hypothesis H_skdf_inj :
  forall p1 l1 c1 p2 l2 c2,
    CryptSkdf p1 l1 c1 = CryptSkdf p2 l2 c2 ->
    p1 = p2 /\ l1 = l2 /\ c1 = c2.

(* No fixpoints: a derived key never equals its parent (freshness). *)
Hypothesis H_skdf_ne_parent :
  forall p l c, CryptSkdf p l c <> p.

(* MAC/signature correctness and (symbolic) soundness (§5.6.3, §5.6.4).
   For a symmetric MAC, verification recomputes and compares, giving both. *)
Hypothesis H_verify_correct :
  forall k d, CryptVerify k d (CryptSign k d) = true.
Hypothesis H_verify_sound :
  forall k d s, CryptVerify k d s = true -> s = CryptSign k d.

(* ========================================================================= *)
(* 7. Initialization (§5.4)                                                  *)
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
(* 8. Command interface (§8)                                                 *)
(* ========================================================================= *)


Inductive Command : Type :=
| MARS_SelfTest         (fullTest : bool)                                                       (* §8.1.1 *)
| MARS_CapabilityGet    (pt : PT)                                                               (* §8.1.2 *)
| MARS_SequenceHash                                                                             (* §8.2.1 *)
| MARS_SequenceUpdate   (data : HashInput)                                                      (* §8.2.2 *)
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



Definition is_capability (cmd : Command) : bool :=
  match cmd with MARS_CapabilityGet _ => true | _ => false end.

Definition is_pcr_extend (cmd : Command) : bool :=
  match cmd with MARS_PcrExtend _ _ => true | _ => false end.

Definition is_dp_derive (cmd : Command) : bool :=
  match cmd with MARS_DpDerive _ _ => true | _ => false end.

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

       
(* The transition function.  Note the explicit `as c return` annotations:
   the result type depends on the command, so both matches must be
   annotated for the dependent elimination to typecheck. *)
Definition transition (st : MARS_STATE) (e : Env) (cmd : Command) : MARS_STATE * MARS_RC * option (ReturnType cmd) :=
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
        if new_failure then (set_shc_to_none (set_failure st new_failure), MARS_RC_FAILURE, None) else (set_failure st new_failure, MARS_RC_SUCCESS, None)
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
            let shc2 := CryptHashUpdate shc1 (HI_Digest (nth pcrIndex (regs st) DIGEST_ZERO)) in
            let shc3 := CryptHashUpdate shc2 (HI_Digest dig) in
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
            let (st', snapshot) := CryptSnapshot st e regSelect ctx in
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
                let (st', snapshot) := CryptSnapshot st e regSelect ctx' in
                let DP := CryptSkdf (DP st') MARS_LD (KC_Digest snapshot) in
                (set_dp st' DP, MARS_RC_SUCCESS, None)
            | None =>
                let DP := (CryptDpInit (PS st)) in
                (set_dp st DP, MARS_RC_SUCCESS, None)
            end
        else (st, MARS_RC_REG, None)

    (* MARS_RC MARS_PublicRead (bool restricted, const void * ctx, uint16_t ctxlen, void * pub) *)
    | MARS_PublicRead restricted ctx => (st, MARS_RC_COMMAND, None)
        (* let label := if restricted then MARS_LR else MARS_LU in
        let key := CryptAkdf (DP st) label (KC_Context ctx) in
        let pub := ExtractPublicKey key in *)
    (* MARS_RC MARS_Quote (uint32_t regSelect, const void * nonce, uint16_t nlen, const void * ctx, uint16_t ctxlen, void * sig)
       (st', snapshot) := CryptSnapshot(st, regSelect, nonce)
       AK   := CryptXkdf(DP, MARS_LR, ctx)
       sig  := CryptSign(AK, snapshot) *)
    | MARS_Quote regSelect nonce ctx =>
        if valid_select regSelect
        then
            let (st', snapshot) := CryptSnapshot st e regSelect nonce in
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


(* ========================================================================= *)
(* 10. Generic list lemmas                                                   *)
(* ========================================================================= *)

Lemma update_length : forall l i x, length (update l i x) = length l.
Proof. induction l as [|h r IH]; intros [|i] x; simpl; auto. Qed.

Lemma nth_update_eq : forall l i x,
  i < length l -> nth i (update l i x) DIGEST_ZERO = x.
Proof.
  induction l as [|h r IH]; intros [|i] x Hi; simpl in *; try lia; auto.
  apply IH; lia.
Qed.

Lemma nth_update_neq : forall l i j x,
  i <> j -> nth j (update l i x) DIGEST_ZERO = nth j l DIGEST_ZERO.
Proof.
  induction l as [|h r IH]; intros [|i] [|j] x Hij; simpl; auto; try congruence.
Qed.

Lemma nth_map_seq : forall (f : nat -> Digest) n i,
  i < n -> nth i (map f (List.seq 0 n)) DIGEST_ZERO = f i.
Proof.
  intros f n i Hi.
  rewrite nth_indep with (d' := f 0).
  - rewrite map_nth. rewrite seq_nth by lia. simpl. reflexivity.
  - rewrite map_length, seq_length. exact Hi.
Qed.

Lemma len_map_seq : forall (f : nat -> Digest) n,
  length (map f (List.seq 0 n)) = n.
Proof. intros. rewrite map_length, seq_length. reflexivity. Qed.

Lemma sample_length : forall e rs rgs,
  length (sample e rs rgs) = PROFILE_COUNT_REG.
Proof. intros. apply len_map_seq. Qed.

Lemma sample_preserves_pcr : forall e rs rgs i,
  i < PROFILE_COUNT_PCR ->
  nth i (sample e rs rgs) DIGEST_ZERO = nth i rgs DIGEST_ZERO.
Proof.
  intros e rs rgs i Hi. unfold sample.
  rewrite nth_map_seq by (unfold PROFILE_COUNT_REG; lia).
  destruct (PROFILE_COUNT_PCR <=? i) eqn:Hle.
  - exfalso. apply Nat.leb_le in Hle. lia.
  - simpl. reflexivity.
Qed.

(* ========================================================================= *)
(* 10. Lemmas                                                   *)
(* ========================================================================= *)
Lemma cap_st_immutable : forall st pt st' rc o,
  cap_transition st pt = (st', rc, o) ->
  st' = st.
Proof.
  intros st pt st' rc o H1.
  destruct pt.
  - simpl in H1. inversion H1. subst. reflexivity.
  - simpl in H1. inversion H1. subst. reflexivity.
  - simpl in H1. inversion H1. subst. reflexivity.
  - simpl in H1. inversion H1. subst. reflexivity.
  - simpl in H1. inversion H1. subst. reflexivity.
  - simpl in H1. inversion H1. subst. reflexivity.
  - simpl in H1. inversion H1. subst. reflexivity.
  - simpl in H1. inversion H1. subst. reflexivity.
  - simpl in H1. inversion H1. subst. reflexivity.
  - simpl in H1. inversion H1. subst. reflexivity.
  - simpl in H1. inversion H1. subst. reflexivity.
Qed.

Lemma cap_rc_success : forall st pt st' rc o,
  cap_transition st pt = (st', rc, o) ->
  rc = MARS_RC_SUCCESS.
Proof.
  intros st pt st' rc o H1.
  destruct pt.
  - simpl in H1. inversion H1. subst. reflexivity.
  - simpl in H1. inversion H1. subst. reflexivity.
  - simpl in H1. inversion H1. subst. reflexivity.
  - simpl in H1. inversion H1. subst. reflexivity.
  - simpl in H1. inversion H1. subst. reflexivity.
  - simpl in H1. inversion H1. subst. reflexivity.
  - simpl in H1. inversion H1. subst. reflexivity.
  - simpl in H1. inversion H1. subst. reflexivity.
  - simpl in H1. inversion H1. subst. reflexivity.
  - simpl in H1. inversion H1. subst. reflexivity.
  - simpl in H1. inversion H1. subst. reflexivity.
Qed.

(* Field-level facts about CryptSnapshot.  These lemmas isolate the only
   transition helper that can rewrite the register file. *)
Lemma snapshot_preserves_failure : forall st e rs ctx st' dig,
  CryptSnapshot st e rs ctx = (st', dig) ->
  failure st' = failure st.
Proof.
  intros st e rs ctx st' dig H.
  unfold CryptSnapshot in H; cbn in H.
  inversion H; reflexivity.
Qed.

Lemma snapshot_preserves_ps : forall st e rs ctx st' dig,
  CryptSnapshot st e rs ctx = (st', dig) ->
  PS st' = PS st.
Proof.
  intros st e rs ctx st' dig H.
  unfold CryptSnapshot in H; cbn in H.
  inversion H; reflexivity.
Qed.

Lemma snapshot_preserves_dp : forall st e rs ctx st' dig,
  CryptSnapshot st e rs ctx = (st', dig) ->
  DP st' = DP st.
Proof.
  intros st e rs ctx st' dig H.
  unfold CryptSnapshot in H; cbn in H.
  inversion H; reflexivity.
Qed.

Lemma snapshot_preserves_shc : forall st e rs ctx st' dig,
  CryptSnapshot st e rs ctx = (st', dig) ->
  shc st' = shc st.
Proof.
  intros st e rs ctx st' dig H.
  unfold CryptSnapshot in H; cbn in H.
  inversion H; reflexivity.
Qed.

Lemma snapshot_reg_length : forall st e rs ctx st' dig,
  CryptSnapshot st e rs ctx = (st', dig) ->
  length (regs st') = PROFILE_COUNT_REG.
Proof.
  intros st e rs ctx st' dig H.
  unfold CryptSnapshot in H; cbn in H.
  inversion H; subst; apply sample_length.
Qed.

Lemma snapshot_preserves_pcr_value : forall st e rs ctx st' dig i,
  CryptSnapshot st e rs ctx = (st', dig) ->
  i < PROFILE_COUNT_PCR ->
  nth i (regs st') DIGEST_ZERO = nth i (regs st) DIGEST_ZERO.
Proof.
  intros st e rs ctx st' dig i Hsnap Hi.
  unfold CryptSnapshot in Hsnap; cbn in Hsnap.
  inversion Hsnap; subst; apply sample_preserves_pcr; exact Hi.
Qed.

(* ========================================================================= *)
(* 12. Group A -- structural theorems (NO cryptographic hypotheses)          *)
(* ========================================================================= *)

(* --- A1. Failure-mode lockout and absorption (§5.3.1) ------------------- *)
Theorem failure_lockout : forall st e cmd,
  failure st = true ->
  is_capability cmd = false ->
  transition st e cmd = (st, MARS_RC_FAILURE, None).
Proof.
  intros st e cmd H1 H2. 
  unfold transition. 
  rewrite -> H1.
  destruct cmd.
    - reflexivity.
    - discriminate.
    - reflexivity.
    - reflexivity.
    - reflexivity.
    - reflexivity.
    - reflexivity.
    - reflexivity.
    - reflexivity.
    - reflexivity.
    - reflexivity.
    - reflexivity.
    - reflexivity.
Qed.

Theorem failure_absorbing : forall st e cmd st' rc o,
  failure st = true ->
  transition st e cmd = (st', rc, o) ->
  failure st' = true.
Proof.
  intros st e cmd st' rc o H1 H2.
  unfold transition in H2. 
  rewrite H1 in H2.
  destruct cmd. 
  all:try(inversion H2; subst; apply H1).
  - apply cap_st_immutable in H2. rewrite -> H2. apply H1.
  (* - inversion H2. subst. apply H1.
  - apply cap_st_immutable in H2. rewrite -> H2. apply H1.
  - inversion H2. subst. apply H1.
  - inversion H2. subst. apply H1.
  - inversion H2. subst. apply H1.
  - inversion H2. subst. apply H1.
  - inversion H2. subst. apply H1.
  - inversion H2. subst. apply H1.
  - inversion H2. subst. apply H1.
  - inversion H2. subst. apply H1.
  - inversion H2. subst. apply H1.
  - inversion H2. subst. apply H1.
  - inversion H2. subst. apply H1. *)
Qed.

Theorem capability_available_in_failure : forall st e pt,
  failure st = true ->
  transition st e (MARS_CapabilityGet pt) = cap_transition st pt.
Proof.
  intros st e pt Hfail. unfold transition. rewrite Hfail. reflexivity.
Qed.

Theorem selftest_failure_enters_failure : forall st e fullTest,
  failure st = false ->
  CryptSelfTest fullTest = false ->
  transition st e (MARS_SelfTest fullTest) =
    let base := set_shc_to_none st in
    (set_shc_to_none (set_failure base true), MARS_RC_FAILURE, None).
Proof.
  intros st e fullTest Hfail Htest.
  unfold transition. rewrite Hfail. cbn. rewrite Htest. cbn.
  rewrite Hfail. reflexivity.
Qed.

Theorem selftest_success_preserves_normal_mode : forall st e fullTest,
  failure st = false ->
  CryptSelfTest fullTest = true ->
  transition st e (MARS_SelfTest fullTest) =
    let base := set_shc_to_none st in
    (set_failure base false, MARS_RC_SUCCESS, None).
Proof.
  intros st e fullTest Hfail Htest.
  unfold transition. rewrite Hfail. cbn. rewrite Htest. cbn.
  rewrite Hfail. reflexivity.
Qed.

(* --- A2. The Primary Seed is immutable (§5.3.2) ------------------------- *)
Theorem ps_immutable : forall st e cmd st' rc o,
  transition st e cmd = (st', rc, o) ->
  PS st' = PS st.
Proof.
  intros [f ps dpv r sequence] e cmd st' rc o H.
  unfold transition, CryptSnapshot, cap_transition in H.
  destruct f; destruct cmd; cbn in H;
    repeat
      match goal with
      | Hm : context [if ?b then _ else _] |- _ => destruct b eqn:?
      | Hm : context [match ?x with _ => _ end] |- _ => destruct x eqn:?
      end;
    inversion H; reflexivity.
Qed.

(* --- A2b. Initialization establishes the protocol's base state (§5.4) --- *)
Theorem init_ps : PS mars_init = PS_INIT.
Proof. reflexivity. Qed.

Theorem init_dp : DP mars_init = CryptDpInit PS_INIT.
Proof. reflexivity. Qed.

Theorem init_sequence_idle : shc mars_init = None.
Proof. reflexivity. Qed.

Theorem init_failure_reflects_selftest :
  failure mars_init = negb (CryptSelfTest true).
Proof. reflexivity. Qed.

Theorem init_ready_if_selftest_passes :
  CryptSelfTest true = true ->
  failure mars_init = false.
Proof.
  intro Htest. unfold mars_init; cbn. rewrite Htest. reflexivity.
Qed.

Theorem init_pcr_zero : forall i,
  i < PROFILE_COUNT_PCR ->
  nth i (regs mars_init) DIGEST_ZERO = DIGEST_ZERO.
Proof.
  intros i Hi.
  change
    (nth i
       (map
          (fun j =>
             if j <? PROFILE_COUNT_PCR
             then DIGEST_ZERO
             else TSR_INIT j)
          (List.seq 0 PROFILE_COUNT_REG))
       DIGEST_ZERO = DIGEST_ZERO).
  rewrite nth_map_seq by (unfold PROFILE_COUNT_REG; lia).
  destruct (i <? PROFILE_COUNT_PCR) eqn:Hlt.
  - reflexivity.
  - apply Nat.ltb_ge in Hlt. lia.
Qed.

Theorem init_tsr_profile_value : forall i,
  PROFILE_COUNT_PCR <= i ->
  i < PROFILE_COUNT_REG ->
  nth i (regs mars_init) DIGEST_ZERO = TSR_INIT i.
Proof.
  intros i Hipcr Hireg.
  change
    (nth i
       (map
          (fun j =>
             if j <? PROFILE_COUNT_PCR
             then DIGEST_ZERO
             else TSR_INIT j)
          (List.seq 0 PROFILE_COUNT_REG))
       DIGEST_ZERO = TSR_INIT i).
  rewrite nth_map_seq by exact Hireg.
  destruct (i <? PROFILE_COUNT_PCR) eqn:Hlt.
  - apply Nat.ltb_lt in Hlt. lia.
  - reflexivity.
Qed.

(* --- A3. Register-file length invariant (§5.3.4) ------------------------ *)
Theorem init_reg_length : length (regs mars_init) = PROFILE_COUNT_REG.
Proof.
  unfold mars_init; cbn.
  apply len_map_seq.
Qed.

Theorem transition_preserves_reg_length : forall st e cmd st' rc o,
  transition st e cmd = (st', rc, o) ->
  length (regs st) = PROFILE_COUNT_REG ->
  length (regs st') = PROFILE_COUNT_REG.
Proof.
  intros st e cmd st' rc o Htrans Hlen.
  unfold transition in Htrans.
  destruct (failure st) eqn:Hfail.
  - destruct cmd; cbn in Htrans;
      try (inversion Htrans; subst; exact Hlen).
    apply cap_st_immutable in Htrans; subst; exact Hlen.
  - destruct cmd; cbn in Htrans;
      repeat
        match goal with
        | Hm : context [if ?b then _ else _] |- _ => destruct b eqn:?
        | Hm : context [match ?x with _ => _ end] |- _ => destruct x eqn:?
        | Hm : context [let '(_, _) := ?x in _] |- _ =>
            destruct x eqn:?
        end;
      try (inversion Htrans; subst; cbn; rewrite ?update_length; exact Hlen);
      try (apply cap_st_immutable in Htrans; subst; cbn; exact Hlen);
      try (inversion Htrans; subst; cbn; apply sample_length).
Qed.

(* --- A3b. Only MARS_DpDerive may overwrite the volatile DP (§5.3.3) ---- *)
Theorem dp_only_changed_by_dpderive : forall st e cmd st' rc o,
  transition st e cmd = (st', rc, o) ->
  is_dp_derive cmd = false ->
  DP st' = DP st.
Proof.
  intros st e cmd st' rc o Htrans Hnotdp.
  unfold transition in Htrans.
  destruct (failure st) eqn:Hfail.
  - destruct cmd; cbn in Htrans;
      try (inversion Htrans; subst; reflexivity).
    apply cap_st_immutable in Htrans; subst; reflexivity.
  - destruct cmd; cbn in Hnotdp; try discriminate; cbn in Htrans;
      repeat
        match goal with
        | Hm : context [if ?b then _ else _] |- _ => destruct b eqn:?
        | Hm : context [match ?x with _ => _ end] |- _ => destruct x eqn:?
        | Hm : context [let '(_, _) := ?x in _] |- _ =>
            destruct x eqn:?
        end;
      try (inversion Htrans; subst; reflexivity).
    apply cap_st_immutable in Htrans; subst; reflexivity.
Qed.

Theorem dp_reset_correct : forall st e regSelect,
  failure st = false ->
  valid_select regSelect = true ->
  transition st e (MARS_DpDerive regSelect None) =
    (set_dp (set_shc_to_none st) (CryptDpInit (PS st)),
     MARS_RC_SUCCESS, None).
Proof.
  intros st e regSelect Hfail Hvalid.
  unfold transition. rewrite Hfail. cbn. rewrite Hvalid. reflexivity.
Qed.

Theorem dp_derive_shape : forall st e regSelect ctx,
  failure st = false ->
  valid_select regSelect = true ->
  transition st e (MARS_DpDerive regSelect (Some ctx)) =
    let base := set_shc_to_none st in
    let '(sampled, snapshot) := CryptSnapshot base e regSelect ctx in
    (set_dp sampled
       (CryptSkdf (DP sampled) MARS_LD (KC_Digest snapshot)),
     MARS_RC_SUCCESS, None).
Proof.
  intros st e regSelect ctx Hfail Hvalid.
  unfold transition. rewrite Hfail. cbn. rewrite Hvalid. reflexivity.
Qed.

(* --- A4. PCR integrity: only PcrExtend writes a PCR (§5.3.4.2) ---------- *)
Theorem pcr_only_changed_by_extend : forall st e cmd st' rc o i,
  transition st e cmd = (st', rc, o) ->
  is_pcr_extend cmd = false ->
  i < PROFILE_COUNT_PCR ->
  nth i (regs st') DIGEST_ZERO = nth i (regs st) DIGEST_ZERO.
Proof.
  intros st e cmd st' rc o i Htrans Hnotext Hi.
  unfold transition in Htrans.
  destruct (failure st) eqn:Hfail.
  - destruct cmd; cbn in Htrans;
      try (inversion Htrans; subst; reflexivity).
    apply cap_st_immutable in Htrans; subst; reflexivity.
  - destruct cmd; cbn in Hnotext; try discriminate; cbn in Htrans;
      repeat
        match goal with
        | Hm : context [if ?b then _ else _] |- _ => destruct b eqn:?
        | Hm : context [match ?x with _ => _ end] |- _ => destruct x eqn:?
        | Hm : context [let '(_, _) := ?x in _] |- _ =>
            destruct x eqn:?
        end;
      try (inversion Htrans; subst; reflexivity).
    all: try (apply cap_st_immutable in Htrans; subst; reflexivity).
    all: inversion Htrans; subst; cbn;
         apply sample_preserves_pcr; exact Hi.
Qed.

Definition pcr_extend_value
    (st : MARS_STATE) (pcrIndex : nat) (dig : Digest) : Digest :=
  CryptHashFinal
    (CryptHashUpdate
       (CryptHashUpdate CryptHashInit
          (HI_Digest (nth pcrIndex (regs st) DIGEST_ZERO)))
       (HI_Digest dig)).

Theorem pcr_extend_correct : forall st e pcrIndex dig,
  failure st = false ->
  pcrIndex < PROFILE_COUNT_PCR ->
  transition st e (MARS_PcrExtend pcrIndex dig) =
    (set_reg (set_shc_to_none st) pcrIndex
       (pcr_extend_value st pcrIndex dig),
     MARS_RC_SUCCESS, None).
Proof.
  intros st e pcrIndex dig Hfail Hindex.
  assert (pcrIndex <? PROFILE_COUNT_PCR = true) as Hlt by
      (apply Nat.ltb_lt; exact Hindex).
  unfold transition. rewrite Hfail, Hlt. cbn.
  unfold pcr_extend_value. reflexivity.
Qed.

Theorem pcr_extend_invalid_index : forall st e pcrIndex dig,
  failure st = false ->
  PROFILE_COUNT_PCR <= pcrIndex ->
  transition st e (MARS_PcrExtend pcrIndex dig) =
    (set_shc_to_none st, MARS_RC_REG, None).
Proof.
  intros st e pcrIndex dig Hfail Hindex.
  assert (pcrIndex <? PROFILE_COUNT_PCR = false) as Hge by
      (apply Nat.ltb_ge; exact Hindex).
  unfold transition. rewrite Hfail, Hge. cbn. reflexivity.
Qed.

Theorem pcr_extend_preserves_other_registers :
  forall st e pcrIndex dig st' rc out j,
    failure st = false ->
    pcrIndex < PROFILE_COUNT_PCR ->
    transition st e (MARS_PcrExtend pcrIndex dig) = (st', rc, out) ->
    pcrIndex <> j ->
    nth j (regs st') DIGEST_ZERO = nth j (regs st) DIGEST_ZERO.
Proof.
  intros st e pcrIndex dig st' rc out j Hfail Hindex Htrans Hneq.
  rewrite (pcr_extend_correct st e pcrIndex dig Hfail Hindex) in Htrans.
  inversion Htrans; subst. cbn.
  apply nth_update_neq. exact Hneq.
Qed.

Theorem pcr_extend_updates_selected :
  forall st e pcrIndex dig st' rc out,
    failure st = false ->
    pcrIndex < PROFILE_COUNT_PCR ->
    length (regs st) = PROFILE_COUNT_REG ->
    transition st e (MARS_PcrExtend pcrIndex dig) = (st', rc, out) ->
    nth pcrIndex (regs st') DIGEST_ZERO =
      pcr_extend_value st pcrIndex dig.
Proof.
  intros st e pcrIndex dig st' rc out
         Hfail Hindex Hlength Htrans.
  rewrite (pcr_extend_correct st e pcrIndex dig Hfail Hindex) in Htrans.
  inversion Htrans; subst. cbn.
  apply nth_update_eq. rewrite Hlength.
  unfold PROFILE_COUNT_REG. lia.
Qed.

Theorem pcr_extend_result_nonzero : forall st pcrIndex dig,
  pcr_extend_value st pcrIndex dig <> DIGEST_ZERO.
Proof.
  intros st pcrIndex dig. unfold pcr_extend_value.
  apply H_hash_nonzero.
Qed.

(* TSR sampling facts (§5.3.4.3, §5.6.9).  In this model TSR_SAMPLE is
   indexed by the global register index, as is the REG array. *)
Lemma sample_selected_tsr : forall e regSelect rgs i,
  PROFILE_COUNT_PCR <= i ->
  i < PROFILE_COUNT_REG ->
  N.testbit regSelect (N.of_nat i) = true ->
  nth i (sample e regSelect rgs) DIGEST_ZERO = TSR_SAMPLE e i.
Proof.
  intros e regSelect rgs i Hipcr Hireg Hselected.
  unfold sample. rewrite nth_map_seq by exact Hireg.
  assert (PROFILE_COUNT_PCR <=? i = true) as Hge by
      (apply Nat.leb_le; exact Hipcr).
  assert (i <? PROFILE_COUNT_REG = true) as Hlt by
      (apply Nat.ltb_lt; exact Hireg).
  rewrite Hge, Hlt, Hselected. reflexivity.
Qed.

Lemma sample_preserves_unselected : forall e regSelect rgs i,
  i < PROFILE_COUNT_REG ->
  N.testbit regSelect (N.of_nat i) = false ->
  nth i (sample e regSelect rgs) DIGEST_ZERO =
  nth i rgs DIGEST_ZERO.
Proof.
  intros e regSelect rgs i Hireg Hunselected.
  unfold sample. rewrite nth_map_seq by exact Hireg.
  rewrite Hunselected. repeat rewrite andb_false_r. reflexivity.
Qed.

Theorem snapshot_selected_tsr : forall st e regSelect ctx st' snapshot i,
  CryptSnapshot st e regSelect ctx = (st', snapshot) ->
  PROFILE_COUNT_PCR <= i ->
  i < PROFILE_COUNT_REG ->
  N.testbit regSelect (N.of_nat i) = true ->
  nth i (regs st') DIGEST_ZERO = TSR_SAMPLE e i.
Proof.
  intros st e regSelect ctx st' snapshot i Hsnap Hipcr Hireg Hselected.
  unfold CryptSnapshot in Hsnap; cbn in Hsnap.
  inversion Hsnap; subst.
  apply sample_selected_tsr; assumption.
Qed.

Theorem snapshot_preserves_unselected : forall st e regSelect ctx st' snapshot i,
  CryptSnapshot st e regSelect ctx = (st', snapshot) ->
  i < PROFILE_COUNT_REG ->
  N.testbit regSelect (N.of_nat i) = false ->
  nth i (regs st') DIGEST_ZERO = nth i (regs st) DIGEST_ZERO.
Proof.
  intros st e regSelect ctx st' snapshot i Hsnap Hireg Hunselected.
  unfold CryptSnapshot in Hsnap; cbn in Hsnap.
  inversion Hsnap; subst.
  apply sample_preserves_unselected; assumption.
Qed.

Theorem reg_read_correct : forall st e regIndex,
  failure st = false ->
  regIndex < PROFILE_COUNT_REG ->
  transition st e (MARS_RegRead regIndex) =
    (set_shc_to_none st, MARS_RC_SUCCESS,
     Some (nth regIndex (regs st) DIGEST_ZERO)).
Proof.
  intros st e regIndex Hfail Hindex.
  assert (regIndex <? PROFILE_COUNT_REG = true) as Hlt by
      (apply Nat.ltb_lt; exact Hindex).
  unfold transition. rewrite Hfail, Hlt. cbn. reflexivity.
Qed.

Theorem reg_read_invalid_index : forall st e regIndex,
  failure st = false ->
  PROFILE_COUNT_REG <= regIndex ->
  transition st e (MARS_RegRead regIndex) =
    (set_shc_to_none st, MARS_RC_REG, None).
Proof.
  intros st e regIndex Hfail Hindex.
  assert (regIndex <? PROFILE_COUNT_REG = false) as Hge by
      (apply Nat.ltb_ge; exact Hindex).
  unfold transition. rewrite Hfail, Hge. cbn. reflexivity.
Qed.



(* --- A5. Sequence discipline (§8.2, §5.7) ------------------------------- *)
Theorem sequence_start_correct : forall st e,
  failure st = false ->
  transition st e MARS_SequenceHash =
    (set_shc st (Some CryptHashInit), MARS_RC_SUCCESS, None).
Proof.
  intros st e Hfail. unfold transition. rewrite Hfail. reflexivity.
Qed.

Theorem sequence_update_correct : forall st e acc data,
  failure st = false ->
  shc st = Some acc ->
  transition st e (MARS_SequenceUpdate data) =
    (set_shc st (Some (CryptHashUpdate acc data)),
     MARS_RC_SUCCESS, None).
Proof.
  intros st e acc data Hfail Hshc.
  unfold transition. rewrite Hfail. cbn. rewrite Hshc. reflexivity.
Qed.

Theorem sequence_complete_correct : forall st e acc,
  failure st = false ->
  shc st = Some acc ->
  transition st e MARS_SequenceComplete =
    (set_shc_to_none st, MARS_RC_SUCCESS,
     Some (CryptHashFinal acc)).
Proof.
  intros st e acc Hfail Hshc.
  unfold transition. rewrite Hfail. cbn. rewrite Hshc. reflexivity.
Qed.

Theorem sequence_not_started : forall st e,
  failure st = false ->
  shc st = None ->
  (forall d, transition st e (MARS_SequenceUpdate d) = (st, MARS_RC_SEQ, None)) /\
  transition st e MARS_SequenceComplete = (st, MARS_RC_SEQ, None).
Proof.
  intros st e Hfail Hshc.
  split.
  - intro d. unfold transition. rewrite Hfail. cbn. rewrite Hshc. reflexivity.
  - unfold transition. rewrite Hfail. cbn. rewrite Hshc. reflexivity.
Qed.

Theorem interleaving_terminates_sequence : forall st e cmd st' rc o,
  failure st = false ->
  is_seq_cmd cmd = false ->
  is_capability cmd = false ->
  transition st e cmd = (st', rc, o) ->
  shc st' = None.
Proof.
  intros st e cmd st' rc o Hfail Hnonseq Hnoncap Htrans.
  unfold transition in Htrans. rewrite Hfail in Htrans.
  destruct cmd; cbn in Hnonseq, Hnoncap; try discriminate; cbn in Htrans;
    repeat
      match goal with
      | Hm : context [if ?b then _ else _] |- _ => destruct b eqn:?
      | Hm : context [match ?x with _ => _ end] |- _ => destruct x eqn:?
      | Hm : context [let '(_, _) := ?x in _] |- _ =>
          destruct x eqn:?
      end;
    try (inversion Htrans; subst; reflexivity).
  all: try (apply cap_st_immutable in Htrans; subst; reflexivity).
  all: eapply snapshot_preserves_shc in Htrans;
       cbn in Htrans; exact Htrans.
Qed.

Theorem interleaving_terminates_sequence_all : forall st e cmd st' rc o,
  failure st = false ->
  is_seq_cmd cmd = false ->
  transition st e cmd = (st', rc, o) ->
  shc st' = None.
Proof.
  intros st e cmd st' rc o Hfail Hnonseq Htrans.
  unfold transition in Htrans. rewrite Hfail in Htrans.
  destruct cmd; cbn in Hnonseq; try discriminate; cbn in Htrans;
    repeat
      match goal with
      | Hm : context [if ?b then _ else _] |- _ => destruct b eqn:?
      | Hm : context [match ?x with _ => _ end] |- _ => destruct x eqn:?
      | Hm : context [let '(_, _) := ?x in _] |- _ =>
          destruct x eqn:?
      end;
    try (inversion Htrans; subst; reflexivity).
  all: try (apply cap_st_immutable in Htrans; subst; reflexivity).
  all: eapply snapshot_preserves_shc in Htrans;
       cbn in Htrans; exact Htrans.
Qed.

Theorem sequence_complete_consumes_context : forall st e acc,
  failure st = false ->
  shc st = Some acc ->
  let completed := set_shc_to_none st in
  transition completed e MARS_SequenceComplete =
    (completed, MARS_RC_SEQ, None).
Proof.
  intros st e acc Hfail Hshc. cbn.
  unfold transition. cbn. rewrite Hfail. reflexivity.
Qed.

Theorem sequence_one_chunk_trace : forall st e data,
  failure st = false ->
  let started := set_shc st (Some CryptHashInit) in
  let acc := CryptHashUpdate CryptHashInit data in
  let updated := set_shc started (Some acc) in
  transition st e MARS_SequenceHash =
      (started, MARS_RC_SUCCESS, None) /\
  transition started e (MARS_SequenceUpdate data) =
      (updated, MARS_RC_SUCCESS, None) /\
  transition updated e MARS_SequenceComplete =
      (set_shc_to_none updated, MARS_RC_SUCCESS,
       Some (CryptHashFinal acc)).
Proof.
  intros st e data Hfail. cbn.
  split.
  - apply sequence_start_correct; exact Hfail.
  - split.
    + apply sequence_update_correct with (acc := CryptHashInit).
      * cbn. exact Hfail.
      * reflexivity.
    + apply sequence_complete_correct
        with (acc := CryptHashUpdate CryptHashInit data).
      * cbn. exact Hfail.
      * reflexivity.
Qed.

(* ========================================================================= *)
(* 13. Command-shape and response-code properties                            *)
(* ========================================================================= *)

Theorem derive_invalid_select : forall st e regSelect ctx,
  failure st = false ->
  valid_select regSelect = false ->
  transition st e (MARS_Derive regSelect ctx) =
    (set_shc_to_none st, MARS_RC_REG, None).
Proof.
  intros st e regSelect ctx Hfail Hinvalid.
  unfold transition. rewrite Hfail. cbn. rewrite Hinvalid. reflexivity.
Qed.

Theorem dpderive_invalid_select : forall st e regSelect ctx,
  failure st = false ->
  valid_select regSelect = false ->
  transition st e (MARS_DpDerive regSelect ctx) =
    (set_shc_to_none st, MARS_RC_REG, None).
Proof.
  intros st e regSelect ctx Hfail Hinvalid.
  unfold transition. rewrite Hfail. cbn. rewrite Hinvalid. reflexivity.
Qed.

Theorem quote_invalid_select : forall st e regSelect nonce ctx,
  failure st = false ->
  valid_select regSelect = false ->
  transition st e (MARS_Quote regSelect nonce ctx) =
    (set_shc_to_none st, MARS_RC_REG, None).
Proof.
  intros st e regSelect nonce ctx Hfail Hinvalid.
  unfold transition. rewrite Hfail. cbn. rewrite Hinvalid. reflexivity.
Qed.

Theorem derive_shape : forall st e regSelect ctx,
  failure st = false ->
  valid_select regSelect = true ->
  transition st e (MARS_Derive regSelect ctx) =
    let base := set_shc_to_none st in
    let '(sampled, snapshot) := CryptSnapshot base e regSelect ctx in
    (sampled, MARS_RC_SUCCESS,
     Some (CryptSkdf (DP sampled) MARS_LX (KC_Digest snapshot))).
Proof.
  intros st e regSelect ctx Hfail Hvalid.
  unfold transition. rewrite Hfail. cbn. rewrite Hvalid. reflexivity.
Qed.

Theorem quote_shape : forall st e regSelect nonce ctx,
  failure st = false ->
  valid_select regSelect = true ->
  transition st e (MARS_Quote regSelect nonce ctx) =
    let base := set_shc_to_none st in
    let '(sampled, snapshot) := CryptSnapshot base e regSelect nonce in
    let ak := CryptXkdf (DP sampled) MARS_LR (KC_Context ctx) in
    (sampled, MARS_RC_SUCCESS, Some (CryptSign ak snapshot)).
Proof.
  intros st e regSelect nonce ctx Hfail Hvalid.
  unfold transition. rewrite Hfail. cbn. rewrite Hvalid. reflexivity.
Qed.

Theorem sign_shape : forall st e ctx dig,
  failure st = false ->
  transition st e (MARS_Sign ctx dig) =
    let base := set_shc_to_none st in
    let key := CryptXkdf (DP base) MARS_LU (KC_Context ctx) in
    (base, MARS_RC_SUCCESS, Some (CryptSign key dig)).
Proof.
  intros st e ctx dig Hfail.
  unfold transition. rewrite Hfail. reflexivity.
Qed.

Theorem signature_verify_shape : forall st e restricted ctx dig sig,
  failure st = false ->
  transition st e (MARS_SignatureVerify restricted ctx dig sig) =
    let base := set_shc_to_none st in
    let label := if restricted then MARS_LR else MARS_LU in
    let key := CryptXkdf (DP base) label (KC_Context ctx) in
    (base, MARS_RC_SUCCESS, Some (CryptVerify key dig sig)).
Proof.
  intros st e restricted ctx dig sig Hfail.
  unfold transition. rewrite Hfail. reflexivity.
Qed.

(* This is an explicit limitation of the present symmetric-profile model:
   asymmetric public-key extraction is intentionally not implemented. *)
Theorem public_read_unsupported : forall st e restricted ctx,
  failure st = false ->
  transition st e (MARS_PublicRead restricted ctx) =
    (set_shc_to_none st, MARS_RC_COMMAND, None).
Proof.
  intros st e restricted ctx Hfail.
  unfold transition. rewrite Hfail. reflexivity.
Qed.

(* ========================================================================= *)
(* 14. Group B -- cryptographic properties (named ideal assumptions)         *)
(* ========================================================================= *)

Theorem skdf_label_separation : forall parent label1 label2 ctx1 ctx2,
  label1 <> label2 ->
  CryptSkdf parent label1 ctx1 <> CryptSkdf parent label2 ctx2.
Proof.
  intros parent label1 label2 ctx1 ctx2 Hlabels Heq.
  destruct (H_skdf_inj parent label1 ctx1 parent label2 ctx2 Heq)
    as [_ [Hlabel _]].
  contradiction.
Qed.

Theorem skdf_context_separation : forall parent label ctx1 ctx2,
  ctx1 <> ctx2 ->
  CryptSkdf parent label ctx1 <> CryptSkdf parent label ctx2.
Proof.
  intros parent label ctx1 ctx2 Hcontexts Heq.
  destruct (H_skdf_inj parent label ctx1 parent label ctx2 Heq)
    as [_ [_ Hctx]].
  contradiction.
Qed.

Theorem restricted_unrestricted_key_separation : forall parent c1 c2,
  CryptXkdf parent MARS_LU (KC_Context c1) <>
  CryptXkdf parent MARS_LR (KC_Context c2).
Proof.
  intros parent c1 c2. unfold CryptXkdf.
  apply skdf_label_separation. discriminate.
Qed.

Theorem quote_ak_context_separation : forall parent c1 c2,
  c1 <> c2 ->
  CryptXkdf parent MARS_LR (KC_Context c1) <>
  CryptXkdf parent MARS_LR (KC_Context c2).
Proof.
  intros parent c1 c2 Hctx. unfold CryptXkdf.
  apply skdf_context_separation.
  intro Hkc. inversion Hkc. contradiction.
Qed.

Theorem dp_derive_fresh_successor : forall st e regSelect ctx,
  failure st = false ->
  valid_select regSelect = true ->
  exists st',
    transition st e (MARS_DpDerive regSelect (Some ctx)) =
      (st', MARS_RC_SUCCESS, None) /\
    DP st' <> DP st.
Proof.
  intros st e regSelect ctx Hfail Hvalid.
  eexists.
  split.
  - apply dp_derive_shape; assumption.
  - cbn. apply H_skdf_ne_parent.
Qed.

Theorem hash_last_context_binding : forall acc1 acc2 ctx1 ctx2,
  CryptHashFinal (CryptHashUpdate acc1 (HI_Context ctx1)) =
  CryptHashFinal (CryptHashUpdate acc2 (HI_Context ctx2)) ->
  ctx1 = ctx2.
Proof.
  intros acc1 acc2 ctx1 ctx2 Hfinal.
  apply H_hash_inj in Hfinal.
  destruct (H_hash_update_inj
              acc1 (HI_Context ctx1) acc2 (HI_Context ctx2) Hfinal)
    as [_ Hcontext].
  inversion Hcontext. reflexivity.
Qed.

Theorem snapshot_context_binding :
  forall st1 e1 regSelect1 ctx1 sampled1 digest1
         st2 e2 regSelect2 ctx2 sampled2 digest2,
    CryptSnapshot st1 e1 regSelect1 ctx1 = (sampled1, digest1) ->
    CryptSnapshot st2 e2 regSelect2 ctx2 = (sampled2, digest2) ->
    digest1 = digest2 ->
    ctx1 = ctx2.
Proof.
  intros st1 e1 regSelect1 ctx1 sampled1 digest1
         st2 e2 regSelect2 ctx2 sampled2 digest2 Hsnap1 Hsnap2 Hdigest.
  unfold CryptSnapshot in Hsnap1, Hsnap2; cbn in Hsnap1, Hsnap2.
  inversion Hsnap1; inversion Hsnap2; subst.
  eapply hash_last_context_binding. symmetry. exact H3.
Qed.

Theorem generated_unrestricted_signature_verifies : forall st e ctx dig,
  failure st = false ->
  transition st e
    (MARS_SignatureVerify false ctx dig
       (CryptSign
          (CryptXkdf (DP st) MARS_LU (KC_Context ctx)) dig)) =
    (set_shc_to_none st, MARS_RC_SUCCESS, Some true).
Proof.
  intros st e ctx dig Hfail.
  unfold transition. rewrite Hfail. cbn.
  rewrite H_verify_correct. reflexivity.
Qed.

Theorem quote_signature_correct : forall st e regSelect nonce ctx,
  failure st = false ->
  valid_select regSelect = true ->
  let base := set_shc_to_none st in
  let '(sampled, snapshot) := CryptSnapshot base e regSelect nonce in
  let ak := CryptXkdf (DP sampled) MARS_LR (KC_Context ctx) in
  let sig := CryptSign ak snapshot in
  transition st e (MARS_Quote regSelect nonce ctx) =
      (sampled, MARS_RC_SUCCESS, Some sig) /\
  CryptVerify ak snapshot sig = true.
Proof.
  intros st e regSelect nonce ctx Hfail Hvalid.
  remember (CryptSnapshot (set_shc_to_none st) e regSelect nonce)
    as result eqn:Hsnapshot.
  destruct result as [sampled snapshot]. cbn.
  split.
  - apply quote_shape; assumption.
  - apply H_verify_correct.
Qed.

Theorem signature_acceptance_sound :
  forall st e restricted ctx dig sig st' rc result,
    failure st = false ->
    transition st e
      (MARS_SignatureVerify restricted ctx dig sig) =
      (st', rc, Some result) ->
    result = true ->
    sig =
      CryptSign
        (CryptXkdf (DP st)
           (if restricted then MARS_LR else MARS_LU)
           (KC_Context ctx))
        dig.
Proof.
  intros st e restricted ctx dig sig st' rc result
         Hfail Htrans Hresult.
  unfold transition in Htrans. rewrite Hfail in Htrans. cbn in Htrans.
  inversion Htrans; subst.
  apply H_verify_sound. exact H2.
Qed.

(* ========================================================================= *)
(* 15. Trace-level invariants                                                 *)
(* ========================================================================= *)

Inductive RunsFrom (start : MARS_STATE) : MARS_STATE -> Prop :=
| runs_refl : RunsFrom start start
| runs_step : forall st e cmd st' rc (out : option (ReturnType cmd)),
    RunsFrom start st ->
    transition st e cmd = (st', rc, out) ->
    RunsFrom start st'.

Theorem runs_preserve_ps : forall start st,
  RunsFrom start st ->
  PS st = PS start.
Proof.
  intros start st Hrun.
  induction Hrun as
      [|current e cmd next rc out Hprefix IH Hstep].
  - reflexivity.
  - rewrite (ps_immutable current e cmd next rc out Hstep).
    exact IH.
Qed.

Theorem runs_preserve_reg_length : forall start st,
  RunsFrom start st ->
  length (regs start) = PROFILE_COUNT_REG ->
  length (regs st) = PROFILE_COUNT_REG.
Proof.
  intros start st Hrun.
  induction Hrun as
      [|current e cmd next rc out Hprefix IH Hstep];
    intro Hlength.
  - exact Hlength.
  - eapply transition_preserves_reg_length.
    + exact Hstep.
    + apply IH. exact Hlength.
Qed.

Theorem runs_preserve_failure_mode : forall start st,
  RunsFrom start st ->
  failure start = true ->
  failure st = true.
Proof.
  intros start st Hrun.
  induction Hrun as
      [|current e cmd next rc out Hprefix IH Hstep];
    intro Hfailure.
  - exact Hfailure.
  - eapply failure_absorbing.
    + apply IH. exact Hfailure.
    + exact Hstep.
Qed.

Definition Reachable (st : MARS_STATE) : Prop := RunsFrom mars_init st.

Definition state_invariant (st : MARS_STATE) : Prop :=
  PS st = PS_INIT /\
  length (regs st) = PROFILE_COUNT_REG.

Theorem reachable_state_invariant : forall st,
  Reachable st ->
  state_invariant st.
Proof.
  intros st Hreachable. split.
  - unfold Reachable in Hreachable.
    rewrite (runs_preserve_ps mars_init st Hreachable).
    apply init_ps.
  - unfold Reachable in Hreachable.
    eapply runs_preserve_reg_length.
    + exact Hreachable.
    + apply init_reg_length.
Qed.

Theorem transition_deterministic :
  forall st e cmd st1 rc1 out1 st2 rc2 out2,
    transition st e cmd = (st1, rc1, out1) ->
    transition st e cmd = (st2, rc2, out2) ->
    st1 = st2 /\ rc1 = rc2 /\ out1 = out2.
Proof.
  intros st e cmd st1 rc1 out1 st2 rc2 out2 H1 H2.
  rewrite H1 in H2. inversion H2. auto.
Qed.

End MARS_MODEL.


(* ========================================================================= *)
(*  Theorem / specification map (for the thesis evaluation chapter)          *)
(*                                                                           *)
(*  Structural invariants -- no cryptographic security hypothesis:           *)
(*    failure_lockout, failure_absorbing,                                    *)
(*    runs_preserve_failure_mode                    §5.3.1                   *)
(*    ps_immutable, runs_preserve_ps                §5.3.2                   *)
(*    dp_only_changed_by_dpderive, dp_reset_correct §5.3.3, §8.4.2           *)
(*    init_pcr_zero, init_tsr_profile_value,                                  *)
(*    transition_preserves_reg_length,                                        *)
(*    pcr_only_changed_by_extend                    §5.3.4, §5.4             *)
(*    sample_selected_tsr, snapshot_preserves_unselected                      *)
(*                                                  §5.3.4.3, §5.6.9         *)
(*    sequence_not_started, sequence_complete_consumes_context,               *)
(*    interleaving_terminates_sequence_all          §8.2                     *)
(*    derive_shape, quote_shape, sign_shape,                                  *)
(*    signature_verify_shape                        §8.4--§8.5               *)
(*    reachable_state_invariant                     global inductive result   *)
(*                                                                           *)
(*  Cryptographic conclusions -- dependent on named ideal assumptions:       *)
(*    pcr_extend_result_nonzero                     H_hash_nonzero            *)
(*    skdf_label_separation, restricted_unrestricted_key_separation           *)
(*                                                  H_skdf_inj, §5.5          *)
(*    dp_derive_fresh_successor                     H_skdf_ne_parent          *)
(*    snapshot_context_binding                      H_hash_inj and            *)
(*                                                  H_hash_update_inj         *)
(*    generated_unrestricted_signature_verifies,                              *)
(*    quote_signature_correct                       H_verify_correct          *)
(*    signature_acceptance_sound                    H_verify_sound            *)
(*                                                                           *)
(*  Requirements intentionally outside the current model:                    *)
(*    - pointer, buffer, alignment, byte-length and MARS_RC_BUFFER behavior;  *)
(*    - support-function internal errors and their transition to failure mode;*)
(*    - observational confidentiality of PS, DP, AK and derived keys (§5.8);  *)
(*    - asymmetric CryptAkdf/PublicRead behavior (this is a symmetric model); *)
(*    - concrete byte serialization, including 32-bit big-endian regSelect.   *)
(* ========================================================================= *)
