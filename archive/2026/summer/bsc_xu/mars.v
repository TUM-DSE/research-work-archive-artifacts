From Coq Require Import Bool.Bool.
From Coq Require Import Arith.Arith.
From Coq Require Import NArith.
From Coq Require Import Lists.List.
From Coq Require Import Lia.

Import ListNotations.

(* ========================================================================= *)
(* Profile Layer                                                             *)
(* ========================================================================= *)

Record PROFILE_CONSTANT : Type := {
  (* profile constants (§6.1) *)
  PROFILE_COUNT_PCR : nat;                         (* "number of consecutive PCR implemented on this MARS" *)
  PROFILE_COUNT_TSR : nat;                         (* "number of consecutive TSR implemented on this MARS" *)
  PROFILE_COUNT_REG : nat := PROFILE_COUNT_PCR + PROFILE_COUNT_TSR;

  (* "A MARS Profile MUST implement at least one PCR. TSR are optional. The maximum number of PCR plus TSR registers allowed is 32." (§5.3.4) *)
  pcr_ge_1  : 1 <= PROFILE_COUNT_PCR;
  reg_le_32 : PROFILE_COUNT_REG <= 32;

  PROFILE_LEN_DIGEST : nat;                        (* "length of a digest that can be processed or produced" *)
  PROFILE_LEN_SIGN : nat;                          (* "length of signature produced by CryptSign()" *)
  PROFILE_LEN_KSYM : nat;                          (* "length of symmetric key produced by CryptSkdf() if implemented, otherwise 0" *) (* in this model, implemented*)
  PROFILE_LEN_KPUB : nat;                          (* "length of public asymmetric key returned by MARS_PublicRead() if implemented, otherwise 0" *) (* in this model, implemented*)
  PROFILE_LEN_KPRV : nat;                          (* "length of asymmetric key produced by CryptAkdf() if implemented, otherwise 0" *) (* in this model, implemented*)
  PROFILE_LEN_XKDF : nat := PROFILE_LEN_KPRV;      (* "PROFILE_LEN_KPRV if defined, else PROFILE_LEN_KSYM" *) (* in this model, PROFILE_LEN_KPRV *)

  PROFILE_ALG_HASH : nat;                          (* "TCG-registered algorithm for hashing by CryptHash functions" *)
  PROFILE_ALG_SIGN : nat;                          (* "TCG-registered algorithm for signing by CryptSign()" *)
  PROFILE_ALG_SKDF : nat;                          (* "TCG-registered algorithm for symmetric key derivation by CryptSkdf()" *)
  PROFILE_ALG_AKDF : nat;                          (* "TCG-registered algorithm for asymmetric key derivation by CryptAkdf()" *)
}.



Record ABSTRACT_TYPE {P : PROFILE_CONSTANT} : Type := {
  (* abstract types *) 
  Digest : Type;
  DIGEST_ZERO : Digest;

  (* profile-specified values of TSRs at init (§5.4) *) 
  TSR_INIT : list Digest;
  TSR_INIT_length : length TSR_INIT = PROFILE_COUNT_TSR P;

  Context : Type;             (* ctx, nonce, ... *)

  Signature : Type;           (* signature *)

  (* secrets, keys (§5.5) *)
  PrimarySeed : Type;         (* primary seed (PS) (§5.3.2) *)      
  PS_INIT: PrimarySeed;       (* initial PS (§5.3.2) *)

  DerivationParent : Type;    (* derivation parent (DP) (§5.3.3) *) 
  DerivedBytes    : Type;     (* derived bytes (§8.4.1) *) 
  RestrictedKey : Type;       (* restricted signingkey (§8.5.1) *) 
  UnRestrictedKey : Type;     (* unrestricted signingkey (§8.5.2) *) 
  PublicKey : Type;           (* the public portion of the specified key (§8.4.3) *)
  extract_pub : RestrictedKey + UnRestrictedKey -> PublicKey; (* it is used to extract the public part of a signningkey *)
}. 



Section ABSTRACT_TYPE_EXTENSION.

Context {P : PROFILE_CONSTANT}.
Context (T: ABSTRACT_TYPE (P := P)).

(* sampled values for TSRs (§5.3.4.3) *)
Record ENV : Type := {
  TSR_SAMPLE : list (Digest T);
  TSR_SAMPLE_length : length TSR_SAMPLE = (PROFILE_COUNT_TSR P);
}.

(* it turns different types into HashContext which can be fed to shc *)
Inductive HashContext : Type :=
| HC_RegSelect (n : N)
| HC_Digest    (digest : (Digest T))
| HC_Context   (ctx : (Context T)).

(* labels for kdf (§5.5), it specifies the input and output of a kdf *)
Inductive Label : Type -> Type -> Type :=
| MARS_LX : Label (Digest T) (DerivedBytes T)
| MARS_LD : Label (Digest T) (DerivationParent T)
| MARS_LU : Label (Context T) (UnRestrictedKey T)
| MARS_LR : Label (Context T) (RestrictedKey T).

End ABSTRACT_TYPE_EXTENSION.



