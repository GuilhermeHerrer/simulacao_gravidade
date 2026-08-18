use macroquad::{prelude::*};
const GRAV: f32 = 6.6743e-11 / 100f32; // Constante da Gravitação Universal

struct Objeto {
    raio: f32,
    centro: Vec3,
    massa: f32,
    cor: Color,
    vel: Vec3,
}

struct CameraPos {
    phi: f32,   // Teoricamente Z (Angulo entre a camera e Z)
    theta: f32, // Teoricamente X (Angulo entre a camera e X)
    rho: f32,   // Teoricamente Y (Distancia do centro)
}

impl CameraPos {
    fn novo(phi: f32, rho: f32, theta: f32) -> Self {
        CameraPos { phi, theta, rho }
    }

    fn atualizar_camera_pos(&mut self) {
        if is_mouse_button_down(MouseButton::Left) {
            let mouse_delta = mouse_delta_position(); // dx e dy entre a posição atual e antiga do mouse
            self.theta += mouse_delta.x * 5.;
            self.phi += mouse_delta.y * 5.;
        }

        let scroll = mouse_wheel().1; // pega o valor y do scroll do mouse
        self.rho *= 1.0 - scroll * 0.1; // matematica pra suavizar o zoom

        self.phi = self.phi.clamp(0.1, std::f32::consts::PI - 0.1);
        let r = self.rho * self.phi.sin();

        let (x, y, z) = (
            r * self.theta.cos(),
            r * self.theta.sin(),
            self.rho * self.phi.cos(),
        );

        let camera = Camera3D {
            position: Vec3::new(x, y, z),
            target: Vec3::ZERO,
            up: Vec3::Z,
            ..Default::default()
        };

        set_camera(&camera);
    }
}

impl Objeto {
    fn novo(vel: Vec3, centro: Vec3, massa: f32, cor: Color, raio: f32) -> Self {
        Objeto {
            raio,
            centro,
            massa,
            cor,
            vel,
        }
    }

    fn desenhar_esfera(&self) {
        draw_sphere(self.centro, self.raio, None, self.cor);
    }

    fn acelerar(&mut self, accs: Vec3) {
        self.vel += accs;
    }

    fn atualizar_pos(&mut self) {
        self.centro += self.vel;
    }

    // Calcula a força da gravidade entre 2 objetos
    pub fn calcular_gforce(&self, outro: &Objeto) -> Vec3 {
        let dx = outro.centro.x - self.centro.x;
        let dy = outro.centro.y - self.centro.y;
        let dz = outro.centro.z - self.centro.z;

        let mut distancia = (dx * dx + dy * dy + dz * dz).sqrt();
        let (per_x, per_y, per_z) = (dx / distancia, dy / distancia, dz / distancia);

        // Evita tudo sair voando pq a força seria grande demais :)
        distancia *= 105000f32;

        //  Calcula a força de aceleração.
        let g_forca = (GRAV * self.massa * outro.massa) / (distancia * distancia);

        // Aceleração em x, aceleração em y e em z
        Vec3::new
        (
            (g_forca * per_x) / self.massa,
            (g_forca * per_y) / self.massa,
            (g_forca * per_z) / self.massa,
        )
    }


    fn checar_colisao(&self, outro: &Objeto) -> Option<(Vec3, Vec3)>{
        let distancia = self.centro.distance(outro.centro);
        let soma_dos_raios = self.raio + outro.raio;
        if distancia <= soma_dos_raios {
            return Some((-self.vel, -outro.vel));
        }
        None
    }
}


fn desenhar_grid(espacamento: f32, comprimento: f32) {
    draw_line_3d(vec3(0., comprimento, 0.), vec3(0., -comprimento, 0.), WHITE);
    draw_line_3d(vec3(comprimento, 0., 0.), vec3(-comprimento, 0., 0.), WHITE);

    let quantidade = (comprimento / espacamento) as i32;

    for n in 0..=quantidade {
        let distancia = n as f32 * espacamento;

        draw_line_3d(vec3(distancia, comprimento, 0.), vec3(distancia, -comprimento, 0.), WHITE);
        draw_line_3d(vec3(comprimento, distancia, 0.), vec3(-comprimento, distancia, 0.), WHITE);
        draw_line_3d(vec3(-distancia, comprimento, 0.), vec3(-distancia, -comprimento, 0.), WHITE);
        draw_line_3d(vec3(comprimento, -distancia, 0.), vec3(-comprimento, -distancia, 0.), WHITE);
    }
}

#[macroquad::main("Simulação 3D")]
async fn main() {
    let mut camera = CameraPos::novo(0., 800., 0.);
    let mut objetos = vec![
        Objeto::novo(Vec3::ZERO, Vec3::ZERO, 15.97e24, RED, 100.),
        Objeto::novo(vec3(0., 0.75, 0.75), Vec3 { x: 500., y: 0., z: 0. }, 7.35e22, BLUE, 50.),
        Objeto::novo(vec3(0., -0.75, -0.75), Vec3 { x: -600., y: 0., z: 250. }, 7.35e22, GREEN, 25.)
        ];
    loop {
        clear_background(BLACK);
        desenhar_grid(50., 1000.);
        camera.atualizar_camera_pos();

        for p in 0..objetos.len(){
            for n in (1 + p)..objetos.len(){
                let accs_p: Vec3 = objetos[p].calcular_gforce(&objetos[n]);
                let accs_n: Vec3 = objetos[n].calcular_gforce(&objetos[p]);

                objetos[p].acelerar(accs_p);
                objetos[n].acelerar(accs_n);

                if let Some((vel_p, vel_n)) = objetos[p].checar_colisao(&objetos[n]){
                    objetos[p].vel = vel_p;
                    objetos[n].vel = vel_n;
                }
            }
            objetos[p].desenhar_esfera();
            objetos[p].atualizar_pos();
        }

        next_frame().await;
    }
}
