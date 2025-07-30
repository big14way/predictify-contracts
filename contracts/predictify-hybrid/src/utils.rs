extern crate alloc;

use soroban_sdk::{Address, Env, Map, String, Symbol, Vec};
use alloc::string::ToString;

use crate::errors::Error;

/// Comprehensive utility function system for Predictify Hybrid contract
///
/// This module provides a centralized collection of utility functions with:
/// - Time and date manipulation utilities
/// - String manipulation and formatting utilities
/// - Numeric calculation helpers
/// - Validation utility functions
/// - Conversion utility functions
/// - Testing utility functions
/// - Common helper functions for contract operations
// ===== TIME AND DATE UTILITIES =====
///   Time and date utility functions
pub struct TimeUtils;

impl TimeUtils {
    /// Convert days to seconds
    pub fn days_to_seconds(days: u32) -> u64 {
        days as u64 * 24 * 60 * 60
    }

    /// Convert hours to seconds
    pub fn hours_to_seconds(hours: u32) -> u64 {
        hours as u64 * 60 * 60
    }

    /// Convert minutes to seconds
    pub fn minutes_to_seconds(minutes: u32) -> u64 {
        minutes as u64 * 60
    }

    /// Calculate time difference between two timestamps
    pub fn time_difference(timestamp1: u64, timestamp2: u64) -> u64 {
        timestamp1.abs_diff(timestamp2)
    }

    /// Check if a timestamp is in the future
    pub fn is_future_timestamp(current_time: u64, future_time: u64) -> bool {
        future_time > current_time
    }

    /// Check if a timestamp is in the past
    pub fn is_past_timestamp(current_time: u64, past_time: u64) -> bool {
        past_time < current_time
    }

    /// Format duration in human-readable format
    pub fn format_duration(seconds: u64) -> String {
        let days = seconds / (24 * 60 * 60);
        let hours = (seconds % (24 * 60 * 60)) / (60 * 60);
        let minutes = (seconds % (60 * 60)) / 60;
        let env = Env::default();
        let mut s = alloc::string::String::new();
        if days > 0 {
            s.push_str(&days.to_string());
            s.push_str("d ");
            s.push_str(&hours.to_string());
            s.push_str("h ");
            s.push_str(&minutes.to_string());
            s.push('m');
        } else if hours > 0 {
            s.push_str(&hours.to_string());
            s.push_str("h ");
            s.push_str(&minutes.to_string());
            s.push('m');
        } else {
            s.push_str(&minutes.to_string());
            s.push('m');
        }
        String::from_str(&env, &s)
    }

    /// Calculate time until deadline
    pub fn time_until_deadline(current_time: u64, deadline: u64) -> u64 {
        deadline.saturating_sub(current_time)
    }

    /// Check if deadline has passed
    pub fn is_deadline_passed(current_time: u64, deadline: u64) -> bool {
        current_time >= deadline
    }
}

// ===== STRING UTILITIES =====

/// String manipulation and formatting utilities
pub struct StringUtils;

impl StringUtils {
    /// Convert string to uppercase
    pub fn to_uppercase(s: &String) -> String {
        // For now, return the original string (proper conversion is complex in Soroban)
        s.clone()
    }

    /// Convert string to lowercase
    pub fn to_lowercase(s: &String) -> String {
        // For now, return the original string (proper conversion is complex in Soroban)
        s.clone()
    }

    /// Trim whitespace from string
    pub fn trim(s: &String) -> String {
        // For now, return the original string (proper trimming is complex in Soroban)
        s.clone()
    }

    /// Truncate string to specified length
    pub fn truncate(s: &String, max_length: u32) -> String {
        let env = Env::default();
        // Simple truncation by checking length (proper character manipulation is complex in Soroban)
        if s.len() <= max_length {
            s.clone()
        } else {
            // Return empty string for now since proper slicing is complex
            String::from_str(&env, "")
        }
    }

    /// Split string by delimiter
    pub fn split(s: &String, _delimiter: &str) -> Vec<String> {
        let env = Env::default();
        // For now, return a vector with just the original string
        let mut result = Vec::new(&env);
        result.push_back(s.clone());
        result
    }

    /// Join strings with delimiter
    pub fn join(strings: &Vec<String>, _delimiter: &str) -> String {
        let env = Env::default();
        // For now, return the first string or empty if empty
        if !strings.is_empty() {
            strings.get(0).unwrap_or(String::from_str(&env, ""))
        } else {
            String::from_str(&env, "")
        }
    }

    /// Check if string contains substring
    pub fn contains(_s: &String, _substring: &str) -> bool {
        // For now, always return false (complex string matching in Soroban)
        false
    }

    /// Check if string starts with prefix
    pub fn starts_with(_s: &String, _prefix: &str) -> bool {
        // For now, always return false (complex string matching in Soroban)
        false
    }

    /// Check if string ends with suffix
    pub fn ends_with(_s: &String, _suffix: &str) -> bool {
        // For now, always return false (complex string matching in Soroban)
        false
    }

