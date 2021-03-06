use super::*;
use crate::{ast, error::Span};

pub fn compile_block<'code>(
    state: &mut IRGenState,
    mut builder: BlockBuilder,
    statements: impl IntoIterator<Item = (ast::Statement<'code>, Span)>,
    bindings: &mut BindingCounter,
    variables: &mut VariableTracker<'code>,
    block_depth: usize,
    source_info: &SourceMetadata,
) -> Result<BlockBuilder, VarE> {
    // clean the variables for now
    variables.variables_at_depth(block_depth).clear();

    for (st, st_span) in statements {
        builder = statement::compile_statement(
            state,
            builder,
            bindings,
            st,
            variables,
            block_depth,
            source_info,
        )
        .map_err(|e| e.with_backup_source(st_span, source_info))?;
    }
    Ok(builder)
}
