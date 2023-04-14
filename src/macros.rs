
// pub(crate) use mando_param;
#[macro_export]
macro_rules! mando {
    () => {
        println!("Well, that didn't work at all, did it now?")
    };
    ($function_parm:expr; $param_vec:expr) => {
        {
            
        let mut mando_vector: Vec<Mando2<fn(std::vec::Vec<MandoParam>, &mut World, u128, u128)>> = Vec::new();
        let mando: Mando2<fn(std::vec::Vec<MandoParam>, &mut World, u128, u128)> = Mando2 { mando_func: $function_parm, params: $param_vec };
        mando_vector.push(mando);
        mando_vector
    }

    };
    // ($function_parm:expr; $( $x:expr ),*) => {
    //     {
    //     let mut mando_vector: Vec<Mando2> = Vec::new();
    //     let mut mando_parm_vec: Vec<MandoParam> = Vec::new();
    //     $(
    //         mando_parm_vec.push(mando_param!($x));
    //     )*
    //     let mando = Mando2 { mandoFunc: $function_parm, params: mando_parm_vec };
    //     // mando_vector.push(mando);
    //     mando_vector
    // }

    // };
    // ($function_parm:expr; $( $a:expr ),*; $( $b:expr ),*) => {
    //     {
    //     let mut mando_vector: Vec<Mando2> = Vec::new();
    //     let mut mando_parm_vec: Vec<MandoParam> = Vec::new();
    //     // let varm = vec![];
    //     $(mando_parm_vec.push(mando_param!($a));)*
    //     $(mando_parm_vec.push(mando_param!($b));)*
        
        

    //     let mando = Mando2 { mandoFunc: $function_parm, params: mando_parm_vec };
    //     // mando_vector.push(mando);
    //     mando_vector
    // }
    // };
    // ($function_parm:expr; $( $a:expr ),*; $( $b:expr ),*; $( $c:expr ),*) => {
    //     {
    //     let mut mando_vector: Vec<Mando2> = Vec::new();
    //     let mut mando_parm_vec: Vec<MandoParam> = Vec::new();
    //     // let varm = vec![];
    //     $(mando_parm_vec.push(mando_param!($a));)*
    //     $(mando_parm_vec.push(mando_param!($b));)*
    //     $(mando_parm_vec.push(mando_param!($c));)*
        

    //     let mando = Mando2 { mandoFunc: $function_parm, mandoParams: mando_parm_vec };
    //     // mando_vector.push(mando);
    //     mando_vector
    // }
    // };
    // ($function_parm:expr; $( $a:expr ),*; $( $b:expr ),*; $( $c:expr ),*; $( $d:expr ),*) => {
    //     {
    //     let mut mando_vector: Vec<Mando2> = Vec::new();
    //     let mut mando_parm_vec: Vec<MandoParam> = Vec::new();
    //     // let varm = vec![];
    //     $(mando_parm_vec.push(mando_param!($a));)*
    //     $(mando_parm_vec.push(mando_param!($b));)*
    //     $(mando_parm_vec.push(mando_param!($c));)*
    //     $(mando_parm_vec.push(mando_param!($d));)*
        

    //     let mando = Mando2 { mandoFunc: $function_parm, mandoParams: mando_parm_vec };
    //     // mando_vector.push(mando);
    //     mando_vector
    // }
    // };
    // ($function_parm:expr; $( $a:expr ),*; $( $b:expr ),*; $( $c:expr ),*; $( $d:expr ),*; $( $e:expr ),*) => {
    //     {
    //     let mut mando_vector: Vec<Mando2> = Vec::new();
    //     let mut mando_parm_vec: Vec<MandoParam> = Vec::new();
    //     // let varm = vec![];
    //     $(mando_parm_vec.push(mando_param!($a));)*
    //     $(mando_parm_vec.push(mando_param!($b));)*
    //     $(mando_parm_vec.push(mando_param!($c));)*
    //     $(mando_parm_vec.push(mando_param!($d));)*
    //     $(mando_parm_vec.push(mando_param!($e));)*
        

    //     let mando = Mando2 { mandoFunc: $function_parm, mandoParams: mando_parm_vec };
    //     // mando_vector.push(mando);
    //     mando_vector
    // }
    // };
    // ($function_parm:expr; $( $a:expr ),*; $( $b:expr ),*; $( $c:expr ),*; $( $d:expr ),*; $( $e:expr ),*; $( $f:expr ),*) => {
    //     {
    //     let mut mando_vector: Vec<Mando2> = Vec::new();
    //     let mut mando_parm_vec: Vec<MandoParam> = Vec::new();
    //     // let varm = vec![];
    //     $(mando_parm_vec.push(mando_param!($a));)*
    //     $(mando_parm_vec.push(mando_param!($b));)*
    //     $(mando_parm_vec.push(mando_param!($c));)*
    //     $(mando_parm_vec.push(mando_param!($d));)*
    //     $(mando_parm_vec.push(mando_param!($e));)*
    //     $(mando_parm_vec.push(mando_param!($f));)*

        

    //     let mando = Mando2 { mandoFunc: $function_parm, mandoParams: mando_parm_vec };
    //     // mando_vector.push(mando);
    //     mando_vector
    // }
    // };
    // ($function_parm:expr; $( $a:expr ),*; $( $b:expr ),*; $( $c:expr ),*; $( $d:expr ),*; $( $e:expr ),*; $( $f:expr ),*; $( $g:expr ),*) => {
    //     {
    //     let mut mando_vector: Vec<Mando2> = Vec::new();
    //     let mut mando_parm_vec: Vec<MandoParam> = Vec::new();
    //     // let varm = vec![];
    //     $(mando_parm_vec.push(mando_param!($a));)*
    //     $(mando_parm_vec.push(mando_param!($b));)*
    //     $(mando_parm_vec.push(mando_param!($c));)*
    //     $(mando_parm_vec.push(mando_param!($d));)*
    //     $(mando_parm_vec.push(mando_param!($e));)*
    //     $(mando_parm_vec.push(mando_param!($f));)*
    //     $(mando_parm_vec.push(mando_param!($g));)*


    //     let mando = Mando2 { mandoFunc: $function_parm, mandoParams: mando_parm_vec };
    //     // mando_vector.push(mando);
    //     mando_vector
    // }
    // };
    // ($function_parm:expr; $( $a:expr ),*; $( $b:expr ),*; $( $c:expr ),*; $( $d:expr ),*; $( $e:expr ),*; $( $f:expr ),*; $( $g:expr ),*; $( $h:expr ),*) => {
    //     {
    //     let mut mando_vector: Vec<Mando2> = Vec::new();
    //     let mut mando_parm_vec: Vec<MandoParam> = Vec::new();
    //     // let varm = vec![];
    //     $(mando_parm_vec.push(mando_param!($a));)*
    //     $(mando_parm_vec.push(mando_param!($b));)*
    //     $(mando_parm_vec.push(mando_param!($c));)*
    //     $(mando_parm_vec.push(mando_param!($d));)*
    //     $(mando_parm_vec.push(mando_param!($e));)*
    //     $(mando_parm_vec.push(mando_param!($f));)*
    //     $(mando_parm_vec.push(mando_param!($g));)*
    //     $(mando_parm_vec.push(mando_param!($h));)*

    //     let mando = Mando2 { mandoFunc: $function_parm, mandoParams: mando_parm_vec };
    //     // mando_vector.push(mando);
    //     mando_vector
    // }
    // };
}
#[macro_export]
macro_rules! mando_param {
    ( $a:expr ) => {
        {
            let mut mando_param = MandoParam::String("Well, this is embarrassing".to_owned());
                match $a.type_name() {
                    "f32" => {
                        println!("Added a f32");
                        let val = $a as f32;
                        mando_param = MandoParam::Float(val);
                    },
                    "u32" => {
                        println!("Added a u32");
                        let val = $a as u32;
                        mando_param = MandoParam::Int(val);
                    },
                    "i32" => {
                        println!("Added a u32");
                        let val = $a as u32;
                        mando_param = MandoParam::Int(val);
                    },
                    "IVec2" => {
                        println!("Added a IVec2");
                        // let val: IVec2 = $a;
                        // mando_param = MandoParam::IVec2(val);
                    },
                    "Vec2" => {
                        println!("Added a Vec2");
                        // let val = $a as Vec2;
                        // mando_param = MandoParam::Vector2(val);
                    },
                    "Vec3" => {
                        println!("Added a Vec3");
                        // let val = $a as Vec3;
                        // mando_param = MandoParam::Vector3(val);
                    },
                    "String" => {
                        println!("Added a String");
                        // let val = $a as String;
                        // mando_param = MandoParam::String(val);
                    },
                    "Entity" => {
                        println!("Added a Entity");
                        // let val = $a as Entity;
                        // mando_param = MandoParam::BevyEntity(val);
                    },
                    "Vec<Entity>" => {
                        println!("Added a Vec<Entity>");
                        // let val = $a as Vec<Entity>;
                        // mando_param = MandoParam::BevyEntities(val);
                    },
                    _ => {
                        println!("incompatible type {} found. please use a float, i32, or bool.", $a.type_name());
                    }
                }
            mando_param
        }
    };
}

// mod helper_macros {
// #[macro_export]
// macro_rules! mando_param {
//     ( $x:expr ) => {
//         {
//             let mut mando_param = MandoParam::Int(1);
        
//                 match $a.type_name() {
//                     "f32" => {
//                         println!("Added a float");
//                         let val: f32 = $x;
//                         mando_param = MandoParam::Float(val);
//                     },
//                     "i32" => {
//                         println!("Added a float");
//                         let val: f32 = $x;
//                         mando_param = MandoParam::Float(val);
//                     },
//                     _ => {
//                         println!("incompatible type {} found. please use a float, i32, or bool.", $x.type_name());
//                     }
//                 }
//             mando_param
//         }
//     };
// }
// }