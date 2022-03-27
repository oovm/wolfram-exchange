use crate::{Circle, Ellipse, Point};
use wolfram_wxf::{ToWolfram, WolframValue};

impl ToWolfram for Ellipse {
    fn to_wolfram(&self) -> WolframValue {
        todo!()
    }
}

impl ToWolfram for Circle {
    /// ```wolfram
    /// Circle[{x,y},r]
    /// ```
    fn to_wolfram(&self) -> WolframValue {
        let center = WolframValue::list(vec![self.center.x.to_wolfram(), self.center.y.to_wolfram()]);
        WolframValue::function("Circle", vec![center, self.radius.to_wolfram()])
    }
}

impl ToWolfram for Point {
    /// ```wolfram
    /// Point[{x, y}]
    /// ```
    fn to_wolfram(&self) -> WolframValue {
        let point = WolframValue::list(vec![self.x.to_wolfram(), self.y.to_wolfram()]);
        WolframValue::function("Point", vec![point])
    }
}
