//! Internal errors from the `inkling` processing engine.

use std::{error::Error, fmt};

use crate::{
    error::runtime::InklingError,
    follow::ChoiceInfo,
    knot::{Address, AddressKind},
    node::Stack,
};

impl Error for InternalError {}

#[derive(Clone, Debug)]
/// Internal errors from `inkling`.
///
/// These are errors which arise when the library produces objects, trees, text
/// or internal stacks that are inconsistent with each other or themselves.
///
/// If the library is well written these should not possibly occur; at least until
/// this point every part of the internals are fully deterministic. That obviously
/// goes for a lot of buggy code that has been written since forever, so nothing
/// unique there.
///
/// Either way, all those sorts of errors are encapsulated here. They should never
/// be caused by invalid user input or Ink files, those errors should be captured
/// by either the parent [`InklingError`][crate::error::InklingError]
/// or parsing [`ReadError`][crate::error::ReadError] error structures.
pub enum InternalError {
    /// The internal stack of knots is inconsistent or has not been set properly.
    BadKnotStack(StackError),
    /// Could not `Process` a line of text into its final form.
    CouldNotProcess(ProcessError),
    /// Selected branch index does not exist.
    IncorrectChoiceIndex {
        /// Selection index.
        selection: usize,
        /// Available choices at the branching point.
        available_choices: Vec<ChoiceInfo>,
        /// Index in `stack` where the error occurred.
        stack_index: usize,
        /// Stack of choices from the root node to the branching point.
        stack: Stack,
    },
    /// Current stack is not properly representing the graph or has some indexing problems.
    IncorrectNodeStack(IncorrectNodeStackError),
    /// Tried to use a variable address as a location.
    UseOfVariableAsLocation { name: String },
    /// Tried to use an unvalidated address after the story was parsed.
    UseOfUnvalidatedAddress { address: Address },
}

impl_from_error![
    InternalError;
    [BadKnotStack, StackError],
    [CouldNotProcess, ProcessError],
    [IncorrectNodeStack, IncorrectNodeStackError]
];

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use IncorrectNodeStackError::*;
        use InternalError::*;
        use ProcessErrorKind::*;
        use StackError::*;

        match self {
            BadKnotStack(err) => match err {
                BadAddress {
                    address: Address::Validated(AddressKind::Location { knot, stitch }),
                } => write!(
                    f,
                    "The currently set knot address (knot: {}, stitch: {}) does not \
                     actually represent a knot in the story",
                    knot, stitch
                ),
                BadAddress { address } => write!(
                    f,
                    "Tried to used a non-validated or non-location `Address` ('{:?}') in \
                     a function",
                    address
                ),
                NoLastChoices => write!(
                    f,
                    "Tried to follow with a choice but the last set of presented choices has \
                     not been saved"
                ),
                NoRootKnot { knot_name } => write!(
                    f,
                    "After reading a set of knots, the root knot with name {} \
                     does not exist in the set",
                    knot_name
                ),
                NoStack => write!(
                    f,
                    "There is no currently set knot or address to follow the story from"
                ),
            },
            CouldNotProcess(ProcessError { kind }) => match kind {
                InvalidAlternativeIndex => write!(
                    f,
                    "When processing an alternative, an invalid index was used to pick an item"
                ),
                InklingError(err) => write!(f, "{}", err),
            },
            IncorrectChoiceIndex {
                selection,
                ref available_choices,
                stack_index,
                ref stack,
            } => write!(
                f,
                "Tried to resume after a choice was made but the chosen index does not exist \
                 in the set of choices. Somehow a faulty set of choices was created from this \
                 branch point and returned upwards, the stack is wrong, or the wrong set of \
                 choices was used elsewhere in the preparation of the choice list. \
                 Selection index: {}, number of branches: {} \
                 (node level: {}, stack: {:?})",
                selection,
                available_choices.len(),
                stack_index,
                stack
            ),
            IncorrectNodeStack(err) => match err {
                EmptyStack => write!(f, "Tried to advance through a knot with an empty stack"),
                ExpectedBranchingPoint { stack_index, stack } => {
                    let item_number = stack[*stack_index];

                    write!(
                        f,
                        "While resuming a follow the stack found a regular line where \
                         it expected a branch point to nest deeper into. \
                         The stack has been corrupted. \
                         (stack level: {}, item number: {}, stack: {:?}",
                        stack_index, item_number, stack
                    )
                }
                MissingBranchIndex { stack_index, stack } => write!(
                    f,
                    "While resuming a follow the stack did not contain an index to \
                         select a branch with from a set of choices. The stack has been \
                         corrupted.
                         (stack level: {}, attempted index: {}, stack: {:?}",
                    stack_index,
                    stack_index + 1,
                    stack
                ),
                OutOfBounds {
                    stack_index,
                    stack,
                    num_items,
                } => write!(
                    f,
                    "Current stack has invalid index {} at node level {}: size of set is {} \
                     (stack: {:?})",
                    stack[*stack_index], stack_index, num_items, stack
                ),
            },
            UseOfVariableAsLocation { name } => write!(
                f,
                "Tried to use variable '{}' as a location in the story",
                name
            ),
            UseOfUnvalidatedAddress { address } => {
                write!(f, "Tried to use unvalidated address '{:?}'", address)
            }
        }
    }
}

#[derive(Clone, Debug)]
/// Error from processing content into its final format.
pub struct ProcessError {
    /// Error variant.
    pub kind: ProcessErrorKind,
}

impl From<InklingError> for ProcessError {
    fn from(err: InklingError) -> Self {
        ProcessError {
            kind: ProcessErrorKind::InklingError(Box::new(err)),
        }
    }
}

#[derive(Clone, Debug)]
/// Variant of `ProcessError`.
pub enum ProcessErrorKind {
    /// An `Alternative` sequence tried to access an item with an out-of-bounds index.
    InvalidAlternativeIndex,
    /// An `InklingError` encountered during processing.
    InklingError(Box<InklingError>),
}

#[derive(Clone, Debug)]
/// Errors related to the stack of `Knots`, `Stitches` and choices set to
/// the [`Story`][crate::story::Story].
pub enum StackError {
    /// The current stack of `Address`es is empty and a follow was requested.
    NoStack,
    /// An invalid address was used inside the system.
    ///
    /// This means that some bad assumptions have been made somewhere. Addresses are
    /// always supposed to be verified as valid before use.
    BadAddress { address: Address },
    /// No set of presented choices have been added to the system.
    NoLastChoices,
    /// No root knot was added to the stack when the `Story` was constructed.
    NoRootKnot { knot_name: String },
}

#[derive(Clone, Debug)]
/// Current node tree [`Stack`][crate::node::Stack] is incorrect.
pub enum IncorrectNodeStackError {
    /// Tried to follow through nodes with an empty stack.
    EmptyStack,
    /// Found a `Line` object where a set of branching choices should be.
    ExpectedBranchingPoint { stack_index: usize, stack: Stack },
    /// Tried to follow a branch but stack does not have an index for the follow,
    /// it is too short.
    MissingBranchIndex { stack_index: usize, stack: Stack },
    /// Stack contains an invalid index for the current node level.
    OutOfBounds {
        stack_index: usize,
        stack: Stack,
        num_items: usize,
    },
}
