
#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Bin {
        left: Box<Self>,
        op: BinOP,
        right: Box<Self>
    },
    Number(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinOP {
    Add,
    Minus
}

#[macro_export]
macro_rules! expr {
    ((add $left:tt $right:tt)) => {
        $crate::Expression::Bin {
            left: Box::new(expr!($left)),
            op: $crate::BinOP::Add,
            right: Box::new(expr!($right)),
        }
    };
    ((minus $left:tt $right:tt)) => {
        $crate::Expression::Bin {
            left: Box::new(expr!($left)),
            op: $crate::BinOP::Minus,
            right: Box::new(expr!($right)),
        }
    };
    ($e:expr) => {
        $crate::Expression::from($e)
    }
}

macro_rules! impl_number {
    ($t: ident) => {
        impl From<$t> for crate::Expression {
            fn from(v: $t) -> Self {
                Expression::Number(v as f64)
            }
        } 
    }
}

macro_rules! impl_numbers {
    ($($t:ident)*) => {
        $(
            impl_number!($t);
        )*
    }
}

impl_numbers!(u8 u16 u32 u64 i8 i16 i32 i64 usize f32 f64);


#[test]
fn expr() {
    expr!((add 1 2));
    expr!((minus 1 2));
    expr!((minus (add 1.0 2) 2));
    expr!(1);
}
