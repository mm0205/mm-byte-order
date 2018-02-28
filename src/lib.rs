//! `mm_byte_order` provides the features that converts byte order.

/// provides the features that converts byte order.
pub trait Converter {
    type OutputType;

    /// Converts the byte order.
    ///
    /// # Examples
    ///
    /// Converts the integer byte order.
    ///
    /// ```
    /// use mm_byte_order::Converter;
    ///
    /// let src_u16 = 0x1234_u16;
    /// let dest_u16 = src_u16.convert_byte_order();
    ///
    /// assert_eq!(0x3412, dest_u16);
    /// assert_eq!(src_u16, dest_u16.convert_byte_order());
    ///
    /// let src_i16 = 0xFFFE_u16 as i16;
    /// let dest_i16 = src_i16.convert_byte_order();
    ///
    /// assert_eq!(-257, dest_i16);
    /// assert_eq!(0xFEFF_u16 as i16, dest_i16);
    /// assert_eq!(src_i16, dest_i16.convert_byte_order());
    /// assert_eq!(0xFFFE_u16 as i16, dest_i16.convert_byte_order());
    ///
    /// let src_u32 = 0x12345678_u32;
    /// let dest_u32 = src_u32.convert_byte_order();
    ///
    /// assert_eq!(0x78563412, dest_u32);
    /// assert_eq!(src_u32, dest_u32.convert_byte_order());
    ///
    /// let src_i32 = 0xFF123456_u32 as i32;
    /// let dest_i32 = src_i32.convert_byte_order();
    ///
    /// assert_eq!(1446253311, dest_i32);
    /// assert_eq!(0x563412FF, dest_i32);
    /// assert_eq!(-15584170, dest_i32.convert_byte_order());
    /// assert_eq!(0xFF123456_u32 as i32, dest_i32.convert_byte_order());
    ///
    /// let src_u64 = 0x1234567812345678_u64;
    /// let dest_u64 = src_u64.convert_byte_order();
    ///
    /// assert_eq!(0x7856341278563412, dest_u64);
    /// assert_eq!(src_u64, dest_u64.convert_byte_order());
    ///
    /// let src_i64 = 0xFF345678123456FF_u64 as i64;
    /// let dest_i64 = src_i64.convert_byte_order();
    ///
    /// assert_eq!(-47793492107840257, dest_i64);
    /// assert_eq!(0xFF563412785634FF_u64 as i64, dest_i64);
    /// assert_eq!(-57325821547489537, dest_i64.convert_byte_order());
    /// assert_eq!(0xFF345678123456FF_u64 as i64, dest_i64.convert_byte_order());
    ///
    /// ```
    ///
    fn convert_byte_order(&self) -> Self::OutputType;
}

impl Converter for u8 {
    type OutputType = u8;

    fn convert_byte_order(&self) -> Self::OutputType {
        self.swap_bytes()
    }
}

impl Converter for i8 {
    type OutputType = i8;
    fn convert_byte_order(&self) -> Self::OutputType {
        self.swap_bytes()
    }
}

impl Converter for u16 {
    type OutputType = u16;
    fn convert_byte_order(&self) -> Self::OutputType {
        self.swap_bytes()
    }
}

impl Converter for i16 {
    type OutputType = i16;
    fn convert_byte_order(&self) -> Self::OutputType {
        self.swap_bytes()
    }
}

impl Converter for u32 {
    type OutputType = u32;
    fn convert_byte_order(&self) -> Self::OutputType {
        self.swap_bytes()
    }
}

impl Converter for i32 {
    type OutputType = i32;
    fn convert_byte_order(&self) -> Self::OutputType {
        self.swap_bytes()
    }
}

impl Converter for u64 {
    type OutputType = u64;
    fn convert_byte_order(&self) -> Self::OutputType {
        self.swap_bytes()
    }
}

impl Converter for i64 {
    type OutputType = i64;
    fn convert_byte_order(&self) -> Self::OutputType {
        self.swap_bytes()
    }
}

impl<'a> Converter for &'a [u8] {
    type OutputType = Vec<u8>;
    fn convert_byte_order(&self) -> Self::OutputType {
        let mut result = vec![0; self.len()];
        let max_index = result.len() - 1;
        for (i, x) in self.iter().enumerate() {
            result[max_index - i] = *x;
        }
        result
    }
}