mod ascii_from_wrapping_integer;

use ascii_from_wrapping_integer::AsciiFromWrappingInteger;
use strum_macros::EnumIter;

use super::{Instruction, PushInstruction, instruction_error::PushInstructionError};
use crate::{
    error::{InstructionResult, MapInstructionError},
    push_vm::HasStack,
};

/// The variants of `CharInstruction` represent all the instructions that are
/// available the primarily act on the character stack.
#[derive(Debug, strum_macros::Display, Copy, Clone, PartialEq, Eq, EnumIter)]
#[non_exhaustive]
#[must_use]
pub enum CharInstruction {
    /// Push a character onto the character stack
    ///
    /// This is typically used to push constant values or input values
    /// onto the character stack.
    #[strum(to_string = "Push({0})")]
    Push(char),

    /// Convert the top of the integer stack to a character,
    /// and push that character to the top of the character stack.
    /// To ensure that the integer is a legal ASCII code, we'll
    /// take it mod 128 before converting it to a character.
    /// Note that this does *not* support more complex Unicode
    /// characters.
    AsciiFromWrappingInteger(AsciiFromWrappingInteger),
}

impl From<CharInstruction> for PushInstruction {
    fn from(instr: CharInstruction) -> Self {
        Self::CharInstruction(instr)
    }
}

impl<S> Instruction<S> for CharInstruction
where
    S: Clone + HasStack<char> + HasStack<bool> + HasStack<i64>,
{
    type Error = PushInstructionError;

    fn perform(&self, state: S) -> InstructionResult<S, Self::Error> {
        match self {
            Self::Push(c) => state.with_push(*c).map_err_into(),
            Self::AsciiFromWrappingInteger(ascii_wrap) => ascii_wrap.perform(state),
        }
    }
}

/*
All the `char` instructions from Clojush:
https://github.com/lspector/Clojush/blob/master/src/clojush/instructions/char.clj

- allfromstring (push all the chars in a string on the char stack)
- frominteger
   - Mod by 128 to force an ASCII code
- fromfloat
   - Convert to int and mod by 128
- isletter
- isdigit
- iswhitespace
- touppercase
- tolowercase
   - Leave char alone if not a letter

(define-registered
  char_allfromstring
  ^{:stack-types [:char :string]}
  (fn [state]
    (if (empty? (:string state))
      state
      (loop [char-list (reverse (top-item :string state))
             loop-state (pop-item :string state)]
        (if (empty? char-list)
          loop-state
          (recur (rest char-list)
                 (push-item (first char-list) :char loop-state)))))))

(define-registered
  char_frominteger
  ^{:stack-types [:char :integer]}
  (fn [state]
    (if (not (empty? (:integer state)))
      (let [item (stack-ref :integer 0 state)]
        (->> (pop-item :integer state)
             (push-item (char (mod item 128)) :char)))
      state)))

(define-registered
  char_fromfloat
  ^{:stack-types [:char :float]}
  (fn [state]
    (if (not (empty? (:float state)))
      (let [item (stack-ref :float 0 state)]
        (->> (pop-item :float state)
             (push-item (char (mod (long item) 128)) :char)))
      state)))

(define-registered
  char_isletter
  ^{:stack-types [:char :boolean]}
  (fn [state]
    (if (not (empty? (:char state)))
      (let [item (stack-ref :char 0 state)]
        (->> (pop-item :char state)
             (push-item (Character/isLetter item)
                        :boolean)))
      state)))

(define-registered
  char_isdigit
  ^{:stack-types [:char :boolean]}
  (fn [state]
    (if (not (empty? (:char state)))
      (let [item (stack-ref :char 0 state)]
        (->> (pop-item :char state)
             (push-item (Character/isDigit item)
                        :boolean)))
      state)))

(define-registered
  char_iswhitespace
  ^{:stack-types [:char :boolean]}
  (fn [state]
    (if (not (empty? (:char state)))
      (let [item (stack-ref :char 0 state)]
        (->> (pop-item :char state)
             (push-item (or (= item \newline)
                            (= item \space)
                            (= item \tab))
                        :boolean)))
      state)))

(define-registered
  char_uppercase
  ^{:stack-types [:char]}
  (fn [state]
    (if (not (empty? (:char state)))
      (let [cha (stack-ref :char 0 state)]
        (->> (pop-item :char state)
             (push-item
              (if (and (>= (int cha) 97) (<= (int cha) 122))
                (char (- (int cha) 32))
                cha)
              :char)))
      state)))

(define-registered
  char_lowercase
  ^{:stack-types [:char]}
  (fn [state]
    (if (not (empty? (:char state)))
      (let [cha (stack-ref :char 0 state)]
        (->> (pop-item :char state)
             (push-item
              (if (and (>= (int cha) 65) (<= (int cha) 90))
                (char (+ (int cha) 32))
                cha)
              :char)))
      state)))
 */
