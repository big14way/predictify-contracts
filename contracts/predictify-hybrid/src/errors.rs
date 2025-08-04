
use soroban_sdk::contracterror;

/// Essential error enum for the Predictify Hybrid contract

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {

    // Core errors
    Unauthorized = 1,
    MarketClosed = 2,
    MarketNotFound = 3,
    InsufficientStake = 4,
    InvalidOutcome = 5,
    AlreadyClaimed = 6,
    
    // Essential market errors
    MarketAlreadyResolved = 7,
    NothingToClaim = 8,
    AlreadyVoted = 9,
    AlreadyDisputed = 10,
    OracleUnavailable = 11,
    
    // Reentrancy errors  
    ReentrancyAttack = 101,
    InvalidReentrancyState = 102,
    InconsistentReentrancyState = 103,
    InvalidCallState = 104,
    CallStackOverflow = 105,
    
    // Essential validation errors
    InvalidInput = 201,
    InvalidConfig = 202,
    MarketNotResolved = 203,
    InvalidThreshold = 204,
    InvalidState = 205,
    
    // Essential Oracle errors
    OracleDataStale = 301,
    OraclePriceOutOfRange = 302,
    InvalidOracleFeed = 303,
    
    // Essential system errors  
    ConfigurationNotFound = 401,
    NoFeesToCollect = 402,
    FeeAlreadyCollected = 403,
    InternalError = 500,
}

/// Error helper functions for common scenarios
pub mod helpers {
    use super::Error;
    use soroban_sdk::{panic_with_error, String, Env, Vec, Address};

    /// Validate that the caller is the admin
    pub fn require_admin(
        env: &Env,
        caller: &Address,
        admin: &Address,
    ) -> Result<(), Error> {
        if caller != admin {
            panic_with_error!(env, Error::Unauthorized);

        }
    }

    /// Generate detailed error message with context
    pub fn generate_detailed_error_message(error: &Error, context: &ErrorContext) -> String {
        let base_message = error.description();
        let operation = &context.operation;
        
        match error {
            Error::Unauthorized => {
                String::from_str(context.call_chain.env(), "Authorization failed for operation. User may not have required permissions.")
            }
            Error::MarketNotFound => {
                String::from_str(context.call_chain.env(), "Market not found during operation. The market may have been removed or the ID is incorrect.")
            }
            Error::MarketClosed => {
                String::from_str(context.call_chain.env(), "Market is closed and cannot accept new operations. Operation was attempted on a closed market.")
            }
            Error::OracleUnavailable => {
                String::from_str(context.call_chain.env(), "Oracle service is unavailable during operation. External data source may be down or unreachable.")
            }
            Error::InsufficientStake => {
                String::from_str(context.call_chain.env(), "Insufficient stake amount for operation. Please increase your stake to meet the minimum requirement.")
            }
            Error::AlreadyVoted => {
                String::from_str(context.call_chain.env(), "User has already voted in this market. Operation cannot be performed as voting is limited to one vote per user.")
            }
            Error::InvalidInput => {
                String::from_str(context.call_chain.env(), "Invalid input provided for operation. Please check your input parameters and try again.")
            }
            Error::InvalidState => {
                String::from_str(context.call_chain.env(), "Invalid system state for operation. The system may be in an unexpected state.")
            }
            _ => {
                String::from_str(context.call_chain.env(), "Error during operation. Please check the operation parameters and try again.")
            }
        }
    }

    /// Handle error recovery based on error type and context
    pub fn handle_error_recovery(env: &Env, error: &Error, context: &ErrorContext) -> Result<bool, Error> {
        let recovery_strategy = Self::get_error_recovery_strategy(error);
        
        match recovery_strategy {
            RecoveryStrategy::Retry => {
                // For retryable errors, return success to allow retry
                Ok(true)
            }
            RecoveryStrategy::RetryWithDelay => {
                // For errors that need delay, check if enough time has passed
                let last_attempt = context.timestamp;
                let current_time = env.ledger().timestamp();
                let delay_required = 60; // 1 minute delay
                
                if current_time - last_attempt >= delay_required {
                    Ok(true)
                } else {
                    Err(Error::InvalidState)
                }
            }
            RecoveryStrategy::AlternativeMethod => {
                // Try alternative approach based on error type
                match error {
                    Error::OracleUnavailable => {
                        // Try fallback oracle or cached data
                        Ok(true)
                    }
                    Error::MarketNotFound => {
                        // Try to find similar market or suggest alternatives
                        Ok(false)
                    }
                    _ => Ok(false)
                }
            }
            RecoveryStrategy::Skip => {
                // Skip the operation and continue
                Ok(true)
            }
            RecoveryStrategy::Abort => {
                // Abort the operation
                Ok(false)
            }
            RecoveryStrategy::ManualIntervention => {
                // Require manual intervention
                Err(Error::InvalidState)
            }
            RecoveryStrategy::NoRecovery => {
                // No recovery possible
                Ok(false)
            }
        }
    }