    /// Replace substring in string
    pub fn replace(s: &String, _old: &str, _new: &str) -> String {
        // For now, return the original string (complex replacement in Soroban)
        s.clone()
    }

    /// Validate string length
    pub fn validate_string_length(s: &String, min_length: u32, max_length: u32) -> Result<(), Error> {
        let len = s.len();
        if len < min_length || len > max_length {
            Err(Error::InvalidInput)
        } else {
            Ok(())
        }
    }

    /// Sanitize string (remove special characters)
    pub fn sanitize_string(s: &String) -> String {
        // For now, return the original string (complex sanitization in Soroban)
        s.clone()
    }

    /// Generate random string
    pub fn generate_random_string(env: &Env, length: u32) -> String {
        let mut result = alloc::string::String::new();
        for _ in 0..length {
            let random_char = (env.ledger().timestamp() % 26) as u8 + b'a';
            result.push(random_char as char);
        }
        String::from_str(env, &result)
    }
}

// ===== NUMERIC UTILITIES =====

/// Numeric calculation and manipulation utilities
pub struct NumericUtils;

impl NumericUtils {
    /// Calculate percentage
    pub fn calculate_percentage(percentage: &i128, value: &i128, denominator: &i128) -> i128 {
        (percentage * value) / denominator
    }

    /// Round to nearest multiple
    pub fn round_to_nearest(value: &i128, multiple: &i128) -> i128 {
        ((value + multiple / 2) / multiple) * multiple
    }

    /// Clamp value between min and max
    pub fn clamp(value: &i128, min: &i128, max: &i128) -> i128 {
        if *value < *min {
            *min
        } else if *value > *max {
            *max
        } else {
            *value
        }
    }

    /// Check if value is within range
    pub fn is_within_range(value: &i128, min: &i128, max: &i128) -> bool {
        *value >= *min && *value <= *max
    }

    /// Calculate absolute difference
    pub fn abs_difference(a: &i128, b: &i128) -> i128 {
        if *a > *b {
            *a - *b
        } else {
            *b - *a
        }
    }

    /// Calculate square root (integer approximation)
    pub fn sqrt(value: &i128) -> i128 {
        if *value <= 0 {
            return 0;
        }
        let mut x = *value;
        let mut y = (*value + 1) / 2;
        while y < x {
            x = y;
            y = (x + *value / x) / 2;
        }
        x
    }

    /// Calculate weighted average
    pub fn weighted_average(values: &Vec<i128>, weights: &Vec<i128>) -> i128 {
        if values.is_empty() || weights.is_empty() || values.len() != weights.len() {
            return 0;
        }
        let mut sum = 0;
        let mut weight_sum = 0;
        for i in 0..values.len() {
            let v = values.get_unchecked(i);
            let w = weights.get_unchecked(i);
            sum += v * w;
            weight_sum += w;
        }
        if weight_sum == 0 {
            0
        } else {
            sum / weight_sum
        }
    }

    /// Calculate simple interest
    pub fn simple_interest(principal: &i128, rate: &i128, periods: &i128) -> i128 {
        principal + (principal * rate * periods) / 100
    }

    /// Convert number to string
    pub fn i128_to_string(env: &Env, value: &i128) -> String {
        String::from_str(env, &value.to_string())
    }

    /// Convert string to number
    pub fn string_to_i128(_s: &String) -> i128 {
        // For now, return 0 (complex string parsing in Soroban)
        0
    }
}

// ===== VALIDATION UTILITIES =====

/// Validation utility functions
pub struct ValidationUtils;

impl ValidationUtils {
    /// Validate positive number
    pub fn validate_positive_number(value: &i128) -> bool {
        *value > 0
    }

    /// Validate number range
    pub fn validate_number_range(value: &i128, min: &i128, max: &i128) -> bool {
        *value >= *min && *value <= *max
    }

    /// Validate future timestamp
    pub fn validate_future_timestamp(timestamp: &u64) -> bool {
        let current_time = Env::default().ledger().timestamp();
        *timestamp > current_time
    }

    /// Validate address format
    pub fn validate_address(_address: &Address) -> Result<(), Error> {
        // Address validation is handled by Soroban SDK
        Ok(())
    }

    /// Validate email format (basic)
    pub fn validate_email(_email: &String) -> bool {
        // For now, always return true (complex email validation in Soroban)
        true
    }

    /// Validate URL format (basic)
    pub fn validate_url(_url: &String) -> bool {
        // For now, always return true (complex URL validation in Soroban)
        true
    }
}

// ===== CONVERSION UTILITIES =====

/// Conversion utility functions
pub struct ConversionUtils;

