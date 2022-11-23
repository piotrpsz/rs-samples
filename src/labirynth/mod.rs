#![allow(dead_code)]

use serde::{Deserialize, Serialize};

pub mod server;
pub mod client;

/********************************************************************
*                                                                   *
*                             P O I N T                             *
*                                                                   *
********************************************************************/

#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    x: usize,
    y: usize,
    exit: bool,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y, exit: false }
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

/********************************************************************
*                                                                   *
*                            R O U T E                              *
*                                                                   *
********************************************************************/

pub struct Route(Vec<Point>);

impl Route {
    /// Creates new route for passed point.
    /// No route can be empty.
    pub fn new(p: Point) -> Route {
        let mut data: Vec<Point> = Vec::new();
        data.push(p);
        Route(data)
    }

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
        let p = self[self.0.len() - 1];
        Point::new(p.x, p.y)
    }

    pub fn append(&mut self, p: &Point) {
        self.0.push(p);
    }

    pub fn clone(&self) -> Route {
        self.0.cop
    }
}