#![allow(dead_code)]

use serde::{Deserialize, Serialize};

pub mod server;
pub mod client;

/********************************************************************
*                                                                   *
*                             P O I N T                             *
*                                                                   *
********************************************************************/

#[derive(Serialize, Deserialize, Copy, Debug)]
pub struct Point {
    x: usize,
    y: usize,
    exit: bool,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y, exit: false }
    }

    pub fn is_exit(&self) -> bool {
        self.exit
    }

    fn from_str(text: &str) -> Option<Point> {
        match serde_json::from_str(text) {
            Ok(p) => Some(p),
            Err(err) => {
                println!("{:?}", err);
                None
            }
        }
    }

    pub fn to_json(&self) -> Option<String> {
        match serde_json::to_string(self) {
            Ok(text) => Some(text),
            Err(err) => {
                println!("{:?}", err);
                None
            }
        }
    }
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Point{x: self.x, y: self.y, exit: self.exit}
    }
}


/********************************************************************
*                                                                   *
*                            R O U T E                              *
*                                                                   *
********************************************************************/

#[derive(Debug)]
pub struct Route(Vec<Point>);


impl Route {
    /// Creates new route for passed point.
    /// No route can be empty.
    pub fn new(p: Point) -> Route {
        let mut data: Vec<Point> = Vec::new();
        data.push(p);
        Route(data)
    }

    /// Chekcs if passed point is already in the route.
    pub fn contains(&self, point: &Point) -> bool {
        for p in &self.0 {
            if (p.x == point.x) && (p.y == point.y) {
                return true;
            }
        }
        false
    }

    /// Returns the last point of the route.
    /// We remember that the route is never empty.
    pub fn last_point(&self) -> Point {
        let i = self.0.len() - 1;
        let p = &self.0[i];
        p.clone()
    }

    /// Appends next point to route.
    pub fn append(&mut self, p: Point) {
        self.0.push(p);
    }
}

impl Clone for Route {
    fn clone(&self) -> Route {
        Route(self.0.clone())
    }
}