    /// Validate that the outcome is valid for the market
    pub fn require_valid_outcome(
        env: &Env,
        outcome: &String,
        outcomes: &Vec<String>,
    ) -> Result<(), Error> {
        if !outcomes.contains(outcome) {
            panic_with_error!(env, Error::InvalidOutcome);
        }
        Ok(())

    }

    /// Log error details for debugging and analysis
    pub fn log_error_details(env: &Env, detailed_error: &DetailedError) {
        // In a real implementation, this would log to a persistent storage
        // For now, we'll just emit the error event
        Self::emit_error_event(env, detailed_error);
    }

    /// Get error recovery strategy based on error type
    pub fn get_error_recovery_strategy(error: &Error) -> RecoveryStrategy {
        match error {
            // Retryable errors
            Error::OracleUnavailable => RecoveryStrategy::RetryWithDelay,
            Error::InvalidInput => RecoveryStrategy::Retry,
            
            // Alternative method errors
            Error::MarketNotFound => RecoveryStrategy::AlternativeMethod,
            Error::ConfigurationNotFound => RecoveryStrategy::AlternativeMethod,
            
            // Skip errors
            Error::AlreadyVoted => RecoveryStrategy::Skip,
            Error::AlreadyClaimed => RecoveryStrategy::Skip,
            Error::FeeAlreadyCollected => RecoveryStrategy::Skip,
            
            // Abort errors
            Error::Unauthorized => RecoveryStrategy::Abort,
            Error::MarketClosed => RecoveryStrategy::Abort,
            Error::MarketAlreadyResolved => RecoveryStrategy::Abort,
            
            // Manual intervention errors
            Error::AdminNotSet => RecoveryStrategy::ManualIntervention,
            Error::DisputeFeeDistributionFailed => RecoveryStrategy::ManualIntervention,
            
            // No recovery errors
            Error::InvalidState => RecoveryStrategy::NoRecovery,
            Error::InvalidOracleConfig => RecoveryStrategy::NoRecovery,
            
            // Default to abort for unknown errors
            _ => RecoveryStrategy::Abort,
        }
    }


    /// Validate oracle configuration
    pub fn require_valid_oracle_config(
        env: &Env,
        config: &crate::OracleConfig,
    ) -> Result<(), Error> {
        if config.threshold <= 0 {
            panic_with_error!(env, Error::InvalidConfig);
        }

        if config.comparison != String::from_str(env, "gt")
            && config.comparison != String::from_str(env, "lt")
            && config.comparison != String::from_str(env, "eq")
        {
            panic_with_error!(env, Error::InvalidConfig);

        }
        
        Ok(())
    }


    /// Validate market creation parameters
    pub fn require_valid_market_params(
        env: &Env,
        question: &String,
        outcomes: &Vec<String>,
        duration_days: u32,
    ) -> Result<(), Error> {
        if question.is_empty() {
            panic_with_error!(env, Error::InvalidInput);
        }

        if outcomes.len() < 2 {
            panic_with_error!(env, Error::InvalidInput);

        }
    }


        if duration_days == 0 || duration_days > 365 {
            panic_with_error!(env, Error::InvalidInput);

        }
    }

    /// Get technical details for debugging
    fn get_technical_details(error: &Error, context: &ErrorContext) -> String {
        let error_code = error.code();
        let error_num = *error as u32;
        let timestamp = context.timestamp;
        
        String::from_str(context.call_chain.env(), "Error details for debugging")
    }
}
