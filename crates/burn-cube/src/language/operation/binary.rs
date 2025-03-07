use crate::language::operation::base::binary_expand;
use crate::language::{CubeContext, ExpandElement, UInt, BF16, F16, F32, F64, I32, I64};
use crate::unexpanded;

pub mod add {

    use crate::dialect::Operator;

    use super::*;

    pub fn expand(
        context: &mut CubeContext,
        lhs: ExpandElement,
        rhs: ExpandElement,
    ) -> ExpandElement {
        binary_expand(context, lhs, rhs, Operator::Add)
    }

    macro_rules! impl_add {
        ($type:ty) => {
            impl core::ops::Add for $type {
                type Output = Self;

                fn add(self, _rhs: Self) -> Self::Output {
                    unexpanded!()
                }
            }
        };
    }

    impl_add!(F16);
    impl_add!(BF16);
    impl_add!(F32);
    impl_add!(F64);
    impl_add!(I32);
    impl_add!(I64);
    impl_add!(UInt);
}

pub mod sub {
    use crate::dialect::Operator;

    use super::*;

    pub fn expand(
        context: &mut CubeContext,
        lhs: ExpandElement,
        rhs: ExpandElement,
    ) -> ExpandElement {
        binary_expand(context, lhs, rhs, Operator::Sub)
    }

    macro_rules! impl_sub {
        ($type:ty) => {
            impl core::ops::Sub for $type {
                type Output = Self;

                fn sub(self, _rhs: Self) -> Self::Output {
                    unexpanded!()
                }
            }
        };
    }

    impl_sub!(F16);
    impl_sub!(BF16);
    impl_sub!(F32);
    impl_sub!(F64);
    impl_sub!(I32);
    impl_sub!(I64);
    impl_sub!(UInt);
}

pub mod mul {
    use crate::dialect::Operator;

    use super::*;

    pub fn expand(
        context: &mut CubeContext,
        lhs: ExpandElement,
        rhs: ExpandElement,
    ) -> ExpandElement {
        binary_expand(context, lhs, rhs, Operator::Mul)
    }

    macro_rules! impl_mul {
        ($type:ty) => {
            impl core::ops::Mul for $type {
                type Output = Self;

                fn mul(self, _rhs: Self) -> Self::Output {
                    unexpanded!()
                }
            }
        };
    }

    impl_mul!(F16);
    impl_mul!(BF16);
    impl_mul!(F32);
    impl_mul!(F64);
    impl_mul!(I32);
    impl_mul!(I64);
    impl_mul!(UInt);
}

pub mod div {
    use crate::dialect::Operator;

    use super::*;

    pub fn expand(
        context: &mut CubeContext,
        lhs: ExpandElement,
        rhs: ExpandElement,
    ) -> ExpandElement {
        binary_expand(context, lhs, rhs, Operator::Div)
    }

    macro_rules! impl_div {
        ($type:ty) => {
            impl core::ops::Div for $type {
                type Output = Self;

                fn div(self, _rhs: Self) -> Self::Output {
                    unexpanded!()
                }
            }
        };
    }

    impl_div!(F16);
    impl_div!(BF16);
    impl_div!(F32);
    impl_div!(F64);
    impl_div!(I32);
    impl_div!(I64);
    impl_div!(UInt);
}

pub mod rem {
    use crate::dialect::Operator;

    use super::*;

    pub fn expand(
        context: &mut CubeContext,
        lhs: ExpandElement,
        rhs: ExpandElement,
    ) -> ExpandElement {
        binary_expand(context, lhs, rhs, Operator::Modulo)
    }

    macro_rules! impl_rem {
        ($type:ty) => {
            impl core::ops::Rem for $type {
                type Output = Self;

                fn rem(self, _rhs: Self) -> Self::Output {
                    unexpanded!()
                }
            }
        };
    }

    impl_rem!(I32);
    impl_rem!(I64);
    impl_rem!(UInt);
}

pub mod and {
    use crate::dialect::Operator;

    use super::*;

    pub fn expand(
        context: &mut CubeContext,
        lhs: ExpandElement,
        rhs: ExpandElement,
    ) -> ExpandElement {
        binary_expand(context, lhs, rhs, Operator::And)
    }
}

pub mod or {
    use crate::dialect::Operator;

    use super::*;

    pub fn expand(
        context: &mut CubeContext,
        lhs: ExpandElement,
        rhs: ExpandElement,
    ) -> ExpandElement {
        binary_expand(context, lhs, rhs, Operator::Or)
    }
}
