use macroquad::{
    camera::Camera3D,
    input::{is_mouse_button_down, mouse_delta_position, mouse_wheel},
    math::Vec3,
    prelude::*,
    window::next_frame,
};

struct Node {
    pos: Vec3,
}

impl Node {
    fn novo(pos: Vec3) -> Self {
        Node { pos }
    }
}

fn criar_grid(nodes: &[Node]) {
    draw_sphere(Vec3::ZERO, 1., None, YELLOW); // desenha um esfera no centro da grid

    let total = nodes.len(); // ve o tamanho do vetor que tem todos os nodes

    for i in 0..total { // Eixo X
        for j in 0..total { // Eixo Y
            let (x, y) = (nodes[i].pos.x, nodes[j].pos.x); // Pego valor de X de ambas variaveis(inicialmente iguais)
                                                                     // Serve para saber a posição x de ambos os pontos, no momento em que eles mudarem
                                                                     // Ambos iniciam 0,0 mas abaixo adicionamos 1 ao index de X para podermos traçar as linhas
            if i + 1 < total { // Anda um na lista
                let x_prox = nodes[i + 1].pos.x; // Pega o valor de X de i + 1.


                // Traça as linhas entre os nodes do vetor nodes, agora i está um valor acima de j, oque permite traçar essas linhas!
                draw_line_3d(vec3(x, y, 0.), vec3(x_prox, y, 0.), WHITE); // quando i+1 = 1 start = 0,0,0 end = 50,0,0
                draw_line_3d(vec3(x, -y, 0.), vec3(x_prox, -y, 0.), WHITE); // start = 0,-0,0 end = 50,-0,0
                draw_line_3d(vec3(-x, y, 0.), vec3(-x_prox, y, 0.), WHITE); // start = -0,0,0 end = -50,0,0
                draw_line_3d(vec3(-x, -y, 0.), vec3(-x_prox, -y, 0.), WHITE); // start = -0,-0,0 end = -50,-0,0
            }

            // Mesma logica anterior, porem para Y
            if j + 1 < total {
                let y_prox = nodes[j + 1].pos.x;

                draw_line_3d(vec3(x, y, 0.), vec3(x, y_prox, 0.), WHITE); // quando j+1 = 1 start = 0,0,0 end = 0, 50, 0
                draw_line_3d(vec3(x, -y, 0.), vec3(x, -y_prox, 0.), WHITE); // start = 0, -0,0 end = 0, -50, 0 
                draw_line_3d(vec3(-x, y, 0.), vec3(-x, y_prox, 0.), WHITE); // start = -0,0,0 end = -0,50,0
                draw_line_3d(vec3(-x, -y, 0.), vec3(-x, -y_prox, 0.), WHITE); //start = -0,-0,0 end = -0,-50,0
            }
        }
    }
}

#[macroquad::main("Grid com vertices")]
async fn main() {
    let mut theta: f32 = 0.;
    let mut phi: f32 = 2.;
    let mut rho: f32 = 800.;

    let comprimento = 500.; // tamanho da grid
    let espacamento = 50.; // espaço entre as linhas
    let quantidade = (comprimento / espacamento) as i32; // calcula quantas colunas e linhas vai ter
    let mut nodes: Vec<Node> = vec![]; // inicia um vetor pra armazenar todos os nodes

    for n in 0..=quantidade { // cria todos os nodes ao longo do eixo x
        nodes.push(Node::novo(vec3(
            n as f32 * espacamento,
            0.,
            0.,
        )));
    }

    loop {
        clear_background(BLACK);
        if is_mouse_button_down(MouseButton::Left) {
            let mouse_delta = mouse_delta_position();
            theta += mouse_delta.x * 5.;
            phi += mouse_delta.y * 5.;
        }

        let scroll = mouse_wheel().1;
        rho *= 1.0 - scroll * 0.1;

        phi = phi.clamp(0.1, std::f32::consts::PI - 0.1);

        let r = rho * phi.sin();

        let (x, y, z) = (r * theta.cos(), r * theta.sin(), rho * phi.cos());

        let camera = Camera3D {
            position: Vec3::new(x, y, z),
            target: Vec3::ZERO,
            up: Vec3::Z,
            ..Default::default()
        };

        set_camera(&camera);

        draw_sphere(Vec3::ZERO, 100., None, RED);
        draw_sphere(vec3(0., 0., 500.), 100., None, BLUE);
        draw_cube(vec3(0., 400., 0.), vec3(100., 100., 100.), None, GRAY);
        criar_grid(&nodes);
        set_default_camera();

        println!(
            "Câmera: x={:.0}, y={:.0}, z={:.0}",
            camera.position.x, camera.position.y, camera.position.z
        );
        println!("phi: {:.0}, rho: {:.0}, theta: {:.0}", phi, rho, theta);
        next_frame().await;
    }
}
