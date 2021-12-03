// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

//! Defines physical expressions that can evaluated at runtime during query execution

use std::any::Any;
use std::sync::Arc;

use crate::error::Result;
use crate::physical_plan::{Accumulator, AccumulatorFly, AggregateExpr, PhysicalExpr};
use crate::scalar::ScalarValue;
use arrow::compute;
use arrow::datatypes::DataType;
use arrow::{
    array::{ArrayRef, UInt64Array},
    datatypes::Field,
};
use chrono::prelude::*;

use super::format_state_name;

/// COUNT aggregate expression
/// Returns the amount of non-null values of the given expression.
#[derive(Debug)]
pub struct CountFly {
    name: String,
    data_type: DataType,
    nullable: bool,
    expr: Arc<dyn PhysicalExpr>,
}

impl CountFly {
    /// Create a new COUNT aggregate function.
    pub fn new(
        expr: Arc<dyn PhysicalExpr>,
        name: impl Into<String>,
        data_type: DataType,
    ) -> Self {
        Self {
            name: name.into(),
            expr,
            data_type,
            nullable: true,
        }
    }
}

impl AggregateExpr for CountFly {
    /// Return a reference to Any that can be used for downcasting
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn field(&self) -> Result<Field> {
        Ok(Field::new(
            &self.name,
            self.data_type.clone(),
            self.nullable,
        ))
    }

    fn state_fields(&self) -> Result<Vec<Field>> {
        Ok(vec![Field::new(
            &format_state_name(&self.name, "count"),
            self.data_type.clone(),
            true,
        )])
    }

    fn expressions(&self) -> Vec<Arc<dyn PhysicalExpr>> {
        vec![self.expr.clone()]
    }

    fn create_accumulator(&self) -> Result<Box<dyn Accumulator>> {
        Ok(Box::new(CountAccumulatorFly::new()))
    }

    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug)]
pub struct CountAccumulatorFly {
    count: Vec<u64>,
}

impl CountAccumulatorFly {
    /// new count accumulator
    pub fn new() -> Self {
        Self { count: vec![] }
    }
}

impl Accumulator for CountAccumulatorFly {
    fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()> {
        let array = &values[0];
        self.count[0] += (array.len() - array.data().null_count()) as u64;
        Ok(())
    }

    fn update(&mut self, values: &[ScalarValue]) -> Result<()> {
        let value = &values[0];
        if !value.is_null() {
            self.count[0] += 1;
        }
        Ok(())
    }

    fn merge(&mut self, states: &[ScalarValue]) -> Result<()> {
        let count = &states[0];
        if let ScalarValue::UInt64(Some(delta)) = count {
            self.count[0] += *delta;
        } else {
            unreachable!()
        }
        Ok(())
    }

    fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()> {
        let counts = states[0].as_any().downcast_ref::<UInt64Array>().unwrap();
        let delta = &compute::sum(counts);
        if let Some(d) = delta {
            self.count[0] += *d;
        }
        Ok(())
    }

    fn state(&self) -> Result<Vec<ScalarValue>> {
        Ok(vec![ScalarValue::UInt64(Some(self.count[0]))])
    }

    fn evaluate(&self) -> Result<ScalarValue> {
        Ok(ScalarValue::UInt64(Some(self.count[0])))
    }
}


impl AccumulatorFly for CountAccumulatorFly {
    fn init_state(&mut self, index: usize) {
        assert_eq!(self.count.len(), index);
        self.count.push(0);
    }
    fn update_batch(&mut self, index: usize, values: &[ArrayRef]) -> Result<()> {
        let array = &values[0];
        self.count[index] += (array.len() - array.data().null_count()) as u64;
        Ok(())
    }

    fn update(&mut self, index: usize, values: &[ScalarValue]) -> Result<()> {
        let value = &values[0];
        if !value.is_null() {
            self.count[index] += 1;
        }
        Ok(())
    }

