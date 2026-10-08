From Coq Require Import Bool.Bool.
From Coq Require Import Arith.Arith.
From Coq Require Import Lists.List.
From Coq Require Import Lia.

Import ListNotations.



(* Profile Constants *)
Parameter  PROFILE_COUNT_PCR : nat.
Parameter  PROFILE_COUNT_TSR : nat.
Parameter  PROFILE_COUNT_REG : nat .

Parameter  PROFILE_LEN_DIGEST : nat.
Parameter  PROFILE_LEN_SIGN : nat.
Parameter  PROFILE_LEN_KSYM : nat.
Parameter  PROFILE_LEN_KPUB : nat.
Parameter  PROFILE_LEN_KPRV : nat.
Parameter  PROFILE_LEN_XKDF : nat.

Parameter  PROFILE_ALG_HASH : nat.
Parameter  PROFILE_ALG_SIGN : nat.
Parameter  PROFILE_ALG_SKDF : nat.
Parameter  PROFILE_ALG_AKDF : nat.



(* Response Code *)
Inductive mars_rc : Type :=
| MARS_RC_SUCCESS
| MARS_RC_IO
| MARS_RC_FAILURE
| MARS_RC_BUFFERbytes 
| MARS_RC_COMMAND
| MARS_RC_VALUE
| MARS_RC_REG
| MARS_RC_SEQ.



(* Cryptographic Key Labels *)
Inductive label : Type :=
| MARS_LX   (* eXternal *)
| MARS_LD   (* Derivation Parent *)
| MARS_LU   (* Unrestricted signing *)
| MARS_LR.  (* Restricted attestation *)



(* Abstract Objects *)
Parameter digest : Type.
Parameter secret : Type.
Parameter public : Type.
Parameter signature : Type.
Parameter bytes : Type. 

(* convert a digest to bytes *)
Parameter digest_to_bytes : digest -> bytes.

(* PCR initial value. *)
Parameter digest_zero : digest.
(* TSR initial value. *)
Parameter tsr_initial_value : nat -> digest.




(* Self-test result. *)
Parameter crypt_self_test : bool -> bool.




(* shc initial value *)
Parameter crypt_hash_init : bytes.
Parameter crypt_hash_update : bytes -> bytes -> bytes.
Parameter crypt_hash_final : bytes -> digest.



(* Sign / verify. *)
Parameter crypt_sign : secret -> digest -> signature.
Parameter crypt_verify : secret -> digest -> signature -> bool.




(* KDF for keys and new DP. *)
Parameter crypt_skdf : secret -> label -> bytes -> secret.
Parameter crypt_akdf : secret -> label -> bytes -> secret.
Parameter crypt_xkdf : secret -> label -> bytes -> secret.
Parameter public_key : secret -> public.



(* PS -> initial DP *)
Parameter crypt_dp_init : secret -> secret.

