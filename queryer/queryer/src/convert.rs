use std::sync::Arc;

use anyhow::anyhow;
use polars::{
    lazy::dsl::{col as PolarsCol, Expr as PolarsExpr, Operator as PolarsOperator},
    prelude::{LiteralValue as PolarsLiteralValue, SortOptions as PolarsSortOptions},
};
use sqlparser::ast::{
    BinaryOperator as SqlBinaryOperator, Expr as SqlExpr, Offset as SqlOffset,
    OrderByExpr as SqlOrderByExpr, Select as SqlSelect, SelectItem as SqlSelectItem,
    SetExpr as SqlSetExpr, Statement as SqlStatement, TableFactor as SqlTableFactor,
    TableWithJoins as SqlTableWithJoins, Value as SqlValue,
};

pub struct Sql<'a> {
    pub selection: Vec<PolarsExpr>,
    pub condition: Option<PolarsExpr>,
    pub source: &'a str,
    pub order_by: Vec<(String, PolarsSortOptions)>,
    pub offset: Option<i64>,
    pub limit: Option<u32>,
}

pub struct Source<'a>(&'a [SqlTableWithJoins]);
pub struct Expression(Box<SqlExpr>);
pub struct Operation(SqlBinaryOperator);
pub struct Value(SqlValue);
pub struct Projection<'a>(&'a SqlSelectItem);
pub struct Order<'a>(&'a SqlOrderByExpr);
pub struct Offset<'a>(&'a SqlOffset);
pub struct Limit<'a>(&'a SqlExpr);

impl<'a> TryFrom<&'a SqlStatement> for Sql<'a> {
    type Error = anyhow::Error;

    fn try_from(sql: &'a SqlStatement) -> Result<Self, Self::Error> {
        match sql {
            SqlStatement::Query(q) => {
                let offset = q.offset.as_ref();
                let limit = q.limit.as_ref();
                let orders = &q.order_by;
                let SqlSelect {
                    from: table_with_joins,
                    selection: where_clause,
                    projection,

                    group_by: _,
                    ..
                } = match &q.body.as_ref() {
                    SqlSetExpr::Select(statement) => statement.as_ref(),
                    _ => return Err(anyhow!("We only support Select Query at the moment")),
                };

                let source = Source(table_with_joins).try_into()?;

                let condition = match where_clause {
                    Some(expr) => Some(Expression(Box::new(expr.to_owned())).try_into()?),
                    None => None,
                };

                let mut selection = Vec::with_capacity(8);
                for p in projection {
                    let expr = Projection(p).try_into()?;
                    selection.push(expr);
                }

                let mut order_by = Vec::new();
                for expr in orders {
                    order_by.push(Order(expr).try_into()?);
                }

                let offset = offset.map(|v| Offset(v).into());
                let limit = limit.map(|v| Limit(v).into());

                Ok(Sql {
                    selection,
                    condition,
                    source,
                    order_by,
                    offset,
                    limit,
                })
            }
            _ => Err(anyhow!("We only support Query at the moment")),
        }
    }
}

impl<'a> TryFrom<Source<'a>> for &'a str {
    type Error = anyhow::Error;

    fn try_from(source: Source<'a>) -> Result<Self, Self::Error> {
        println!("{:?}", source.0);
        if source.0.len() != 1 {
            return Err(anyhow!("We only support single data source at the moment"));
        }

        let table = &source.0[0];
        if !table.joins.is_empty() {
            return Err(anyhow!("We do not support joint data source at the moment"));
        }

        match &table.relation {
            // SqlTableFactor::Table { name, .. } => Ok(&name.0.first().unwrap().value),
            SqlTableFactor::Table { name, .. } => Ok(&name.0.first().unwrap().value),
            _ => Err(anyhow!("We only support table")),
        }
    }
}

impl TryFrom<Expression> for PolarsExpr {
    type Error = anyhow::Error;

    fn try_from(expr: Expression) -> Result<Self, Self::Error> {
        match *expr.0 {
            SqlExpr::BinaryOp { left, op, right } => Ok(PolarsExpr::BinaryExpr {
                left: Box::new(Expression(left).try_into()?),
                op: Operation(op).try_into()?,
                right: Box::new(Expression(right).try_into()?),
            }),
            // SqlExpr::Wildcard => Ok(Self::Wildcard),
            // SqlExpr::IsNull(expr) => Ok(Self::IsNull(Box::new(Expression(expr).try_into()?))),
            // SqlExpr::IsNotNull(expr) => Ok(Self::IsNotNull(Box::new(Expression(expr).try_into()?))),
            SqlExpr::Identifier(id) => Ok(Self::Column(Arc::from(id.value.as_str()))),
            SqlExpr::Value(v) => Ok(Self::Literal(Value(v).try_into()?)),
            v => Err(anyhow!("expr {:#?} is not supported", v)),
        }
    }
}