    fn merge(&mut self, index: usize, states: &[ScalarValue]) -> Result<()> {
        let count = &states[0];
        if let ScalarValue::UInt64(Some(delta)) = count {
            self.count[index] += *delta;
        } else {
            unreachable!()
        }
        Ok(())
    }

    fn merge_batch(&mut self, index: usize, states: &[ArrayRef]) -> Result<()> {
        let counts = states[0].as_any().downcast_ref::<UInt64Array>().unwrap();
        let delta = &compute::sum(counts);
        if let Some(d) = delta {
            self.count[index] += *d;
        }
        Ok(())
    }

    fn state(&self, index: usize) -> Result<Vec<ScalarValue>> {
        Ok(vec![ScalarValue::UInt64(Some(self.count[index]))])
    }

    fn evaluate(&self, index: usize) -> Result<ScalarValue> {
        Ok(ScalarValue::UInt64(Some(self.count[index])))
    }

    fn evaluate_all(&self) -> Result<ArrayRef> {
        let dt = Local::now();
        let result = ScalarValue::iter_to_array(
            self.count.iter().map(|x| {
                ScalarValue::UInt64(Some(*x))
            }),
        );
        println!(
            "evaluate_all usage millis: {}",
            Local::now().timestamp_millis() - dt.timestamp_millis()
        );

        result
    }

    fn state_all(&self) -> Result<Vec<Vec<ScalarValue>>> {
        let dt = Local::now();
        let result = Ok(vec![self.count.iter().map(|x| {
            ScalarValue::UInt64(Some(*x))
        }).collect()]);
        println!(
            "state_all usage millis: {}",
            Local::now().timestamp_millis() - dt.timestamp_millis()
        );
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physical_plan::expressions::col;
    use crate::physical_plan::expressions::tests::aggregate;
    use crate::{error::Result, generic_test_op};
    use arrow::record_batch::RecordBatch;
    use arrow::{array::*, datatypes::*};

    #[test]
    fn count_elements() -> Result<()> {
        let a: ArrayRef = Arc::new(Int32Array::from(vec![1, 2, 3, 4, 5]));
        generic_test_op!(
            a,
            DataType::Int32,
            CountFly,
            ScalarValue::from(5u64),
            DataType::UInt64
        )
    }

    #[test]
    fn count_with_nulls() -> Result<()> {
        let a: ArrayRef = Arc::new(Int32Array::from(vec![
            Some(1),
            Some(2),
            None,
            None,
            Some(3),
            None,
        ]));
        generic_test_op!(
            a,
            DataType::Int32,
            CountFly,
            ScalarValue::from(3u64),
            DataType::UInt64
        )
    }

    #[test]
    fn count_all_nulls() -> Result<()> {
        let a: ArrayRef = Arc::new(BooleanArray::from(vec![
            None, None, None, None, None, None, None, None,
        ]));
        generic_test_op!(
            a,
            DataType::Boolean,
            CountFly,
            ScalarValue::from(0u64),
            DataType::UInt64
        )
    }

    #[test]
    fn count_empty() -> Result<()> {
        let a: Vec<bool> = vec![];
        let a: ArrayRef = Arc::new(BooleanArray::from(a));
        generic_test_op!(
            a,
            DataType::Boolean,
            CountFly,
            ScalarValue::from(0u64),
            DataType::UInt64
        )
    }

    #[test]
    fn count_utf8() -> Result<()> {
        let a: ArrayRef =
            Arc::new(StringArray::from(vec!["a", "bb", "ccc", "dddd", "ad"]));
        generic_test_op!(
            a,
            DataType::Utf8,
            CountFly,
            ScalarValue::from(5u64),
            DataType::UInt64
        )
    }

    #[test]
    fn count_large_utf8() -> Result<()> {
        let a: ArrayRef =
            Arc::new(LargeStringArray::from(vec!["a", "bb", "ccc", "dddd", "ad"]));
        generic_test_op!(
            a,
            DataType::LargeUtf8,
            CountFly,
            ScalarValue::from(5u64),
            DataType::UInt64
        )
    }
}
