use stdext::function_name;

pub fn _get_tz_str() -> Result<String, Box<dyn std::error::Error>> {
    match std::env::consts::OS {
        "windows" => Err(format!(
            "{}: Not implemented for {} OS",
            function_name!(),
            std::env::consts::OS
        )
        .into()),
        _ => {
            let etc_file = "/etc/localtime";
            let link_prefix = "/usr/share/zoneinfo/";

            let x = match std::fs::read_link(etc_file) {
                Ok(x) => Ok(x.to_str().unwrap_or_default().to_owned()),
                Err(err) => Err(format!(
                    "{}: Unable to get local timezone: {}: {}",
                    function_name!(),
                    etc_file,
                    err
                )),
            }?;

            let x = match x.strip_prefix(link_prefix) {
                Some(x) => Ok(x.to_owned()),
                None => Err(format!(
                    "{}: Unable to get local timezone: not a correct symlink: {} -> {} (prefix must be {})",
                    function_name!(),
                    etc_file,
                    x,
                    link_prefix
                )),
            }?;

            Ok(x)
        }
    }
}