Record SUPPORT_FUNCTION {P : PROFILE_CONSTANT} {T : ABSTRACT_TYPE (P := P)} : Type := {
  (* CryptSelfTest(fullTest), in this model partial self-testing is supported (§5.6.1) *)
  CryptSelfTest : bool -> bool;

  (* CryptHash-related functions (§5.6.2) *)
  (* CryptHashInit(shc) (§5.6.2.1)
    input: nothing
    output: initialized shc *)
  CryptHashInit : list (HashContext T);
  (* CryptHashUpdate(shc, data, len) (§5.6.2.2)
    input: shc, data
    output: updated shc
    in this model, length of data/context is not be modeled*)
  CryptHashUpdate : list (HashContext T) -> (HashContext T) -> list (HashContext T);
  (* CryptHashFinal(shc, out) (§5.6.2.3)
    input: shc
    output: resulting digest *)
  CryptHashFinal : list (HashContext T) -> (Digest T);

  (* CryptSign(key, digest) (§5.6.3)
    input: key, degest
    output: signature *)
  CryptSign : (RestrictedKey T) + (UnRestrictedKey T) -> (Digest T) -> (Signature T);
  (* CryptVerify(key, digest, signature) (§5.6.4)
    input: key, degest, signature
    output: result *)
  CryptVerify : (RestrictedKey T) + (UnRestrictedKey T) -> (Digest T) -> (Signature T)-> bool;

  (* CryptSkdf(child, parent, label, ctx, ctxlen) (§5.6.5) 
    input: parent, label, ctx
    output: child *)
  CryptSkdf : forall {Input Output : Type}, (DerivationParent T) -> (Label T) Input Output -> Input -> Output;
  (* CryptAkdf(child, parent, label, ctx, ctxlen) (§5.6.6) 
    input: parent, label, ctx
    output: child *)
  CryptAkdf : forall {Input Output : Type}, (DerivationParent T) -> (Label T) Input Output -> Input -> Output;
  (* CryptXkdf is CryptAkdf if CryptAkdf is implemented. Otherwise, CryptXkdf is CryptSkdf. (§5.6.7), in this model, CryptAkdf*)
  CryptXkdf {Input Output : Type} (parent : (DerivationParent T)) (label : (Label T) Input Output) (input : Input) : Output := CryptAkdf parent label input;

  (* CryptDpInit() (§5.6.8) 
    input: PS
    output: DP0 *)
  CryptDpInit : (PrimarySeed T) -> (DerivationParent T);
}.



Section Mars.
Context {P : PROFILE_CONSTANT}.
Context {T : ABSTRACT_TYPE (P := P)}.
Context (F : SUPPORT_FUNCTION (P := P) (T := T)).

(* ========================================================================= *)
(* State Layer                                                               *)
(* ========================================================================= *)

(* MARS State Machine (§5.3)
    failure : failure mode flag (§5.3.1)
    ps      : primary seed, persistent(§5.3.2)
    dp      : derivation parent, volatile (§5.3.3)
    regs    : PCR 0, ..., PCR PROFILE_COUNT_PCR-1, TSR 0, ..., TSR PROFILE_COUNT_TSR-1 (§5.3.4)
    shc     : hash-sequence context 'shc' (§5.6.2.1), (§8.2)
              None means no sequence is running; Some carries the list of data chunks accumulated so far*)
Record MARS_STATE : Type := {
  failure : bool;
  PS      : (PrimarySeed T);
  DP      : (DerivationParent T);
  regs    : list (Digest T);
  shc     : option (list (HashContext T))
}.



(* State Update Functions *)
Definition set_failure (st : MARS_STATE) (new_failure : bool) : MARS_STATE :=
  {|
    failure := new_failure;
    PS := PS st;
    DP := DP st;
    regs := regs st;
    shc := shc st
  |}.

Definition set_dp (st : MARS_STATE) (new_DP : (DerivationParent T)) : MARS_STATE :=
  {|
    failure := failure st;
    PS := PS st;
    DP := new_DP;
    regs := regs st;
    shc := shc st
  |}.

Fixpoint update (l : list (Digest T)) (index : nat) (digest : (Digest T)) : list (Digest T) :=
  match l, index with
  | [], _          => []
  | _ :: l', O      => digest :: l'
  | h :: l', S index'   => h :: update l' index' digest
  end.

Definition set_reg (st : MARS_STATE) (index : nat) (digest : (Digest T)) : MARS_STATE :=
  {|
    failure := failure st;
    PS := PS st;
    DP := DP st;
    regs := update (regs st) index digest;
    shc := shc st
  |}.

Definition set_regs (st : MARS_STATE) (new_regs : list (Digest T)) : MARS_STATE :=
  {|
    failure := failure st;
    PS := PS st;
    DP := DP st;
    regs := new_regs;
    shc := shc st
  |}. 

Definition set_shc (st : MARS_STATE) (new_shc : option (list (HashContext T))) : MARS_STATE :=
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