impl ConversionUtils {
    /// Convert address to string
    pub fn address_to_string(env: &Env, _address: &Address) -> String {
        // For now, return a default address string (complex conversion in Soroban)
        String::from_str(env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF")
    }

    /// Convert string to address
    pub fn string_to_address(_env: &Env, s: &String) -> Address {
        // Implementation
        Address::from_string(s)
    }

    /// Convert symbol to string
    pub fn symbol_to_string(env: &Env, _symbol: &Symbol) -> String {
        // For now, return a default string (complex symbol conversion in Soroban)
        String::from_str(env, "symbol")
    }

    /// Convert string to symbol
    pub fn string_to_symbol(env: &Env, _s: &String) -> Symbol {
        // For now, return a default symbol (complex conversion in Soroban)
        Symbol::new(env, "default")
    }

    /// Convert map to string representation
    pub fn map_to_string(env: &Env, _map: &Map<String, String>) -> String {
        // For now, return empty map string (complex map serialization in Soroban)
        String::from_str(env, "{}")
    }

    /// Convert vec to string representation
    pub fn vec_to_string(env: &Env, _vec: &Vec<String>) -> String {
        // For now, return empty array string (complex vec serialization in Soroban)
        String::from_str(env, "[]")
    }

    /// Compare two maps for equality
    pub fn maps_equal(map1: &Map<String, String>, map2: &Map<String, String>) -> bool {
        if map1.len() != map2.len() {
            return false;
        }
        for key in map1.keys() {
            if let Some(value1) = map1.get(key.clone()) {
                if let Some(value2) = map2.get(key) {
                    if value1 != value2 {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    /// Check if map contains key
    pub fn map_contains_key(map: &Map<String, String>, key: &String) -> bool {
        map.get(key.clone()).is_some()
    }
}

// ===== COMMON UTILITIES =====

/// Common helper functions
pub struct CommonUtils;

impl CommonUtils {
    /// Generate unique ID
    pub fn generate_unique_id(env: &Env, _prefix: &String) -> String {
        let timestamp = env.ledger().timestamp();
        let sequence = env.ledger().sequence();
        let mut id = alloc::string::String::new();
        id.push_str("market");
        id.push('_');
        id.push_str(&timestamp.to_string());
        id.push('_');
        id.push_str(&sequence.to_string());
        String::from_str(env, &id)
    }

    /// Compare two addresses for equality
    pub fn addresses_equal(a: &Address, b: &Address) -> bool {
        a == b
    }

    /// Compare two strings ignoring case
    pub fn strings_equal_ignore_case(a: &String, b: &String) -> bool {
        // For now, just compare directly (case-insensitive comparison is complex in Soroban)
        a == b
    }

    /// Calculate weighted average
    pub fn calculate_weighted_average(values: &Vec<i128>, weights: &Vec<i128>) -> i128 {
        NumericUtils::weighted_average(values, weights)
    }

    /// Calculate simple interest
    pub fn calculate_simple_interest(principal: &i128, rate: &i128, periods: &i128) -> i128 {
        NumericUtils::simple_interest(principal, rate, periods)
    }

    /// Format number with commas
    pub fn format_number_with_commas(env: &Env, number: &i128) -> String {
        let mut s = alloc::string::String::new();
        let num_str = number.to_string();
        for (count, c) in num_str.chars().rev().enumerate() {
            if count > 0 && count % 3 == 0 {
                s.insert(0, ',');
            }
            s.insert(0, c);
        }
        String::from_str(env, &s)
    }

    /// Generate random number within range
    pub fn random_number_in_range(env: &Env, min: &i128, max: &i128) -> i128 {
        let seed = env.ledger().timestamp() as i128;
        min + (seed % (max - min + 1))
    }
}

// ===== TESTING UTILITIES =====

/// Testing utility functions
pub struct TestingUtils;

impl TestingUtils {
    /// Create test data
    pub fn create_test_data(env: &Env) -> String {
        String::from_str(env, "test_data")
    }

    /// Validate test data structure
    pub fn validate_test_data_structure<T>(_data: &T) -> Result<(), Error> {
        // Placeholder for test data validation
        Ok(())
    }

    /// Generate test address
    pub fn generate_test_address(env: &Env) -> Address {
        Address::from_string(&String::from_str(env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF"))
    }

    /// Generate test symbol
    pub fn generate_test_symbol(env: &Env) -> Symbol {
        Symbol::new(env, "test_symbol")
    }

    /// Generate test string
    pub fn generate_test_string(env: &Env) -> String {
        String::from_str(env, "test_string")
    }

    /// Generate test number
    pub fn generate_test_number() -> i128 {
        1000
    }

    /// Create test map
    pub fn create_test_map(env: &Env) -> Map<String, String> {
        let mut map = Map::new(env);
        map.set(String::from_str(env, "key1"), String::from_str(env, "value1"));
        map.set(String::from_str(env, "key2"), String::from_str(env, "value2"));
        map
    }

    /// Create test vec
    pub fn create_test_vec(env: &Env) -> Vec<String> {
        let mut vec = Vec::new(env);
        vec.push_back(String::from_str(env, "item1"));
        vec.push_back(String::from_str(env, "item2"));
        vec.push_back(String::from_str(env, "item3"));
        vec
    }
} 