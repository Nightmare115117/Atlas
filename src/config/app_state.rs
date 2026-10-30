pub struct AppState {
    costo: u32,
}

impl AppState {
    
    pub fn new(costo: u32) -> AppState {
        AppState{costo: costo}
    }

    pub fn get_cost(&self) -> u32 {
        self.costo
    }
}