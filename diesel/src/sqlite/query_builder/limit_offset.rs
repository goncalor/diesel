use query_builder::limit_clause::{LimitClause, NoLimitClause};
use query_builder::limit_offset_clause::{BoxedLimitOffsetClause, LimitOffsetClause};
use query_builder::offset_clause::{NoOffsetClause, OffsetClause};
use query_builder::{AstPass, QueryFragment};
use result::QueryResult;
use sqlite::Sqlite;

impl QueryFragment<Sqlite> for LimitOffsetClause<NoLimitClause, NoOffsetClause> {
    fn walk_ast(&self, _out: AstPass<Sqlite>) -> QueryResult<()> {
        Ok(())
    }
}

impl<L> QueryFragment<Sqlite> for LimitOffsetClause<LimitClause<L>, NoOffsetClause>
where
    LimitClause<L>: QueryFragment<Sqlite>,
{
    fn walk_ast(&self, out: AstPass<Sqlite>) -> QueryResult<()> {
        self.limit_clause.walk_ast(out)?;
        Ok(())
    }
}

impl<O> QueryFragment<Sqlite> for LimitOffsetClause<NoLimitClause, OffsetClause<O>>
where
    OffsetClause<O>: QueryFragment<Sqlite>,
{
    fn walk_ast(&self, mut out: AstPass<Sqlite>) -> QueryResult<()> {
        // Sqlite requires a limit clause in front of any offset clause
        // using `LIMIT -1` is the same as not having any limit clause
        // https://sqlite.org/lang_select.html
        out.push_sql(" LIMIT -1 ");
        self.offset_clause.walk_ast(out)?;
        Ok(())
    }
}

impl<L, O> QueryFragment<Sqlite> for LimitOffsetClause<LimitClause<L>, OffsetClause<O>>
where
    LimitClause<L>: QueryFragment<Sqlite>,
    OffsetClause<O>: QueryFragment<Sqlite>,
{
    fn walk_ast(&self, mut out: AstPass<Sqlite>) -> QueryResult<()> {
        self.limit_clause.walk_ast(out.reborrow())?;
        self.offset_clause.walk_ast(out.reborrow())?;
        Ok(())
    }
}

impl<'a> QueryFragment<Sqlite> for BoxedLimitOffsetClause<'a, Sqlite> {
    fn walk_ast(&self, mut out: AstPass<Sqlite>) -> QueryResult<()> {
        match (self.limit.as_ref(), self.offset.as_ref()) {
            (Some(limit), Some(offset)) => {
                limit.walk_ast(out.reborrow())?;
                offset.walk_ast(out.reborrow())?;
            }
            (Some(limit), None) => {
                limit.walk_ast(out.reborrow())?;
            }
            (None, Some(offset)) => {
                // See the `QueryFragment` implementation for `LimitOffsetClause` for details.
                out.push_sql(" LIMIT -1 ");
                offset.walk_ast(out.reborrow())?;
            }
            (None, None) => {}
        }
        Ok(())
    }
}

impl<'a> From<LimitOffsetClause<NoLimitClause, NoOffsetClause>>
    for BoxedLimitOffsetClause<'a, Sqlite>
{
    fn from(_limit_offset: LimitOffsetClause<NoLimitClause, NoOffsetClause>) -> Self {
        Self {
            limit: None,
            offset: None,
        }
    }
}

impl<'a, L> From<LimitOffsetClause<LimitClause<L>, NoOffsetClause>>
    for BoxedLimitOffsetClause<'a, Sqlite>
where
    L: QueryFragment<Sqlite> + 'a,
{
    fn from(limit_offset: LimitOffsetClause<LimitClause<L>, NoOffsetClause>) -> Self {
        Self {
            limit: Some(Box::new(limit_offset.limit_clause)),
            offset: None,
        }
    }
}

impl<'a, O> From<LimitOffsetClause<NoLimitClause, OffsetClause<O>>>
    for BoxedLimitOffsetClause<'a, Sqlite>
where
    O: QueryFragment<Sqlite> + 'a,
{
    fn from(limit_offset: LimitOffsetClause<NoLimitClause, OffsetClause<O>>) -> Self {
        Self {
            limit: None,
            offset: Some(Box::new(limit_offset.offset_clause)),
        }
    }
}

impl<'a, L, O> From<LimitOffsetClause<LimitClause<L>, OffsetClause<O>>>
    for BoxedLimitOffsetClause<'a, Sqlite>
where
    L: QueryFragment<Sqlite> + 'a,
    O: QueryFragment<Sqlite> + 'a,
{
    fn from(limit_offset: LimitOffsetClause<LimitClause<L>, OffsetClause<O>>) -> Self {
        Self {
            limit: Some(Box::new(limit_offset.limit_clause)),
            offset: Some(Box::new(limit_offset.offset_clause)),
        }
    }
}
