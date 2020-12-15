mod common;
mod input;
mod output;

pub use common::{SeekOrigin, SeekError};
pub use input::Input;
pub use output::Output;

fn calculate_position(position: usize, length: usize, loc: SeekOrigin, offset: i64) -> Result<usize, SeekError> {
    let result = match loc {
        SeekOrigin::Begin => {
            let new_position = if offset >= 0 {
                offset as usize
            } else {
                return Err(SeekError::BeforeBegin)
            };

            if new_position >= length {
                return Err(SeekError::AfterEnd)
            }

            new_position
        },
        SeekOrigin::Current => {
            let new_position = if offset.is_positive() {
                position + offset as usize
            } else {
                let u_offset = offset.wrapping_abs() as usize;
                if position < u_offset {
                    return Err(SeekError::BeforeBegin)
                }
    
                position - u_offset
            };

            if new_position >= length {
                return Err(SeekError::BeforeBegin)
            }

            new_position
        },
        SeekOrigin::End => {
            let new_position = if offset.is_positive() {
                return Err(SeekError::AfterEnd)
            } else {
                let u_offset = offset.wrapping_abs() as usize;

                length - u_offset
            };

            new_position
        }
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seek_begin() {
        assert_eq!(calculate_position(0, 3, SeekOrigin::Begin, 1).unwrap(), 1);
    }

    #[test]
    #[should_panic]
    fn seek_before_begin() {
        calculate_position(0, 3, SeekOrigin::Begin, -1).unwrap();
    }

    #[test]
    fn seek_current() {
        assert_eq!(calculate_position(0, 3, SeekOrigin::Current, 1).unwrap(), 1);
        assert_eq!(calculate_position(1, 3, SeekOrigin::Current, 1).unwrap(), 2);
        assert_eq!(calculate_position(2, 3, SeekOrigin::Current, -2).unwrap(), 0);
    }

    #[test]
    fn seek_end() {
        assert_eq!(calculate_position(0, 3, SeekOrigin::End, -1).unwrap(), 2);
    }
}
