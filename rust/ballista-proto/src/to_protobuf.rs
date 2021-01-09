// Copyright 2020 Andy Grove
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Serde code to convert Arrow schemas and DataFusion logical plans to Ballista protocol
//! buffer format, allowing DataFusion logical plans to be serialized and transmitted between
//! processes.

use std::convert::TryInto;

use crate::{empty_expr_node, empty_logical_plan_node, protobuf, BallistaProtoError};

use arrow::datatypes::{DataType, Schema};
use datafusion::datasource::parquet::ParquetTable;
use datafusion::datasource::{CsvFile, TableProvider};
use datafusion::logical_plan::{Expr, LogicalPlan};
use datafusion::physical_plan::aggregates::AggregateFunction;
use datafusion::scalar::ScalarValue;

impl TryInto<protobuf::LogicalPlanNode> for &LogicalPlan {
    type Error = BallistaProtoError;

    fn try_into(self) -> Result<protobuf::LogicalPlanNode, Self::Error> {
        match self {
            LogicalPlan::TableScan {
                table_name,
                source,
                projection,
                projected_schema,
                filters,
            } => {
                let schema = source.schema();
                let source = source.as_any();
                let projection = projection.as_ref().map(|column_indices| {
                    let columns: Vec<String> = column_indices
                        .iter()
                        .map(|i| schema.field(*i).name().clone())
                        .collect();
                    protobuf::ProjectionColumns { columns }
                });
                let schema: protobuf::Schema = schema.as_ref().try_into()?;

                let mut node = empty_logical_plan_node();

                if let Some(parquet) = source.downcast_ref::<ParquetTable>() {
                    node.scan = Some(protobuf::ScanNode {
                        path: parquet.path().to_owned(),
                        projection,
                        schema: Some(schema),
                        has_header: false,
                        file_format: "parquet".to_owned(),
                    });
                    Ok(node)
                } else if let Some(csv) = source.downcast_ref::<CsvFile>() {
                    node.scan = Some(protobuf::ScanNode {
                        path: csv.path().to_owned(),
                        projection,
                        schema: Some(schema),
                        has_header: csv.has_header(),
                        file_format: "csv".to_owned(),
                    });
                    Ok(node)
                } else {
                    Err(BallistaProtoError::General(format!(
                        "logical plan to_proto unsupported table provider {:?}",
                        source
                    )))
                }
            }
            LogicalPlan::Projection { expr, input, .. } => {
                let input: protobuf::LogicalPlanNode = input.as_ref().try_into()?;
                let mut node = empty_logical_plan_node();
                node.input = Some(Box::new(input));
                node.projection = Some(protobuf::ProjectionNode {
                    expr: expr
                        .iter()
                        .map(|expr| expr.try_into())
                        .collect::<Result<Vec<_>, BallistaProtoError>>()?,
                });
                Ok(node)
            }
            LogicalPlan::Filter { predicate, input } => {
                let input: protobuf::LogicalPlanNode = input.as_ref().try_into()?;
                let mut node = empty_logical_plan_node();
                node.input = Some(Box::new(input));
                node.selection = Some(protobuf::SelectionNode {
                    expr: Some(predicate.try_into()?),
                });
                Ok(node)
            }
            LogicalPlan::Aggregate {
                input,
                group_expr,
                aggr_expr,
                ..
            } => {
                let input: protobuf::LogicalPlanNode = input.as_ref().try_into()?;
                let mut node = empty_logical_plan_node();
                node.input = Some(Box::new(input));
                node.aggregate = Some(protobuf::AggregateNode {
                    group_expr: group_expr
                        .iter()
                        .map(|expr| expr.try_into())
                        .collect::<Result<Vec<_>, BallistaProtoError>>()?,
                    aggr_expr: aggr_expr
                        .iter()
                        .map(|expr| expr.try_into())
                        .collect::<Result<Vec<_>, BallistaProtoError>>()?,
                });
                Ok(node)
            }
            _ => Err(BallistaProtoError::General(format!(
                "logical plan to_proto {:?}",
                self
            ))),
        }
    }
}

impl TryInto<protobuf::LogicalExprNode> for &Expr {
    type Error = BallistaProtoError;

