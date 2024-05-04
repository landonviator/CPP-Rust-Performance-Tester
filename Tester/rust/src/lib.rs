
#[no_mangle]
pub extern "C" fn landon_clip(input: f32, drive: f32) -> f32
{
    let input_drive: f32 = input * drive;
    return 2.0 / 3.14 * input_drive.atan();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
