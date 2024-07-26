use core::f32;

const PI_OVER_4: f32 = f32::consts::PI / 4.0;

#[inline(always)]
fn fast_atan(x: f32) -> f32 {
    return (PI_OVER_4 * x) - (x * (x.abs() - 1.0) * (0.2447 + 0.663*x.abs())); 
}

const A: f32 = 0.0776509570923569;
const B: f32 = -0.287434475393028;
const C: f32 = PI_OVER_4 - A - B;

#[inline(always)]
fn fast_atan_alt(x: f32) -> f32 {
    let squared = x*x;
    //return x - (squared*x/3f32) + squared*squared*x/5f32 - (squared*squared*squared*x/7f32)
    return ((A*squared + B) * squared + C) * x;
}

#[no_mangle]
#[inline(always)]
pub extern "C" fn landon_clip(input: f32, drive: f32) -> f32
{
    let input_drive: f32 = input * drive;
    return 2.0 / 3.14 * input_drive.atan();
}

#[no_mangle]
#[inline(always)]
pub extern "C" fn fast_atan_less_accurate(input: f32, drive: f32) -> f32
{
    let input_drive: f32 = input * drive;
    return 2.0 / 3.14 * fast_atan(input_drive);
}

#[no_mangle]
#[inline(always)]
pub extern "C" fn fast_atan_more_accurate(input: f32, drive: f32) -> f32
{
    let input_drive: f32 = input * drive;
    return 2.0 / 3.14 * fast_atan_alt(input_drive);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_atan() {
        let result = fast_atan(0.2);
        assert_eq!(result, (0.2f32).atan());
    }

    #[test]
    fn test_fast_atan_alt() {
        let result = fast_atan_alt(0.2);
        assert_eq!(result, (0.2f32).atan());
    }
}
