
use serde_json::{Map, Value, json};

pub fn get_example_data() -> Map<String, Value>
{
    let example_json = json!({ "name"   : "John Smith",
                                    "sku"    : "20223",
                                    "price"  : 23.95,
                                    "shipTo" : { "name" : "Jane Smith",
                                                "address" : "123 Maple Street",
                                                "city" : "Pretendville",
                                                "state" : "NY",
                                                "zip"   : "12345" },
                                    "billTo" : { "name" : "John Smith",
                                                "address" : "123 Maple Street",
                                                "city" : "Pretendville",
                                                "state" : "NY",
                                                "zip"   : "12345" }
                                    });
   let x = example_json.as_object();
   let h = x.cloned();
   h.unwrap()
}