impl TryFrom<Operation> for PolarsOperator {
    type Error = anyhow::Error;

    fn try_from(op: Operation) -> Result<Self, Self::Error> {
        match op.0 {
            SqlBinaryOperator::Plus => Ok(Self::Plus),
            SqlBinaryOperator::Minus => Ok(Self::Minus),
            SqlBinaryOperator::Multiply => Ok(Self::Multiply),
            SqlBinaryOperator::Divide => Ok(Self::Divide),
            SqlBinaryOperator::Modulo => Ok(Self::Modulus),
            SqlBinaryOperator::Gt => Ok(Self::Gt),
            SqlBinaryOperator::Lt => Ok(Self::Lt),
            SqlBinaryOperator::GtEq => Ok(Self::GtEq),
            SqlBinaryOperator::LtEq => Ok(Self::LtEq),
            SqlBinaryOperator::Eq => Ok(Self::Eq),
            SqlBinaryOperator::NotEq => Ok(Self::NotEq),
            SqlBinaryOperator::And => Ok(Self::And),
            SqlBinaryOperator::Or => Ok(Self::Or),
            v => Err(anyhow!("Operator {} is not supported", v)),
        }
    }
}

impl TryFrom<Value> for PolarsLiteralValue {
    type Error = anyhow::Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v.0 {
            SqlValue::Number(v, _) => Ok(Self::Float64(v.parse().unwrap())),
            SqlValue::Boolean(v) => Ok(Self::Boolean(v)),
            SqlValue::Null => Ok(PolarsLiteralValue::Null),
            v => Err(anyhow!("Value {} is not supported", v)),
        }
    }
}

impl<'a> TryFrom<Projection<'a>> for PolarsExpr {
    type Error = anyhow::Error;

    fn try_from(p: Projection) -> Result<Self, Self::Error> {
        match p.0 {
            SqlSelectItem::UnnamedExpr(SqlExpr::Identifier(id)) => Ok(PolarsCol(&id.to_string())),
            SqlSelectItem::ExprWithAlias {
                expr: SqlExpr::Identifier(id),
                alias,
            } => Ok(PolarsExpr::Alias(
                Box::new(PolarsExpr::Column(Arc::from(id.to_string()))),
                Arc::from(alias.to_string()),
            )),
            SqlSelectItem::QualifiedWildcard(v, ..) => Ok(PolarsCol(&v.to_string())),
            SqlSelectItem::Wildcard(..) => Ok(PolarsCol("*")),
            item => Err(anyhow!("projection {} not supported", item)),
        }
    }
}

impl<'a> TryFrom<Order<'a>> for (String, PolarsSortOptions) {
    type Error = anyhow::Error;

    fn try_from(order: Order) -> Result<Self, Self::Error> {
        let name = match &order.0.expr {
            SqlExpr::Identifier(id) => id.to_string(),
            expr => {
                return Err(anyhow!(
                    "We only support identifier for order by, got {}",
                    expr
                ))
            }
        };
        Ok((
            name,
            PolarsSortOptions {
                descending: !order.0.asc.unwrap_or(true),
                nulls_last: order.0.nulls_first.unwrap_or(false),
                multithreaded: false,
            },
        ))
    }
}

impl<'a> From<Offset<'a>> for i64 {
    fn from(offset: Offset) -> Self {
        match offset.0 {
            SqlOffset {
                value: SqlExpr::Value(SqlValue::Number(v, _b)),
                ..
            } => v.parse().unwrap_or(0),
            _ => 0,
        }
    }
}

impl<'a> From<Limit<'a>> for u32 {
    fn from(l: Limit<'a>) -> Self {
        match l.0 {
            SqlExpr::Value(SqlValue::Number(v, _b)) => v.parse().unwrap_or(u32::MAX),
            _ => u32::MAX,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TyrDialect;
    use sqlparser::parser::Parser;

    #[test]
    fn parse_sql_works() {
        let url = "http://abc.xyz/abc?a=1&b=2";
        let sql = format!(
            "select a, b, c from {} where a=1 order by c desc limit 5 offset 10",
            url
        );
        let statement = &Parser::parse_sql(&TyrDialect::default(), sql.as_ref()).unwrap()[0];
        let sql: Sql = statement.try_into().unwrap();
        assert_eq!(sql.source, url);
        assert_eq!(sql.limit, Some(5));
        assert_eq!(sql.offset, Some(10));
        assert_eq!(sql.order_by, vec![("c".into(), PolarsSortOptions{
            descending: true,
            nulls_last: false,
            multithreaded: false
        })]);
        assert_eq!(
            sql.selection,
            vec![PolarsCol("a"), PolarsCol("b"), PolarsCol("c")]
        );
    }
}
