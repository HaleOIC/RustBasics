// pub struct NoParaStatement {
//     command: String,
// }

// pub struct UniStatement {
//     command: String,
//     arg: Expression,
// }

// pub struct BinaryStatement {
//     command: String,
//     arg1: Expression,
//     arg2: Expression,
// }


// pub struct IfStatement {
//     condition: CompareExpression,
//     commands: Vec<Box<dyn Statement>>,
// }

// pub struct WhileStatement {
//     condition: CompareExpression,
//     commands: Vec<Box<dyn Statement>>,
// }

// pub struct ProcedureStatement {
//     name: String,
//     commands: Vec<Box<dyn Statement>>,
// }

// trait Statement {
//     fn execute(&self, values: &HashMap<String, f32>) -> Result<(), ()>;
// }

// impl Statement for NoParaStatement {
//     fn execute(&self, values: &HashMap<String, f32>) -> Result<(), ()> {
//         match self.command.as_str() {
//             "PENUP" => {
//                 println!("PENUP");
//                 Ok(())
//             }
//             "PENDOWN" => {
//                 println!("PENDOWN");
//                 Ok(())
//             }
//             _ => Err(()),
//         }
//     }
// }

// impl Statement for UniStatement {
//     fn execute(&self, values: &HashMap<String, f32>) -> Result<(), ()> {
//         // match self.command.as_str() {
//         //     "FD" => {
//         //         let val = self.arg.evaluate(values);
//         //         println!("FD {}", val);
//         //         Ok(())
//         //     }
//         //     "BK" => {
//         //         let val = self.arg.evaluate(values);
//         //         println!("BK {}", val);
//         //         Ok(())
//         //     }
//         //     "RT" => {
//         //         let val = self.arg.evaluate(values);
//         //         println!("RT {}", val);
//         //         Ok(())
//         //     }
//         //     "LT" => {
//         //         let val = self.arg.evaluate(values);
//         //         println!("LT {}", val);
//         //         Ok(())
//         //     }
//         //     _ => Err(()),
//         // }
//         Ok(())
//     }
// }

