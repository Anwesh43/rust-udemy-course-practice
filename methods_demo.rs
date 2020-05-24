struct Point {
    x : f64,
    y : f64
}

struct Line {
    start : Point,
    end : Point
}

impl Line {
    fn dx(&self) -> f64 {
        println!("calling dx method");
        return self.start.x - self.end.x;
    }

    fn dy(&self) -> f64 {
        println!("calling dy method");
        return self.start.y - self.end.y;
    }

    fn dx2(&self) -> f64 {
        return self.dx() * self.dx();
    }

    fn dy2(&self) -> f64 {
        return self.dy() * self.dy();
    }

    fn len(&self) -> f64 {
        // let dx = self.start.x  - self.end.x;
        // let dy = self.start.y - self.end.y;
        return (self.dx2() + self.dy2()).sqrt();
    }
}

fn main() {
    let p1 : Point = Point {x : 0.0, y : 8.0};
    let p2 : Point = Point {x : 6.0, y : 0.0};
    let line : Line = Line {start : p1, end : p2};
    println!("length of line is {}", line.len());
}
