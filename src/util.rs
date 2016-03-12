// use glium::glutin::{VirtualKeyCode, ElementState};
// use glium::glutin::VirtualKeyCode::*;
// use ui::{KeyboardState};


/// Brighten or darken a single color component. 
///
/// An `amount` of `-1.0` will completely minimize (0.0) and `1.0` will completely maximize (1.0).
pub fn adjust_ccmp(component: f32, amount: f32) -> f32 {
    // Clamp amount in [-1.0, 1.0] and component in [0.0, 1.0]:
    let amount = if amount > 1.0 { 1.0 } else if amount < -1.0 { -1.0 } else { amount };
    let component = if component > 1.0 { 1.0 } else if component < 0.0 { 0.0 } else { component };

    if amount >= 0.0 {
        let until_max = 1.0 - component;
        component + (until_max * amount)
    } else {
        let until_min = component;
        component - (-amount * until_min)
    }
}

pub fn adjust_color(color: [f32; 4], amount: f32) -> [f32; 4] {
    [
        adjust_ccmp(color[0], amount),
        adjust_ccmp(color[1], amount),
        adjust_ccmp(color[2], amount),
        color[3],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Need more test variations! Haven't wasted enough time on this yet!
    #[test]
    fn test_adjust_ccmp() {
        assert_eq!(adjust_ccmp(1.0, 1.0), 1.0);
        assert_eq!(adjust_ccmp(5.0, 5.0), 1.0);
        assert_eq!(adjust_ccmp(1.5, 1.5), 1.0);

        assert_eq!(adjust_ccmp(1.0, -1.0), 0.0);
        assert_eq!(adjust_ccmp(-5.0, -5.0), 0.0);
        assert_eq!(adjust_ccmp(5.0, -5.0), 0.0);

        assert_eq!(adjust_ccmp(1.0, 0.0), 1.0);

        assert_eq!(adjust_ccmp(0.5, 1.0), 1.0);
        assert_eq!(adjust_ccmp(0.5, -1.0), 0.0);
        assert_eq!(adjust_ccmp(0.5, 0.0), 0.5);

        assert_eq!(adjust_ccmp(0.5, 0.5), 0.75);
        assert_eq!(adjust_ccmp(0.5, 0.05), 0.525);
        assert_eq!(adjust_ccmp(0.5, -0.5), 0.25);
        assert_eq!(adjust_ccmp(0.5, -0.05), 0.475);
    }
}
