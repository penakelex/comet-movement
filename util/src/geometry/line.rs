use num_traits::Num;

pub fn scale_line_segment<T: Num>(line_segment: T, scale: T) -> T {
    line_segment / scale
}