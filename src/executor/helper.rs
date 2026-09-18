
pub fn calc_seconds(time: &str) -> Result<u32, std::num::ParseIntError> {
    let parts: Vec<u32> = time
        .split(':')
        .map(|x| x.parse::<u32>())
        .collect::<Result<Vec<u32>, _>>()?;

    Ok(parts[0] * 3600 + parts[1] * 60 + parts[2])
}