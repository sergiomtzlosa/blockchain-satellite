
pub trait TypeInfo {
    fn type_name() -> String;
    fn type_of(&self) -> String;
}

#[macro_export]
macro_rules! impl_type_info {
    ($($name:ident$(<$($T:ident),+>)*),*) => {
        $(impl_type_info_single!($name$(<$($T),*>)*);)*
    };
}

#[macro_export]
macro_rules! mut_if {
    ($name:ident = $value:expr, $($any:expr)+) => (let mut $name = $value;);
    ($name:ident = $value:expr,) => (let $name = $value;);
}

#[macro_export]
macro_rules! impl_type_info_single {
    ($name:ident$(<$($T:ident),+>)*) => {
        impl$(<$($T: TypeInfo),*>)* TypeInfo for $name$(<$($T),*>)* {
            fn type_name() -> String {
                mut_if!(res = String::from(stringify!($name)), $($($T)*)*);
                $(
                    res.push('<');
                    $(
                        res.push_str(&$T::type_name());
                        res.push(',');
                    )*
                    res.pop();
                    res.push('>');
                )*
                res
            }
            fn type_of(&self) -> String {
                $name$(::<$($T),*>)*::type_name()
            }
        }
    }
}

impl<'a, T: TypeInfo + ?Sized> TypeInfo for &'a T {
    fn type_name() -> String {
        let mut res = String::from("&");
        res.push_str(&T::type_name());
        res
    }
    fn type_of(&self) -> String {
        <&T>::type_name()
    }
}

impl<'a, T: TypeInfo + ?Sized> TypeInfo for &'a mut T {
    fn type_name() -> String {
        let mut res = String::from("&mut ");
        res.push_str(&T::type_name());
        res
    }
    fn type_of(&self) -> String {
        <&mut T>::type_name()
    }
}

impl_type_info!(i32, i64, f32, f64, str, String, Vec<T>, Result<T,S>);

#[macro_export]
macro_rules! type_of {
    ($x:expr) => { (&$x).type_of() };
}
