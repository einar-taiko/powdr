use std::collections::HashMap;

use ast::analyzed::AlgebraicExpression as Expression;
use ast::analyzed::AlgebraicReference;
use ast::parsed::SelectedExpressions;
use number::FieldElement;

use self::block_machine::BlockMachine;
use self::double_sorted_witness_machine::DoubleSortedWitnesses;
pub use self::fixed_lookup_machine::FixedLookup;
use self::profiling::record_end;
use self::profiling::record_start;
use self::sorted_witness_machine::SortedWitnesses;
use self::write_once_memory::WriteOnceMemory;
use ast::analyzed::IdentityKind;

use super::affine_expression::AffineExpression;
use super::generator::Generator;
use super::EvalResult;
use super::FixedData;
use super::MutableState;
use super::QueryCallback;

mod block_machine;
mod double_sorted_witness_machine;
mod fixed_lookup_machine;
pub mod machine_extractor;
pub mod profiling;
mod sorted_witness_machine;
mod write_once_memory;

/// A machine is a set of witness columns and identities where the columns
/// are used on the right-hand-side of lookups. It can process plookups.
pub trait Machine<'a, T: FieldElement>: Send + Sync {
    /// Like `process_plookup`, but also records the time spent in this machine.
    fn process_plookup_timed<'b, Q: QueryCallback<T>>(
        &mut self,
        mutable_state: &'b mut MutableState<'a, 'b, T, Q>,
        kind: IdentityKind,
        left: &[AffineExpression<&'a AlgebraicReference, T>],
        right: &'a SelectedExpressions<Expression<T>>,
    ) -> Option<EvalResult<'a, T>> {
        record_start(self.name());
        let result = self.process_plookup(mutable_state, kind, left, right);
        record_end(self.name());
        result
    }

    /// Returns a unique name for this machine.
    fn name(&self) -> &str;

    /// Process a plookup. Not all values on the LHS need to be available.
    /// Can update internal data.
    /// Only return an error if this machine is able to handle the query and
    /// it results in a constraint failure.
    /// If this is not the right machine for the query, return `None`.
    fn process_plookup<'b, Q: QueryCallback<T>>(
        &mut self,
        mutable_state: &'b mut MutableState<'a, 'b, T, Q>,
        kind: IdentityKind,
        left: &[AffineExpression<&'a AlgebraicReference, T>],
        right: &'a SelectedExpressions<Expression<T>>,
    ) -> Option<EvalResult<'a, T>>;

    /// Returns the final values of the witness columns.
    fn take_witness_col_values<'b, Q: QueryCallback<T>>(
        &mut self,
        fixed_lookup: &'b mut FixedLookup<T>,
        query_callback: &'b mut Q,
    ) -> HashMap<String, Vec<T>>;
}

/// All known implementations of [Machine].
/// This allows us to treat machines uniformly without putting them into a `Box`,
/// which requires that all lifetime parameters are 'static.
pub enum KnownMachine<'a, T: FieldElement> {
    SortedWitnesses(SortedWitnesses<'a, T>),
    DoubleSortedWitnesses(DoubleSortedWitnesses<T>),
    WriteOnceMemory(WriteOnceMemory<'a, T>),
    BlockMachine(BlockMachine<'a, T>),
    Vm(Generator<'a, T>),
}

impl<'a, T: FieldElement> Machine<'a, T> for KnownMachine<'a, T> {
    fn process_plookup<'b, Q: QueryCallback<T>>(
        &mut self,
        mutable_state: &'b mut MutableState<'a, 'b, T, Q>,
        kind: IdentityKind,
        left: &[AffineExpression<&'a AlgebraicReference, T>],
        right: &'a SelectedExpressions<Expression<T>>,
    ) -> Option<EvalResult<'a, T>> {
        match self {
            KnownMachine::SortedWitnesses(m) => m.process_plookup(mutable_state, kind, left, right),
            KnownMachine::DoubleSortedWitnesses(m) => {
                m.process_plookup(mutable_state, kind, left, right)
            }
            KnownMachine::WriteOnceMemory(m) => m.process_plookup(mutable_state, kind, left, right),
            KnownMachine::BlockMachine(m) => m.process_plookup(mutable_state, kind, left, right),
            KnownMachine::Vm(m) => m.process_plookup(mutable_state, kind, left, right),
        }
    }

    fn name(&self) -> &str {
        match self {
            KnownMachine::SortedWitnesses(m) => m.name(),
            KnownMachine::DoubleSortedWitnesses(m) => m.name(),
            KnownMachine::WriteOnceMemory(m) => m.name(),
            KnownMachine::BlockMachine(m) => m.name(),
            KnownMachine::Vm(m) => m.name(),
        }
    }

    fn take_witness_col_values<'b, Q: QueryCallback<T>>(
        &mut self,
        fixed_lookup: &'b mut FixedLookup<T>,
        query_callback: &'b mut Q,
    ) -> HashMap<String, Vec<T>> {
        match self {
            KnownMachine::SortedWitnesses(m) => {
                m.take_witness_col_values(fixed_lookup, query_callback)
            }
            KnownMachine::DoubleSortedWitnesses(m) => {
                m.take_witness_col_values(fixed_lookup, query_callback)
            }
            KnownMachine::WriteOnceMemory(m) => {
                m.take_witness_col_values(fixed_lookup, query_callback)
            }
            KnownMachine::BlockMachine(m) => {
                m.take_witness_col_values(fixed_lookup, query_callback)
            }
            KnownMachine::Vm(m) => m.take_witness_col_values(fixed_lookup, query_callback),
        }
    }
}