(* snapshot: Hash (regSelect || REG# || … || REG# || ctx) *)
Parameter crypt_snapshot : (nat -> digest) -> list nat -> bytes -> digest.



(* state machine *)
Record mars_state : Type := {
  failure : bool;
  ps : secret;
  dp : secret;
  reg : nat -> digest;
  shc : option bytes
}.


(* State Update Functions  *)
Definition update_reg (regs : nat -> digest) (i : nat) (v : digest) : nat -> digest :=
  fun j => if j =? i then v else regs j.

Definition set_failure (new_failure : bool) (st : mars_state) : mars_state :=
  {|
    failure := new_failure;
    ps := ps st;
    dp := dp st;
    reg := reg st;
    shc := shc st
  |}.

Definition set_dp (new_dp : secret) (st : mars_state) : mars_state :=
  {|
    failure := failure st;
    ps := ps st;
    dp := new_dp;
    reg := reg st;
    shc := shc st
  |}.

Definition set_reg (i : nat) (v : digest) (st : mars_state) : mars_state :=
  {|
    failure := failure st;
    ps := ps st;
    dp := dp st;
    reg := update_reg (reg st) i v;
    shc := shc st
  |}.

Definition set_shc (new_shc : option bytes) (st : mars_state) : mars_state :=
  {|
    failure := failure st;
    ps := ps st;
    dp := dp st;
    reg := reg st;
    shc := new_shc
  |}.




(* Register Validity Functions *)
Definition valid_pcr_index (i : nat) : bool := i <? PROFILE_COUNT_PCR.
Definition valid_reg_index (i : nat) : bool := i <? PROFILE_COUNT_REG.
Definition valid_reg_select (rs : list nat) : bool := forallb valid_reg_index rs.



(* Initialization *)
Definition reg_initial_value (i : nat) : digest :=
  if i <? PROFILE_COUNT_PCR
  then digest_zero
  else tsr_initial_value i.

Definition MARS_Init (st : mars_state) : mars_state :=
  {|
    failure := false;
    ps := ps st;
    dp := crypt_dp_init (ps st);
    reg := reg_initial_value;
    shc := None
  |}.




(* Management *)
Definition MARS_SelfTest (fullTest : bool) (st : mars_state) : mars_rc * mars_state :=
  let test_ok := crypt_self_test fullTest in
  let new_failure := (failure st) || (negb test_ok) in
  let st1 := set_shc None st in
  let st2 := set_failure new_failure st1 in
  if new_failure
  then (MARS_RC_FAILURE, st2)
  else (MARS_RC_SUCCESS, st2).


(* Definition MARS_CapabilityGet (st : mars_state) : mars_state := st. *)




(* Sequence Primitives *)
Definition MARS_SequenceHash (st : mars_state) : mars_rc * mars_state :=
  if failure st
  then (MARS_RC_FAILURE, set_shc None st)
  else (MARS_RC_SUCCESS, set_shc (Some crypt_hash_init) st).

Definition MARS_SequenceUpdate (data : bytes) (st : mars_state) : mars_rc * mars_state :=
  if failure st
  then (MARS_RC_FAILURE, set_shc None st)
  else
    match shc st with
    | None => (MARS_RC_SEQ, st)
    | Some old => let new := crypt_hash_update old data in 
                        (MARS_RC_SUCCESS, set_shc (Some new) st)
    end.

Definition MARS_SequenceComplete (st : mars_state) : mars_rc * option digest * mars_state :=
  if failure st
  then (MARS_RC_FAILURE, None, set_shc None st)
  else
    match shc st with
    | None => (MARS_RC_SEQ, None, st)
    | Some old => let dig := crypt_hash_final old in
                        (MARS_RC_SUCCESS, Some dig, set_shc None st)
    end.



(* Integrity Collection *)
Definition MARS_PcrExtend (pcr_index : nat) (dig : digest) (st : mars_state) : mars_rc * mars_state :=
  if failure st
  then (MARS_RC_FAILURE, set_shc None st)
  else if (valid_pcr_index pcr_index)
       then let shc_init := crypt_hash_init in 
            let digest_old := reg st pcr_index in
            let shc_new := crypt_hash_update shc_init (digest_to_bytes digest_old) in
            let shc_new_new := crypt_hash_update shc_new (digest_to_bytes dig) in
            let dige_new := crypt_hash_final shc_new_new in 
            (MARS_RC_SUCCESS, set_shc None (set_reg pcr_index dige_new st))
       else (MARS_RC_REG, st).

Definition MARS_RegRead (reg_index : nat) (st : mars_state) : mars_rc * option digest * mars_state :=
  if failure st
  then (MARS_RC_FAILURE, None, set_shc None st)
  else if (valid_reg_index reg_index)
       then (MARS_RC_SUCCESS, Some (reg st reg_index), set_shc None st)
       else (MARS_RC_REG, None, set_shc None st).




(* Key Management *)
Definition MARS_Derive (reg_select : list nat) (ctx : bytes) (st : mars_state) : mars_rc * option secret * mars_state :=
  if failure st
  then (MARS_RC_FAILURE, None, set_shc None st)
  else if (valid_reg_select reg_select)
       then  let snap := crypt_snapshot (reg st) reg_select ctx in
             let out := crypt_skdf (dp st) MARS_LX (digest_to_bytes snap) in
             (MARS_RC_SUCCESS, Some out, set_shc None st)
       else (MARS_RC_REG, None, set_shc None st).
        

Definition MARS_DpDerive (reg_select : list nat) (ctx : option bytes) (st : mars_state) : mars_rc * mars_state :=
  if failure st
  then (MARS_RC_FAILURE, set_shc None st)
  else if (valid_reg_select reg_select)
       then match ctx with
            | None => let new_dp := crypt_dp_init (ps st) in
                      (MARS_RC_SUCCESS, set_shc None (set_dp new_dp st))
            | Some c => let snap := crypt_snapshot (reg st) reg_select c in
                        let new_dp := crypt_skdf (dp st) MARS_LD (digest_to_bytes snap) in
                        (MARS_RC_SUCCESS, set_shc None (set_dp new_dp st))
            end
       else (MARS_RC_REG, set_shc None st).

Definition MARS_PublicRead (restricted : bool) (ctx : bytes) (st : mars_state) : mars_rc * option public * mars_state :=
  if failure st
  then (MARS_RC_FAILURE, None, set_shc None st)
  else
    let lab := if restricted then MARS_LR else MARS_LU in
    let k := crypt_akdf (dp st) lab ctx in
    (MARS_RC_SUCCESS, Some (public_key k), set_shc None st).



(* Attestation *)
Definition MARS_Quote (reg_select : list nat) (nonce : bytes) (ctx : bytes) (st : mars_state) : mars_rc * option signature * mars_state :=
  if failure st
  then (MARS_RC_FAILURE, None, set_shc None st)
  else if (valid_reg_select reg_select)
       then let snap := crypt_snapshot (reg st) reg_select nonce in
            let ak := crypt_xkdf (dp st) MARS_LR ctx in
            let sig := crypt_sign ak snap in
         (MARS_RC_SUCCESS, Some sig, set_shc None st)
       else (MARS_RC_REG, None, set_shc None st).

Definition MARS_Sign (ctx : bytes) (dig : digest) (st : mars_state) : mars_rc * option signature * mars_state :=
  if failure st
  then (MARS_RC_FAILURE, None, set_shc None st)
  else
    let k := crypt_xkdf (dp st) MARS_LU ctx in
    let sig := crypt_sign k dig in
    (MARS_RC_SUCCESS, Some sig, set_shc None st).

Definition MARS_SignatureVerify (restricted : bool) (ctx : bytes) (dig : digest) (sig : signature) (st : mars_state) : mars_rc * option bool * mars_state :=
  if failure st
  then (MARS_RC_FAILURE, None, set_shc None st)
  else let lab := if restricted then MARS_LR else MARS_LU in
       let k := crypt_xkdf (dp st) lab ctx in
       let result := crypt_verify k dig sig in
       (MARS_RC_SUCCESS, Some result, set_shc None st).



(* tsr_update *)
(* snapshot *)
(* secret, key *)
(* length of map *)
