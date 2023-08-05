pub unsafe fn reinterpret_cast_slice<TTo>(slice: &[u8]) -> &[TTo] {
    std::slice::from_raw_parts::<TTo>(slice.as_ptr() as *const _, slice.len() / std::mem::size_of::<TTo>())
}

pub fn cast_slice_to_type<TFrom, TTo>(slice: &[TFrom]) -> Vec<TTo> where TFrom : TryInto<TTo> + Copy {
    let vec = slice.iter()
        .map(|elem| {
            if let Ok(elem) = (*elem).try_into() {
                elem
            } else {
                panic!("conversion ddi not work also please write this function to use result smh")
            }
        })
        .collect();

    vec
}