use std::{time::Duration, vec};

use option_chain_tool::opt;
use tokio::time::sleep;
use tracing::*;

#[derive(Debug, Clone)]
struct TestStruct {
    value: Option<TestStruct1>,
}

#[derive(Debug, Clone)]
struct VecStruct1 {
    id: i32,
    name: String,
    some_field: Result<String, String>,
}

#[derive(Debug, Clone)]
struct OkResp {
    id: i32,
    field1: Option<String>,
}

#[derive(Debug, Clone)]
struct TestStruct1 {
    value: Option<TestStruct2>,
    my_vec: Option<Vec<VecStruct1>>,
    some_result_field: Result<OkResp, String>,
}
#[derive(Debug, Clone)]
struct TestStruct2 {
    value: Option<i32>,
    required_value: String,
    required_int_value: i32,
}

#[tokio::test]
async fn test_foo() {
    let test_struct = TestStruct {
        value: Some(TestStruct1 {
            some_result_field: Ok(OkResp {
                id: 1,
                field1: Some("Field value".to_string()),
            }),
            value: Some(TestStruct2 {
                value: Some(42),
                required_value: "100".to_string(),
                required_int_value: 100,
            }),
            my_vec: Some(vec![
                VecStruct1 {
                    id: 1,
                    name: "First".to_string(),
                    some_field: Ok("Value 1".to_string()),
                },
                VecStruct1 {
                    id: 2,
                    name: "Second".to_string(),
                    some_field: Err("Error 2".to_string()),
                },
            ]),
        }),
    };
    // Now use the opt_chain macro!
    let a: Option<&TestStruct2> = opt!(test_struct.value?.value?);
    let a: Option<&i32> = opt!(test_struct.value?.value?.value?);

    let a: Option<&i32> = opt!(test_struct.value?.value?.required_int_value);
    let a: Option<&String> = opt!(test_struct.value?.value?.required_value);
    let a: Option<&String> = opt!(test_struct.value?.my_vec?.get(0)?.name);
    // Print the results
    let a = opt!(test_struct.value?.value?);
    let a = opt!(test_struct.value?.some_result_field?Ok.field1?);
    let a = opt!(test_struct.value?.value?.value?);
    let a = opt!(test_struct.value?.value?.required_int_value);
    let a = opt!(test_struct.value?.value?.required_value);
    let a = opt!(test_struct.value?.my_vec?.get(0)?.name);
    let a = opt!(test_struct.value?.my_vec?.get(0)?.some_field?Ok);

    let a = opt!(test_struct.value?.my_vec?.get(0)?);
    println!("Macro result: {:?}", a);
}
