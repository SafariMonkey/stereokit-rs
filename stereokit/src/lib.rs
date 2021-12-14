pub mod settings;

mod lifecycle;
pub use lifecycle::StereoKit;

pub mod state;
pub use state::StereoKitState;

mod math;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
