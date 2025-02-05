use super::*;
use shopify_function::{run_function_with_input, Result};

#[test]
fn test_result_contains_no_operations() -> Result<()> {
    let result = run_function_with_input(
        function,
        r#"
          {
            "cart": {
              "lines": [
                {
                  "id": "gid://shopify/CartLine/1"
                }
              ]
            },
            "fulfillmentGroups": [
              {
                "handle":  "1",
                "lines": [
                  {
                    "id": "gid://shopify/CartLine/1"
                  }
                ],
                "deliveryGroup": {
                  "id": "gid://shopify/CartDeliveryGroup/1"
                },
                "locationHandles": ["2578303"]
              }
            ],
            "locations": [
              {
                "handle": "2578303",
                "name": "Main St.",
                "address": {
                  "address1": "123 Main St."
                }
              }
            ],
            "deliveryOptionGenerator": {
              "metafield": null
            }
          }
        "#,
    )?;

    let operations = vec![output::Operation {
        add: output::LocalPickupDeliveryOption {
            fulfillment_group_handles: None,
            title: Some("Main St.".to_string()),
            cost: Some(Decimal(1.99)),
            pickup_location: output::PickupLocation {
                location_handle: "2578303".to_string(),
                pickup_instruction: Some("Usually ready in 24 hours.".to_string()),
            },
        },
    }];

    let expected = crate::output::FunctionResult { operations };

    assert_eq!(result, expected);
    Ok(())
}