(* _MARS_Init: PCR := 0; TSR := Profile-specified values; failure := false, then a full self-test may set it; DP := CryptDpInit(). *)
Definition mars_init : MARS_STATE :=
  {| failure := negb ((CryptSelfTest F) true);  (* in this model, we do full testing at initialization*)
     PS      := (PS_INIT T);
     DP      := (CryptDpInit F) (PS_INIT T);
     regs    := map (fun i => if i <? (PROFILE_COUNT_PCR P)
                                 then (DIGEST_ZERO T)
                                 else nth (i - PROFILE_COUNT_PCR P) (TSR_INIT T) (DIGEST_ZERO T))
                    (List.seq 0 (PROFILE_COUNT_REG P));
     shc     := None |}.



(* ========================================================================= *)
(* The command interface (§8)                                                *)
(* ========================================================================= *)

(* CryptSnapshot (§5.6.9) *)
Definition valid_select (regSelect : N) : bool :=
  N.eqb (N.shiftr regSelect (N.of_nat (PROFILE_COUNT_REG P))) 0.

Definition sel_indices (regSelect : N) : list nat :=  
  filter (fun i => N.testbit regSelect (N.of_nat i)) (List.seq 0 (PROFILE_COUNT_REG P)).

Definition sel_values (regSelect : N) (regs : list (Digest T)) : list (Digest T) :=
  map (fun i => nth i regs (DIGEST_ZERO T)) (sel_indices regSelect).

Definition sample (regSelect : N) (env : (ENV T)) (regs : list (Digest T)) : list (Digest T) :=
  map (fun i =>
         if ((PROFILE_COUNT_PCR P) <=? i) && N.testbit regSelect (N.of_nat i)
         then nth (i - PROFILE_COUNT_PCR P) ((TSR_SAMPLE T) env) (DIGEST_ZERO T)
         else nth i regs (DIGEST_ZERO T))
      (List.seq 0 (PROFILE_COUNT_REG P)).

