use csscolorparser::{Color, ParseColorError};

// Allow conversion from Result<Color, ParseColorError> to Result<Color, String>

pub trait ConvertParseColorError {
    fn convert(self) -> Result<Color, String>;
}

impl ConvertParseColorError for Result<Color, ParseColorError> {
    fn convert(self) -> Result<Color, String> {
        fn stringify(x: ParseColorError) -> String {
            match x {
                ParseColorError::InvalidHex => "Invalid HEX",
                ParseColorError::InvalidRgb => "Invalid RGB",
                ParseColorError::InvalidHsl => "Invalid HSL",
                ParseColorError::InvalidHwb => "Invalid HWB",
                ParseColorError::InvalidHsv => "Invalid HSV",
                ParseColorError::InvalidFunction => "Invalid function",
                ParseColorError::InvalidUnknown => "Invalid unknown",
            }
            .to_string()
        }

        self.map_err(stringify)
    }
}

// Macro helpers

/// Error macro
macro_rules! msgerror {
    ($v:tt) => {
        MessageResult::Err($v.into())
    };
}

/// Log macro
macro_rules! log {
    ($v:expr, $body:expr) => {
        {
            use std::io::Write;
    
            info!("{}... ", $v);
            let _ = std::io::stdout().flush();
    
            let start = std::time::Instant::now();
            let _res = $body;
            let elapsed = start.elapsed();
            
            info!("Done! [{:?}]", elapsed);
            
            _res
        }
    };
}

pub(crate) use msgerror;
pub(crate) use log;
