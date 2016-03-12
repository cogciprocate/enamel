#[allow(dead_code)]

// use window::{self};
use ui::{self, Vertex};

const BRDR_Z_OFFSET: f32 = ui::SUBSUBDEPTH;

#[derive(Clone, Copy)]
struct Line {
    pub x: f32,
    pub y: f32,
    pub m: f32,
}


#[derive(Clone, Debug)]
pub struct Shape2d {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub perim: Vec<u16>,
    pub radii: (f32, f32),
    pub color: [f32; 4],
}

impl Shape2d {
    pub fn rectangle(height: f32, width: f32, depth: f32, color: [f32; 4], 
            // ) -> (Vec<Vertex>, Vec<u16>, (f32, f32)) 
            ) -> Shape2d
    {
        let top = height / 2.0;
        let bot = -height / 2.0;
        let left = -width / 2.0;
        let right = width / 2.0;

        let xy_normal = [0.0, 0.0];

        let vertices = vec![
            Vertex::new([ 0.0,      0.0,      depth], color, xy_normal, false),
            Vertex::new([ left,      top,      depth], color, xy_normal, true),
            Vertex::new([ right,      top,      depth], color, xy_normal, true),
            Vertex::new([ right,      bot,      depth], color, xy_normal, true),
            Vertex::new([ left,      bot,      depth], color, xy_normal, true),
        ];

        // println!("\n\n##### Rectangle vertices: {:?}\n", vertices[0].position());

        let indices = vec![
            0, 1, 2,
            2, 3, 0,
            0, 3, 4,
            4, 1, 0,
        ];

        let perim = vec![
            1, 2, 3, 4,
        ];

        let radii = (right, top);

        // (vertices, indices, radii)
        Shape2d { vertices: vertices, indices: indices, perim: perim, radii: radii, 
            color: color }    
    }

    
    pub fn hexagon_panel(height: f32, ew: f32, depth: f32, color: [f32; 4], 
            // ) -> (Vec<Vertex>, Vec<u16>, (f32, f32)) 
            ) -> Shape2d
    {
        // NOTE: width(x): 1.15470053838 (2/sqrt(3)), height(y): 1.0
        let sqrt_3_inv = 1.732050808;

        let a = height / 2.0;
        let s = 1.0 / sqrt_3_inv; // 1/sqrt(3)
        let hs = s * 0.5;

        let vertices = vec![
            Vertex::new([ 0.0,          0.0,      depth], color, [0.0, 0.0], false),
            Vertex::new([-(hs + ew),     a,       depth], color, [0.0, 0.0], true),
            Vertex::new([ hs + ew,      a,       depth], color, [0.0, 0.0], true),
            Vertex::new([ s + ew,      0.0,       depth], color, [0.0, 0.0], true),
            Vertex::new([ hs + ew,     -a,      depth], color, [0.0, 0.0], true),
            Vertex::new([-(hs + ew),     -a,       depth], color, [0.0, 0.0], true),
            Vertex::new([-(s + ew),       0.0,       depth], color, [0.0, 0.0], true),
        ];

        let indices = vec![
            0, 1, 2,
            2, 3, 0,
            0, 3, 4,
            4, 5, 0,
            0, 5, 6,
            6, 1, 0u16,
        ];

        let perim = vec![
            1, 2, 3, 4, 5, 6,
        ];

        let radii = (ew + (s * 0.75), a);

        // (vertices, indices, radii)
        Shape2d { vertices: vertices, indices: indices, perim: perim, radii: radii, 
            color: color }    
    }



    pub fn perim_edges(&self) -> Vec<(usize, (u16, u16))> {
        (0..self.perim.len()).into_iter()
            .map(|edge_idx| {                
                let v_1 = if edge_idx == 0 {
                        self.perim[self.perim.len() - 1]
                    } else {
                        self.perim[edge_idx - 1]
                    };

                let v_2 = self.perim[edge_idx];

                (self.perim[edge_idx] as usize, (v_1, v_2))
            })
            .collect()
    }

    /// Returns a shape with edges extended away from the center by the desired border thickness 't'.
    pub fn as_border(&self, t: f32, color: [f32; 4]) -> Shape2d {
        let perim_edges = self.perim_edges();
        let mut border_lines = Vec::<Line>::with_capacity(perim_edges.len());

        // Find a line which is parallel to each perimeter edge with distance 't':
        for edge in perim_edges.iter() {
            // Get the 3d positions for each vertex on our (original) edge:
            let pos1_3d = self.vertices[(edge.1).0 as usize].position();
            let pos2_3d = self.vertices[(edge.1).1 as usize].position();

            // Create a two-tuple (x, y) for both points, ignoring z:
            let p1 = (pos1_3d[0], pos1_3d[1]);
            let p2 = (pos2_3d[0], pos2_3d[1]);

            // Find slope:
            let m = (p2.1 - p1.1) / (p2.0 - p1.0);
            let m_inv = 1.0 / m;

            // Halfway point between p1 and p2:
            let p = ((p1.0 + p2.0) / 2.0, (p1.1 + p2.1) / 2.0);

            // Delta x and y (+/-):
            let x_ofs = m_inv.atan().cos().abs() * t;
            let y_ofs = m_inv.atan().sin().abs() * t;

            // Shift p away from origin by delta x and y:
            let q = ((p.0 + (x_ofs * p.0.signum())), 
                (p.1 + (y_ofs * p.1.signum())));            

            border_lines.push(Line { x: q.0, y: q.1, m: m });

            // println!("        [p{e0}: {:?}, p{e1}: {:?}, p: {:?}, m: {m}] => [q: {:?}, m: {m}]",
            //     p1, p2, p, q, e0 = (edge.1).0, e1 = (edge.1).1, m = m);
        }

        let mut border_vertices: Vec<(f32, f32)> = Vec::with_capacity(perim_edges.len());

        // Find the intersection between each border line and the one preceeding it:
        for l_idx in 0..border_lines.len() {
            let l1 = if l_idx == 0 {
                    border_lines[border_lines.len() - 1]
                } else {                
                    border_lines[l_idx - 1]
                };                
            let l2 = border_lines[l_idx];

            let (x, y) = if l1.m.is_infinite() {
                    (l1.x, l2.y)
                } else if l2.m.is_infinite() {
                    (l2.x, l1.y)
                } else {
                    let x = ((l1.m * l1.x) - (l2.m * l2.x) - l1.y + l2.y) / (l1.m - l2.m);
                    let y = (l1.m * (x - l1.x)) + l1.y;

                    debug_assert!(((l2.m * (x - l2.x)) + l2.y) - y < 0.0001);

                    (x, y)
                };

            border_vertices.push((x, y));
            // println!("          Intersection: ({}, {})", x, y);
        }

        let mut vertices = self.vertices.clone();

        vertices[0] = self.vertices[0].color(color);

        for l_idx in 0..border_lines.len() {
            let vert_idx = perim_edges[l_idx].0;
            let (v_x, v_y) = border_vertices[l_idx];

            // if l_idx > 1 { break; }

            vertices[vert_idx] = 
                Vertex::new([v_x, v_y, self.vertices[vert_idx].position()[2] + BRDR_Z_OFFSET],
                    color, [0.0, 0.0], self.vertices[vert_idx].is_perimeter());
        }

        let new_shape = Shape2d { 
            vertices: vertices, 
            indices: self.indices.clone(),
            perim: self.perim.clone(),
            color: color, 
            radii: (self.radii.0 + t, self.radii.1 + t),
        };

        // print!("\n");

        new_shape
    }
}

