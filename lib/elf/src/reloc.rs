use super::ElfType;

pub struct ElfRel<T>
where
    T: Clone + Copy + ElfType,
{
    r_offset: T,
    r_info: T,
}

pub struct ElfRela<T>
where
    T: Clone + Copy + ElfType,
{
    r_offset: T,
    r_info: T,
    r_addend: T,
}

pub struct ElfDyn<T>
where
    T: Clone + Copy + ElfType,
{
    d_tag: T,
    d_val: T,
}
