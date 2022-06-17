pub fn run(operation: &Operation) -> Result<(),&str> {
    let output = operation.calculate();

    if let None = output {
        return Err("Failed to produce an output");
    } else {
        println!("Result: {}", output.unwrap())
    }

    Ok(())
}

pub struct Operation {
    pub num_one: f64,
    pub operator: fn(f64, f64) -> Option<f64>,
    pub num_two: f64
}

impl Operation {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 4 {
            return Err("too few arguments");
        }

        let num_one: f64 = args[1].clone().parse::<f64>().unwrap();
        let num_two: f64 = args[3].clone().parse::<f64>().unwrap();
        let operator_str: &str = (&args[2][..]).clone();

        let operator: fn(f64, f64) -> Option<f64> = match operator_str {
            "+" => add,
            "-" => subtract,
            "/" => divide,
            "*" => multiply,
            _ => return Err("invalid operator")
        };

        Ok(Self {num_one, operator, num_two})
    }

    pub fn calculate(&self) -> Option<f64> {
        (self.operator)(self.num_one, self.num_two)
    }
}

fn add(left: f64, right: f64) -> Option<f64> {
    Some(left + right)
}

fn subtract(left: f64, right: f64) -> Option<f64> {
    Some(left - right)
}

fn multiply(left: f64, right: f64) -> Option<f64> {
    Some(left * right)
}

fn divide(left: f64, right: f64) -> Option<f64> {
    Some(left / right)
}