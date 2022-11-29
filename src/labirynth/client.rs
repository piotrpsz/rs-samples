use crate::labirynth::{Point, Route};
use crate::labirynth::server::Server;

pub struct Theseus<'s> {
    server: &'s Server,
}

impl<'s> Theseus<'s> {
    pub fn new(server: &'s Server) -> Theseus<'s> {
        Theseus{server}
    }

    pub fn search(&self) {
        match self.entry_point() {
            Some(p) => {
                self.search_route(Route::new(p));
                println!("OK");
            },
            None => println!("entry point not founs")
        }
    }

    /// Seaeches rest of the route.
    fn search_route(&self, route: Route){
        let current = route.last_point();
        if current.is_exit() {
            println!("{:?}", route);
            return;
        }

        let possible_steps = self.server.possible_steps(current);
        for p in &possible_steps {
            if !route.contains(&p) {
                let mut new_route = route.clone();
                new_route.append(p.clone());
                self.search_route(new_route);
            }
        }
    }

    /// Searches entry point to labirynth.
    /// (Only one entry point, first from top.)
    fn entry_point(&self) -> Option<Point> {
        let (rows, _) = self.server.size();

        for y in 0..rows {
            let point = Point::new(0, y);
            if self.server.valid_point(&point) {
                return Some(point);
            }
        }
        None
    }
}


