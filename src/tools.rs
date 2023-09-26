#[macro_export]
macro_rules! cast {
        ($target: expr, $pat: path) => {
            {
                if let $pat(a) = $target { // #1
                    Ok(a)
                } else {
                    Err(anyhow::anyhow!(
                        "mismatch variant when cast to {} from {:?}", 
                        stringify!($pat), $target)) // #2
                }
            }
        };
    }