(* CryptSnapshot = CryptHash ( regSelect || REG# || ... || REG# || ctx ) *)
Definition CryptSnapshot (st : MARS_STATE) (env : (ENV T)) (regSelect : N) (ctx : (Context T)) : MARS_STATE * (Digest T) :=
  let new_regs := sample regSelect env (regs st) in
  let new_st := set_regs st new_regs in
  let shc1 := (CryptHashInit F) in
  let shc2 := (CryptHashUpdate F) shc1 ((HC_RegSelect T) regSelect) in
  let shc3 := fold_left (CryptHashUpdate F)  (map (fun i => (HC_Digest T) i)(sel_values regSelect (regs new_st))) shc2 in
  let shc4 := (CryptHashUpdate F) shc3 ((HC_Context T) ctx) in
  (new_st, (CryptHashFinal F) shc4).



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



(* commands (§8) *)
Inductive Command : Type :=
| MARS_SelfTest         (fullTest : bool)                                                                 (* §8.1.1 *)
| MARS_CapabilityGet    (pt : PT)                                                                         (* §8.1.2 *)
| MARS_SequenceHash                                                                                       (* §8.2.1 *)
| MARS_SequenceUpdate   (data : (HashContext T))                                                          (* §8.2.2 *)
| MARS_SequenceComplete                                                                                   (* §8.2.3 *)
| MARS_PcrExtend        (pcrIndex : nat) (dig : (Digest T))                                               (* §8.3.1 *)
| MARS_RegRead          (regIndex : nat)                                                                  (* §8.3.2 *)
| MARS_Derive           (regSelect : N) (ctx : (Context T))                                               (* §8.4.1 *)
| MARS_DpDerive         (regSelect : N) (ctx : option (Context T))                                        (* §8.4.2; None = NULL ctx *)
| MARS_PublicRead       (restricted : bool) (ctx : (Context T))                                           (* §8.4.3 *)
| MARS_Quote            (regSelect : N) (nonce : (Context T)) (ctx : (Context T))                         (* §8.5.1 *)
| MARS_Sign             (ctx : (Context T)) (dig : (Digest T))                                            (* §8.5.2 *)
| MARS_SignatureVerify  (restricted : bool) (ctx : (Context T)) (dig : (Digest T)) (sig : (Signature T)). (* §8.5.3 *)



(* response code (§6.2) *)
Inductive MARS_RC : Type :=
| MARS_RC_SUCCESS           (* Command executed as expected *)
| MARS_RC_IO                (* Input / Output or parsing error *) (* It will not be used for this model *) 
| MARS_RC_FAILURE           (* Self-testing placed MARS in failure mode or MARS is otherwise inaccessible *)
| MARS_RC_BUFFER            (* Buffer pointer (null or misaligned) or length invalid *) (* It is memory-level, will not be used for this model *) 
| MARS_RC_COMMAND           (* Command not supported *) (* In this model, all the commands from the specification is modeled, so it will not be used *)
| MARS_RC_VALUE             (* Value out of range or incorrect for command *) (* in this model, property tags are modeled as inductive type, there is no out of range *)
| MARS_RC_REG               (* Invalid register index specified *)
| MARS_RC_SEQ.              (* Sequence not started *)



(* output *)
Definition ReturnType (cmd : Command) : Type :=
    match cmd with
    | MARS_SelfTest _ =>  unit
    | MARS_CapabilityGet _ =>  nat
    | MARS_SequenceHash => unit
    | MARS_SequenceUpdate _ => unit
    | MARS_SequenceComplete => (Digest T)
    | MARS_PcrExtend _ _ => unit
    | MARS_RegRead _ => (Digest T)
    | MARS_Derive _ _ => (DerivedBytes T)
    | MARS_DpDerive _ _=> unit
    | MARS_PublicRead _ _ => (PublicKey T)
    | MARS_Quote _ _ _ => (Signature T) 
    | MARS_Sign _ _ => (Signature T)
    | MARS_SignatureVerify _ _ _ _ => bool
    end.



(* check if it is a sequence command *)
Definition is_seq_cmd (cmd : Command) : bool :=
  match cmd with
  | MARS_SequenceHash | MARS_SequenceUpdate _ | MARS_SequenceComplete => true
  | _ => false
  end.

(* MARS_CapabilityGet (§8.1.2) *)
Definition cap_transition (st : MARS_STATE) (pt : PT) : MARS_STATE * MARS_RC * option nat :=
    match pt with
    | MARS_PT_PCR =>        (set_shc_to_none st, MARS_RC_SUCCESS, Some (PROFILE_COUNT_PCR P))
    | MARS_PT_TSR =>        (set_shc_to_none st, MARS_RC_SUCCESS, Some (PROFILE_COUNT_TSR P))
    | MARS_PT_LEN_DIGEST => (set_shc_to_none st, MARS_RC_SUCCESS, Some (PROFILE_LEN_DIGEST P))
    | MARS_PT_LEN_SIGN =>   (set_shc_to_none st, MARS_RC_SUCCESS, Some (PROFILE_LEN_SIGN P))
    | MARS_PT_LEN_KSYM =>   (set_shc_to_none st, MARS_RC_SUCCESS, Some (PROFILE_LEN_KSYM P))
    | MARS_PT_LEN_KPUB =>   (set_shc_to_none st, MARS_RC_SUCCESS, Some (PROFILE_LEN_KPUB P))
    | MARS_PT_LEN_KPRV =>   (set_shc_to_none st, MARS_RC_SUCCESS, Some (PROFILE_LEN_KPRV P))
    | MARS_PT_ALG_HASH =>   (set_shc_to_none st, MARS_RC_SUCCESS, Some (PROFILE_ALG_HASH P))
    | MARS_PT_ALG_SIGN =>   (set_shc_to_none st, MARS_RC_SUCCESS, Some (PROFILE_ALG_SIGN P))
    | MARS_PT_ALG_SKDF =>   (set_shc_to_none st, MARS_RC_SUCCESS, Some (PROFILE_ALG_SKDF P))
    | MARS_PT_ALG_AKDF =>   (set_shc_to_none st, MARS_RC_SUCCESS, Some (PROFILE_ALG_AKDF P))
    end.

(* transition function *)                                    
Definition transition (st : MARS_STATE) (env : (ENV T)) (cmd : Command) : MARS_STATE * MARS_RC * option (ReturnType cmd) :=
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
        let new_failure := negb((CryptSelfTest F)(fullTest))in
        (set_failure st new_failure, if new_failure then MARS_RC_FAILURE else MARS_RC_SUCCESS, None)

    (* MARS_CapabilityGet (uint16_t pt, void * cap, uint16_t caplen) *)
    | MARS_CapabilityGet pt => cap_transition st pt

    (* MARS_SequenceHash () *)
    | MARS_SequenceHash => 
        let shc1 := (CryptHashInit F) in
        (set_shc st (Some shc1), MARS_RC_SUCCESS, None)
    (* MARS_SequenceUpdate (const void * in, size_t inlen, void * out, size_t * outlen) *)
    (* "MAY, depending on the MARS_SequenceFunc() algorithm, produce 
       additional output that SHALL be written to the output buffer
       specified.", in this model, it doesn't produce. *)
    | MARS_SequenceUpdate data =>
        match shc st with
        | None   => (st, MARS_RC_SEQ, None)
        | Some shc' => 
            let shc1 := (CryptHashUpdate F) shc' data in
            (set_shc st (Some shc1), MARS_RC_SUCCESS, None)
        end

    (* MARS_SequenceComplete (void * out, size_t * outlen) *)
    | MARS_SequenceComplete =>
        match shc st with
        | None   => (st, MARS_RC_SEQ, None)
        | Some shc' => 
            let out := (CryptHashFinal F) shc' in 
            (set_shc_to_none st, MARS_RC_SUCCESS, Some out)
        end

    (* MARS_PcrExtend (uint16_t pcrIndex, const void * dig) *)
    | MARS_PcrExtend pcrIndex dig =>
        if pcrIndex <? (PROFILE_COUNT_PCR P)
        then 
            let shc1 := (CryptHashInit F) in
            let shc2 := (CryptHashUpdate F) shc1 ((HC_Digest T) (nth pcrIndex (regs st) (DIGEST_ZERO T))) in
            let shc3 := (CryptHashUpdate F) shc2 ((HC_Digest T) dig) in
            let x := (CryptHashFinal F) shc3 in
            (set_reg st pcrIndex x, MARS_RC_SUCCESS, None)
        else (st, MARS_RC_REG, None)

    (* MARS_RegRead (uint16_t regIndex, void * dig) *)
    | MARS_RegRead regIndex => 
        if regIndex <? (PROFILE_COUNT_REG P)
        then 
            let dig := nth regIndex (regs st) (DIGEST_ZERO T) in
            (st, MARS_RC_SUCCESS, Some dig)
        else (st, MARS_RC_REG, None)

    (* MARS_Derive (uint32_t regSelect, const void * ctx, uint16_t ctxlen, void * out)
       out := CryptSkdf(DP, MARS_LX, CryptSnapshot(regSelect, ctx)) *)
    | MARS_Derive regSelect ctx =>
        if valid_select regSelect
        then
            let (st', snapshot) := CryptSnapshot st env regSelect ctx in
            let out := (CryptSkdf F) (DP st') (MARS_LX T) snapshot in
            (st', MARS_RC_SUCCESS, Some out)
        else (st, MARS_RC_REG, None)

    (* MARS_DpDerive (uint32_t regSelect, const void * ctx, uint16_t ctxlen)
       ctx = Some c : DP := CryptSkdf(DP, MARS_LD, snapshot)
       ctx = None   : DP := CryptDpInit()  (no snapshot, no TSR sampling) *)
    | MARS_DpDerive regSelect ctx =>
        if valid_select regSelect
        then
            match ctx with
            | Some ctx' =>
                let (st', snapshot) := CryptSnapshot st env regSelect ctx' in
                let DP' := (CryptSkdf F) (DP st') (MARS_LD T) snapshot in
                (set_dp st' DP', MARS_RC_SUCCESS, None)
            | None =>
                let DP' := ((CryptDpInit F) (PS st)) in
                (set_dp st DP', MARS_RC_SUCCESS, None)
            end
        else (st, MARS_RC_REG, None)        

    (* MARS_PublicRead (bool restricted, const void * ctx, uint16_t ctxlen, void * pub) *)
    | MARS_PublicRead restricted ctx => 
        if restricted
        then
            let label :=  (MARS_LR T) in
            let key := (CryptAkdf F) (DP st) label ctx in
            let pub := (extract_pub T) (inl key) in
            (st, MARS_RC_SUCCESS, Some pub)    
        else
            let label :=  (MARS_LU T) in
            let key := (CryptAkdf F) (DP st) label ctx in
            let pub := (extract_pub T) (inr key) in
            (st, MARS_RC_SUCCESS, Some pub) 

    (* MARS_Quote (uint32_t regSelect, const void * nonce, uint16_t nlen, const void * ctx, uint16_t ctxlen, void * sig)
       (st', snapshot) := CryptSnapshot(st, regSelect, nonce)
       AK   := CryptXkdf(DP, MARS_LR, ctx)
       sig  := CryptSign(AK, snapshot) *)
    | MARS_Quote regSelect nonce ctx =>
        if valid_select regSelect
        then
            let (st', snapshot) := CryptSnapshot st env regSelect nonce in
            let AK := (CryptXkdf F) (DP st') (MARS_LR T) ctx in
            let sig := (CryptSign F) (inl AK) snapshot in
            (st', MARS_RC_SUCCESS, Some sig)
        else (st, MARS_RC_REG, None)

    (* MARS_Sign (const void * ctx, uint16_t ctxlen, const void * dig, void * sig)
       key := CryptXkdf(DP, MARS_LU, ctx)
       sig := CryptSign(key, dig) *)
    | MARS_Sign ctx dig => 
        let key := (CryptXkdf F) (DP st) (MARS_LU T) ctx in
        let sig := (CryptSign F) (inr key) dig in
        (st, MARS_RC_SUCCESS, Some sig)

    (* MARS_SignatureVerify (bool restricted, const void * ctx, uint16_t ctxlen, const void * dig, const void * sig, bool * result)
       key := CryptXkdf(DP, restricted ? MARS_LR : MARS_LU, ctx)
       result := CryptVerify(key, dig, sig) *)   
    | MARS_SignatureVerify restricted ctx dig sig =>
        if restricted
        then
            let label :=  (MARS_LR T) in
            let key := (CryptXkdf F) (DP st) label ctx in
            let result := (CryptVerify F) (inl key) dig sig in
            (st, MARS_RC_SUCCESS, Some result)
        else
            let label :=  (MARS_LU T) in
            let key := (CryptXkdf F) (DP st) label ctx in
            let result := (CryptVerify F) (inr key) dig sig in
            (st, MARS_RC_SUCCESS, Some result)
    end.



(* ========================================================================= *)
(* Properties and Invariants                                                 *)
(* ========================================================================= *)

(*
   The remainder of this file states and proves the properties/invariants extracted from the MARS Library Specification v1r14:
   [P1]  PS persistent (§5.3.2):                  the Primary Seed is never modified                                                          -> ps_persistent
   [P2]  register-count invariant (§5.3.4):       length regs = PROFILE_COUNT_REG is preserved by every command                               -> transition_preserves_wf
   [P3]  failure-mode lockout (§5.3.1):           in failure mode the state is frozen                                                         -> failure_mode_freezes_state
                                                  everything but MARS_CapabilityGet returns MARS_RC_FAILURE                                   -> failure_mode_rc 
                                                  MARS_CapabilityGet still succeeds                                                           -> capability_get_available_in_failure
   [P4]  failure is raised only by (failed) self-testing (§5.2, §8.1.1)                                                                       -> failure_raised_only_by_selftest
         failure mode is entered iff a self-test fails (§5.2, §8.1.1)                                                                         -> selftest_semantics
   [P5]  sequence discipline (S8.2):              non-sequence commands terminate an running sequence                                         -> nonseq_command_terminates_sequence
                                                  Update/Complete without a started sequence return MARS_RC_SEQ and leave the state unchanged -> sequence_update_requires_start, sequence_complete_requires_start
   [P6]  registers discipline (§5.3.4.2, §8.3.1): PCR are modified only by MARS_PcrExtend                                                     -> pcr_modified_only_by_pcr_extend
                                                  out-of-range indices are rejected                                                           -> pcr_extend_invalid_index
                                                  other registers are framed                                                                  -> pcr_extend_frame                                            
                                                  TSR not extenable                                                                           -> pcr_extend_cannot_touch_tsr
   [P7]  DP discipline (§5.3.3, §8.4.2):          DP changes only via MARS_DpDerive                                                           -> dp_modified_only_by_dpderive
   [P8]  invalid regSelect rejection (§5.3.4.1)                                                                                               -> invalid_regselect_rejected
   [P9]  reachability:                            invariants along every trace                                                                -> reachable_wf, reachable_ps_persistent
*)




(* the state after transition *)
Definition state_of_transition (st : MARS_STATE) (env : (ENV T)) (cmd : Command) : MARS_STATE :=
  fst (fst (transition st env cmd)).
(* the response code after transition *)
Definition rc_of_transition (st : MARS_STATE) (env : (ENV T)) (cmd : Command) : MARS_RC :=
  snd (fst (transition st env cmd)).



(* list helpers *)
Lemma update_length : forall l index digest, length (update l index digest) = length l.
Proof. induction l; destruct index; simpl; auto. Qed.

Lemma nth_update_eq : forall l index digest default, index < length l -> nth index (update l index digest) default = digest.
Proof.
  induction l; destruct index; simpl; intros; try lia; auto.
  apply IHl; lia.
Qed.

Lemma nth_update_neq : forall l index jndex digest default, jndex <> index -> nth jndex (update l index digest) default = nth jndex l default.
Proof.
  induction l; destruct index, jndex; simpl; intros; try congruence; auto.
Qed.

Lemma nth_map_seq :
  forall (A : Type) (f : nat -> A) (d : A) (n index : nat),
    index < n -> nth index (map f (seq 0 n)) d = f index.
Proof.
  intros A f d n index Hi.
  rewrite nth_indep with (d' := f 0).
  - rewrite map_nth, seq_nth by lia. reflexivity.
  - rewrite length_map, length_seq. exact Hi.
Qed.

Lemma sample_length : forall regSelect env regs, length (sample regSelect env regs) = (PROFILE_COUNT_REG P).
Proof. intros; unfold sample; now rewrite length_map, length_seq. Qed.

Lemma sample_preserves_pcr :
  forall regSelect env regs pcrIndex, pcrIndex < (PROFILE_COUNT_PCR P) ->
    nth pcrIndex (sample regSelect env regs) (DIGEST_ZERO T) = nth pcrIndex regs (DIGEST_ZERO T).
Proof.
  intros regSelect env regs pcrIndex Hi.
  unfold sample. rewrite nth_map_seq.
  - destruct ((PROFILE_COUNT_PCR P) <=? pcrIndex) eqn:E.
    + apply Nat.leb_le in E; lia.
    + reflexivity.
  - unfold PROFILE_COUNT_REG; lia.
Qed.



(* snapshot frame *)
Lemma snapshot_frame :
  forall st env regSelect ctx,
    let st' := fst (CryptSnapshot st env regSelect ctx) in
    PS st' = PS st /\ DP st' = DP st /\ failure st' = failure st
    /\ shc st' = shc st /\ regs st' = sample regSelect env (regs st).
Proof. intros; unfold CryptSnapshot; simpl; repeat split; reflexivity. Qed.



(* generic case-analysis tactic over the transition function *)
Ltac step :=
  unfold state_of_transition, rc_of_transition, transition, cap_transition, CryptSnapshot,
         set_failure, set_dp, set_reg, set_regs, set_shc, set_shc_to_none;
  simpl.

Ltac branches :=
  repeat first
    [ match goal with
      | |- context [ if ?b then _ else _ ] => destruct b eqn:?
      | |- context [ match ?x with Some _ => _ | None => _ end ] => destruct x eqn:?
      | |- context [ match ?x with (_, _) => _ end ] => destruct x eqn:?
      end
    ; simpl ].



(* [P1] PS persistence (§5.3.2) *)
Theorem ps_persistent : 
  forall st env cmd, 
    PS (state_of_transition st env cmd) 
      = PS st.
Proof.
  intros st env cmd.
  destruct cmd; step; branches; 
    try reflexivity;
    try (destruct pt; reflexivity).
Qed.



(* [P2] register-count invariant (§5.3.4) *)
(* well-formed state *)
Definition well_formed_st (st : MARS_STATE) : Prop :=
  length (regs st) = (PROFILE_COUNT_REG P).

Theorem transition_preserves_wf :
  forall st env cmd, well_formed_st st -> well_formed_st (state_of_transition st env cmd).
Proof.
  intros st env cmd Hwf. unfold well_formed_st in *.
  destruct cmd; step; branches; simpl; auto;
  try (destruct pt; simpl; auto);
  try (rewrite update_length; auto);
  try apply sample_length.
Qed.



(* [P3] failure-mode lockout (§5.3.1) *)
Theorem failure_mode_freezes_state :
  forall st env cmd, failure st = true ->
    (forall pt, cmd <> MARS_CapabilityGet pt) ->
    state_of_transition st env cmd = st.
Proof.
  intros st env cmd Hf Hnot. unfold state_of_transition, transition. rewrite Hf.
  destruct cmd; try reflexivity.
  exfalso; eapply Hnot; reflexivity.
Qed.

Theorem failure_mode_rc :
  forall st env cmd, failure st = true ->
    (forall pt, cmd <> MARS_CapabilityGet pt) ->
    rc_of_transition st env cmd = MARS_RC_FAILURE.
Proof.
  intros st env cmd Hf Hnot. unfold rc_of_transition, transition. rewrite Hf.
  destruct cmd; try reflexivity.
  exfalso; eapply Hnot; reflexivity.
Qed.

Theorem capability_get_available_in_failure :
  forall st env pt, failure st = true ->
    rc_of_transition st env (MARS_CapabilityGet pt) = MARS_RC_SUCCESS.
Proof.
  intros st env pt Hf. unfold rc_of_transition, transition. rewrite Hf.
  destruct pt; reflexivity.
Qed.


(* [P4] failure flag can only be raised by MARS_SelfTest (in this model) *)
Theorem failure_raised_only_by_selftest :
  forall st env cmd, failure st = false ->
    (forall fullTest, cmd <> MARS_SelfTest fullTest) ->
    failure (state_of_transition st env cmd) = false.
Proof.
  intros st env cmd Hf Hnot.
  destruct cmd; try (exfalso; eapply Hnot; reflexivity);
  step; rewrite Hf; branches; simpl; auto;
  try (destruct pt; simpl; auto).
Qed.

(* failure mode is entered iff a self-test fails (§5.2, §8.1.1) *)
Theorem selftest_semantics :
  forall st env fullTest, failure st = false ->
    failure (state_of_transition st env (MARS_SelfTest fullTest)) = negb ((CryptSelfTest F) fullTest)
    /\ rc_of_transition st env (MARS_SelfTest fullTest)
       = (if (CryptSelfTest F) fullTest then MARS_RC_SUCCESS else MARS_RC_FAILURE).
Proof.
  intros st env fullTest Hf. step. rewrite Hf. simpl.
  destruct ((CryptSelfTest F) fullTest); simpl; auto.
Qed.



(* [P5] sequence discipline (§8.2) *)

(* any non-sequence command terminates an running sequence *)
Theorem nonseq_command_terminates_sequence :
  forall st env cmd, failure st = false -> is_seq_cmd cmd = false ->
    shc (state_of_transition st env cmd) = None.
Proof.
  intros st env cmd Hf Hseq.
  destruct cmd; simpl in Hseq; try discriminate;
  step; rewrite Hf; branches; simpl; auto;
  try (destruct pt; simpl; auto).
Qed.

(* Update/Complete without a started sequence: MARS_RC_SEQ, state unchanged *)
Theorem sequence_update_requires_start :
  forall st env data, failure st = false -> shc st = None ->
    rc_of_transition st env (MARS_SequenceUpdate data) = MARS_RC_SEQ
    /\ state_of_transition st env (MARS_SequenceUpdate data) = st.
Proof.
  intros st env data Hf Hs. step. rewrite Hf, Hs. auto.
Qed.

Theorem sequence_complete_requires_start :
  forall st env, failure st = false -> shc st = None ->
    rc_of_transition st env MARS_SequenceComplete = MARS_RC_SEQ
    /\ state_of_transition st env MARS_SequenceComplete = st.
Proof.
  intros st env Hf Hs. step. rewrite Hf, Hs. auto.
Qed.



(* [P6] REG discipline (§5.3.4.2, §8.3.1) *)
(* PCR values can change only through MARS_PcrExtend (§5.3.4.2): every other command preserves every PCR *)
Theorem pcr_modified_only_by_pcr_extend :
  forall st env cmd, failure st = false ->
    (forall pcrIndex dig, cmd <> MARS_PcrExtend pcrIndex dig) ->
    forall pcrJndex, pcrJndex < (PROFILE_COUNT_PCR P) ->
      nth pcrJndex (regs (state_of_transition st env cmd)) (DIGEST_ZERO T) = nth pcrJndex (regs st) (DIGEST_ZERO T).
Proof.
  intros st env cmd Hf Hnot pcrJndex Hj.
  destruct cmd; try (exfalso; eapply Hnot; reflexivity);
  step; rewrite Hf; branches; simpl; auto;
  try (destruct pt; simpl; auto);
  try (apply sample_preserves_pcr; auto).
Qed.



(* out-of-range index is rejected without touching registers *)
Theorem pcr_extend_invalid_index :
  forall st env pcrIndex dig, failure st = false -> (PROFILE_COUNT_PCR P) <= pcrIndex ->
    rc_of_transition st env (MARS_PcrExtend pcrIndex dig) = MARS_RC_REG
    /\ regs (state_of_transition st env (MARS_PcrExtend pcrIndex dig)) = regs st.
Proof.
  intros st env pcrIndex dig Hf Hi. step. rewrite Hf.
  destruct (pcrIndex <? (PROFILE_COUNT_PCR P)) eqn:E; simpl.
  - apply Nat.ltb_lt in E; lia.
  - auto.
Qed.



(* extending PCR pcrIndex leaves every other register untouched *)
Theorem pcr_extend_frame :
  forall st env pcrIndex dig pcrJndex, failure st = false -> pcrJndex <> pcrIndex ->
    nth pcrJndex (regs (state_of_transition st env (MARS_PcrExtend pcrIndex dig))) (DIGEST_ZERO T)
    = nth pcrJndex (regs st) (DIGEST_ZERO T).
Proof.
  intros st env pcrIndex dig pcrJndex Hf Hij. step. rewrite Hf.
  destruct (pcrIndex <? (PROFILE_COUNT_PCR P)); simpl; auto.
  apply nth_update_neq; auto.
Qed.



(* TSRs are not extendable (§5.3.4.3): MARS_PcrExtend can never modify a TSR, whether the index aliases a TSR (rejected) or targets another register *)
Theorem pcr_extend_cannot_touch_tsr :
  forall st env i d j, failure st = false -> (PROFILE_COUNT_PCR P) <= j ->
    nth j (regs (state_of_transition st env (MARS_PcrExtend i d))) (DIGEST_ZERO T)
    = nth j (regs st) (DIGEST_ZERO T).
Proof.
  intros st env i d j Hf Hj.
  destruct (Nat.eq_dec j i) as [->|Hne].
  - destruct (pcr_extend_invalid_index st env i d Hf Hj) as [_ ->]. reflexivity.
  - now apply pcr_extend_frame.
Qed.



(* [P7]  DP discipline (S5.3.3, S8.4.2) *)
(* the DP can change only through MARS_DpDerive *)
Theorem dp_modified_only_by_dpderive :
  forall st env cmd,
    (forall regSelect c, cmd <> MARS_DpDerive regSelect c) ->
    DP (state_of_transition st env cmd) = DP st.
Proof.
  intros st env cmd Hnot.
  destruct cmd; try (exfalso; eapply Hnot; reflexivity);
  step; branches; simpl; auto;
  try (destruct pt; simpl; auto).
Qed.



(* [P8] invalid regSelect is rejected with MARS_RC_REG, leaving regs and DP intact *)
Theorem invalid_regselect_rejected :
  forall st env regSelect ctx nonce octx,
    failure st = false -> valid_select regSelect = false ->
       rc_of_transition st env (MARS_Derive regSelect ctx) = MARS_RC_REG
    /\ rc_of_transition st env (MARS_DpDerive regSelect octx) = MARS_RC_REG
    /\ rc_of_transition st env (MARS_Quote regSelect nonce ctx) = MARS_RC_REG
    /\ regs (state_of_transition st env (MARS_Derive regSelect ctx)) = regs st
    /\ regs (state_of_transition st env (MARS_DpDerive regSelect octx)) = regs st
    /\ regs (state_of_transition st env (MARS_Quote regSelect nonce ctx)) = regs st
    /\ DP (state_of_transition st env (MARS_DpDerive regSelect octx)) = DP st.
Proof.
  intros st env regSelect ctx nonce octx Hf Hv.
  repeat split; step; rewrite Hf, Hv; reflexivity.
Qed.



(* [P9] reachability: invariants along every trace *)
Theorem init_wf : well_formed_st mars_init.
Proof.
  unfold well_formed_st, mars_init; simpl.
  now rewrite length_map, length_seq.
Qed.

Inductive reachable : MARS_STATE -> Prop :=
| reach_init : reachable mars_init
| reach_step : forall st env cmd, reachable st -> reachable (state_of_transition st env cmd).

Theorem reachable_wf : forall st, reachable st -> well_formed_st st.
Proof.
  induction 1; [ exact init_wf | now apply transition_preserves_wf ].
Qed.

(* the Primary Seed is persistent for the lifetime of the device (§5.3.2) *)
Theorem reachable_ps_persistent :
  forall st, reachable st -> PS st = (PS_INIT T).
Proof.
  induction 1; [ reflexivity | now rewrite ps_persistent ].
Qed.

End Mars.