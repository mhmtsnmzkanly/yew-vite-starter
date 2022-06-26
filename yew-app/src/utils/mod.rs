//pub mod ;
pub mod logger;

/* Kodlar */

// String veriyi, &str dönüştürür.
pub fn to_str(string_val: String) -> &'static str {
    Box::leak(string_val.into_boxed_str())
}

// &str veriyi, String dönüştürür.
pub fn to_string(str_val: &str ) -> String {
    String::from(str_val)
}

pub fn vec_to_string(vec_str: Vec<&str>) -> String {
	vec_str.into_iter().collect()
}