    fn try_into(self) -> Result<protobuf::LogicalExprNode, Self::Error> {
        match self {
            Expr::Column(name) => {
                let mut expr = empty_expr_node();
                expr.has_column_name = true;
                expr.column_name = name.clone();
                Ok(expr)
            }
            Expr::Alias(expr, alias) => {
                let mut expr_node = empty_expr_node();
                expr_node.alias = Some(Box::new(protobuf::AliasNode {
                    expr: Some(Box::new(expr.as_ref().try_into()?)),
                    alias: alias.to_owned(),
                }));
                Ok(expr_node)
            }
            Expr::Literal(value) => match value {
                ScalarValue::Utf8(s) => {
                    let mut expr = empty_expr_node();
                    expr.has_literal_string = true;
                    expr.literal_string = s.as_ref().unwrap().to_owned(); //TODO remove unwrap
                    Ok(expr)
                }
                ScalarValue::Int8(n) => {
                    let mut expr = empty_expr_node();
                    expr.has_literal_i8 = true;
                    expr.literal_int = n.unwrap() as i64; // TODO remove unwrap
                    Ok(expr)
                }
                ScalarValue::Int16(n) => {
                    let mut expr = empty_expr_node();
                    expr.has_literal_i16 = true;
                    expr.literal_int = n.unwrap() as i64; // TODO remove unwrap
                    Ok(expr)
                }
                ScalarValue::Int32(n) => {
                    let mut expr = empty_expr_node();
                    expr.has_literal_i32 = true;
                    expr.literal_int = n.unwrap() as i64; // TODO remove unwrap
                    Ok(expr)
                }
                ScalarValue::Int64(n) => {
                    let mut expr = empty_expr_node();
                    expr.has_literal_i64 = true;
                    expr.literal_int = n.unwrap() as i64; // TODO remove unwrap
                    Ok(expr)
                }
                ScalarValue::UInt8(n) => {
                    let mut expr = empty_expr_node();
                    expr.has_literal_u8 = true;
                    expr.literal_uint = n.unwrap() as u64; // TODO remove unwrap
                    Ok(expr)
                }
                ScalarValue::UInt16(n) => {
                    let mut expr = empty_expr_node();
                    expr.has_literal_u16 = true;
                    expr.literal_uint = n.unwrap() as u64; // TODO remove unwrap
                    Ok(expr)
                }
                ScalarValue::UInt32(n) => {
                    let mut expr = empty_expr_node();
                    expr.has_literal_u32 = true;
                    expr.literal_uint = n.unwrap() as u64; // TODO remove unwrap
                    Ok(expr)
                }
                ScalarValue::UInt64(n) => {
                    let mut expr = empty_expr_node();
                    expr.has_literal_u64 = true;
                    expr.literal_uint = n.unwrap() as u64; // TODO remove unwrap
                    Ok(expr)
                }
                ScalarValue::Float32(n) => {
                    let mut expr = empty_expr_node();
                    expr.has_literal_f32 = true;
                    expr.literal_f32 = n.unwrap() as f32; // TODO remove unwrap
                    Ok(expr)
                }
                ScalarValue::Float64(n) => {
                    let mut expr = empty_expr_node();
                    expr.has_literal_f64 = true;
                    expr.literal_f64 = n.unwrap() as f64; // TODO remove unwrap
                    Ok(expr)
                }
                other => Err(BallistaProtoError::General(format!(
                    "to_proto unsupported scalar value {:?}",
                    other
                ))),
            },
            Expr::BinaryExpr { left, op, right } => {
                let mut expr = empty_expr_node();
                expr.binary_expr = Some(Box::new(protobuf::BinaryExprNode {
                    l: Some(Box::new(left.as_ref().try_into()?)),
                    r: Some(Box::new(right.as_ref().try_into()?)),
                    op: format!("{:?}", op),
                }));
                Ok(expr)
            }
            Expr::AggregateFunction {
                ref fun, ref args, ..
            } => {
                let mut expr = empty_expr_node();

                let aggr_function = match fun {
                    AggregateFunction::Min => Ok(protobuf::AggregateFunction::Min),
                    AggregateFunction::Max => Ok(protobuf::AggregateFunction::Max),
                    AggregateFunction::Sum => Ok(protobuf::AggregateFunction::Sum),
                    AggregateFunction::Avg => Ok(protobuf::AggregateFunction::Avg),
                    AggregateFunction::Count => Ok(protobuf::AggregateFunction::Count),
                }?;

                let arg = &args[0];
                expr.aggregate_expr = Some(Box::new(protobuf::AggregateExprNode {
                    aggr_function: aggr_function.into(),
                    expr: Some(Box::new(arg.try_into()?)),
                }));
                Ok(expr)
            }
            _ => Err(BallistaProtoError::General(format!(
                "logical expr to_proto {:?}",
                self
            ))),
        }
    }
}

impl TryInto<protobuf::Schema> for &Schema {
    type Error = BallistaProtoError;

    fn try_into(self) -> Result<protobuf::Schema, Self::Error> {
        Ok(protobuf::Schema {
            columns: self
                .fields()
                .iter()
                .map(|field| {
                    let proto = to_proto_arrow_type(&field.data_type());
                    proto.and_then(|arrow_type| {
                        Ok(protobuf::Field {
                            name: field.name().to_owned(),
                            arrow_type: arrow_type.into(),
                            nullable: field.is_nullable(),
                            children: vec![],
                        })
                    })
                })
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

fn to_proto_arrow_type(dt: &DataType) -> Result<protobuf::ArrowType, BallistaProtoError> {
    match dt {
        DataType::Int8 => Ok(protobuf::ArrowType::Int8),
        DataType::Int16 => Ok(protobuf::ArrowType::Int16),
        DataType::Int32 => Ok(protobuf::ArrowType::Int32),
        DataType::Int64 => Ok(protobuf::ArrowType::Int64),
        DataType::UInt8 => Ok(protobuf::ArrowType::Uint8),
        DataType::UInt16 => Ok(protobuf::ArrowType::Uint16),
        DataType::UInt32 => Ok(protobuf::ArrowType::Uint32),
        DataType::UInt64 => Ok(protobuf::ArrowType::Uint64),
        DataType::Float32 => Ok(protobuf::ArrowType::Float),
        DataType::Float64 => Ok(protobuf::ArrowType::Double),
        DataType::Utf8 => Ok(protobuf::ArrowType::Utf8),
        other => Err(BallistaProtoError::General(format!(
            "Unsupported data type {:?}",
            other
        ))),
    }
}