extern crate rustc_serialize;

#[macro_export]
macro_rules! json {
    ([$($val:tt),*]) => {{
    	use rustc_serialize::json::{Json, Array};
        let mut array = Array::new();
        $( array.push(json!($val)); )*
        Json::Array(array)
    }};
    ({ $($key:expr => $val:tt),* }) => {{
    	use rustc_serialize::json::{Json, Object};
        let mut object = Object::new();
        $( object.insert($key.to_owned(), json!($val)); )*
        Json::Object(object)
    }};
    ($val:expr) => {{
        use rustc_serialize::json::ToJson;
        $val.to_json()
    }};
}

mod test {

	#[test]
	fn test_json_plain() {
		use rustc_serialize::json::{Json};
		assert_eq!(Json::I64(1), json!(1i64));
		assert_eq!(Json::U64(2), json!(2u64));
		assert_eq!(Json::F64(3.1), json!(3.1f64));
		assert_eq!(Json::String("string".to_string()), json!("string"));
		assert_eq!(Json::Boolean(true), json!(true));
		assert_eq!(Json::Null, json!(Json::Null));
	}

	#[test]
	fn test_json_array() {
		use rustc_serialize::json::{Json, Array};
		let mut array = Array::new();
		array.push(Json::I64(1));
		array.push(Json::I64(2));
		array.push(Json::I64(3));
		array.push(Json::I64(4));
		array.push(Json::I64(5));
		assert_eq!(Json::Array(array), json!([1i64,2,3,4,5]));
	}

	#[test]
	fn test_json_object() {
		use rustc_serialize::json::{Json, Object};
		let mut object = Object::new();
		object.insert("one".to_string(), Json::F64(3.1));
		let mut inner = Object::new();
		inner.insert("sub".to_string(), Json::String("string".to_string()));
		object.insert("two".to_string(), Json::Object(inner));
		assert_eq!(Json::Object(object), json!({
			"one" => 3.1f64,
			"two" => (json!({
				"sub" => "string"
			}))
		}));
	}
}
