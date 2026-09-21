use macroquad::prelude::*;
const GRAV: f32 = 6.6743e-11 / 100f32; // Constante da Gravitação Universal
const VEL_LUZ: f32 = 299_792_458.; // Velocidade da luz

struct Objeto {
    raio: f32,
    centro: Vec3,
    massa: f32,
    cor: Color,
    vel: Vec3,
    rs: f32,
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
            rs: (2. * GRAV * massa) / (VEL_LUZ * VEL_LUZ) * 100000., // Calcular raio de Schwarzschild.
        }
    }

    fn desenhar_esfera(&self, objetos: &[Objeto]) {
        draw_sphere(
            vec3(
                self.centro.x,
                self.centro.y,
                calcular_z(objetos, self.centro.x, self.centro.y),
            ),
            self.raio,
            None,
            self.cor,
        );
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
        Vec3::new(
            (g_forca * per_x) / self.massa,
            (g_forca * per_y) / self.massa,
            (g_forca * per_z) / self.massa,
        )
    }

    fn checar_colisao(&self, outro: &Objeto) -> Option<(Vec3, Vec3)> {
        let distancia = self.centro.distance(outro.centro);
        let soma_dos_raios = self.raio + outro.raio;
        if distancia <= soma_dos_raios {
            return Some((-self.vel, -outro.vel));
        }
        None
    }
}

fn calcular_z(objetos: &[Objeto], x: f32, y: f32) -> f32 {
    // Distorção total em Z no objeto atual.
    let mut z_total = 0.;

    for obj in objetos {
        // Pitagoras básico para conseguir a distancia r.
        let dx = x - obj.centro.x;
        let dy = y - obj.centro.y;
        let r = (dx * dx + dy * dy).sqrt();

        // Logica para não dar NaN.
        if r > obj.rs {
            let z_objeto = 2.0 * (obj.rs * (r - obj.rs)).sqrt();
            z_total += z_objeto;
        } else {
            z_total += 2.0 * obj.rs;
        }
    }

    z_total
}

fn desenhar_grid(espacamento: f32, comprimento: f32, objetos: &[Objeto]) {
    // Ve a quantidade de linhas horizontais/verticais seram necessarias para cobrir tudo.
    let quant_linhas = ((comprimento * 2.0) / espacamento) as i32;

    for i in 0..=quant_linhas {
        for n in 0..=quant_linhas {
            // Valor atual em x e em y (separados para ser mais facil de mandar para o calcular_z);
            let x = -comprimento + (i as f32 * espacamento);
            let y = -comprimento + (n as f32 * espacamento);

            // Proximo valor de x e y (Serve para poder fazer os passos tambem, já que é necessario, por exemplo, o ponto x atual
            // representado pelo x sozinho, e o proximo valor dele para desenhar a linha fazendo pequenos avanços.) e o min impede
            // que se desenhe algo fora dos limites da grid.
            let prox_x = (x + espacamento).min(comprimento);
            let prox_y = (y + espacamento).min(comprimento);

            // Logica para não desenhar linhas redudantes.
            if x < comprimento {
                //é necessario dois Zs já que a linha é feita de dois nodes.
                let z1 = calcular_z(objetos, x, y);
                let z2 = calcular_z(objetos, prox_x, y);
                draw_line_3d(vec3(x, y, z1), vec3(prox_x, y, z2), GRAY);
            }

            if y < comprimento {
                let z1 = calcular_z(objetos, x, y);
                let z2 = calcular_z(objetos, x, prox_y);
                draw_line_3d(vec3(x, y, z1), vec3(x, prox_y, z2), GRAY);
            }
        }
    }
}

#[macroquad::main("Simulação 3D")]
async fn main() {
    let mut camera = CameraPos::novo(0., 800., 0.);
    let mut objetos = vec![
        Objeto::novo(Vec3::ZERO, Vec3::ZERO, 15.97e24, RED, 100.),
        Objeto::novo(
            vec3(0., 1., 0.),
            Vec3 {
                x: 500.,
                y: 0.,
                z: 0.,
            },
            7.35e22,
            BLUE,
            50.,
        ),
        Objeto::novo(
            vec3(0., -0.75, 0.),
            Vec3 {
                x: -600.,
                y: 0.,
                z: 0.,
            },
            7.35e22,
            GREEN,
            25.,
        ),
    ];
    loop {
        clear_background(BLACK);
        desenhar_grid(50., 1000., &objetos);
        camera.atualizar_camera_pos();

        for p in 0..objetos.len() {
            for n in (1 + p)..objetos.len() {
                let accs_p: Vec3 = objetos[p].calcular_gforce(&objetos[n]);
                let accs_n: Vec3 = objetos[n].calcular_gforce(&objetos[p]);

                objetos[p].acelerar(accs_p);
                objetos[n].acelerar(accs_n);

                if let Some((vel_p, vel_n)) = objetos[p].checar_colisao(&objetos[n]) {
                    objetos[p].vel = vel_p;
                    objetos[n].vel = vel_n;
                }
            }
            objetos[p].desenhar_esfera(&objetos);
            objetos[p].atualizar_pos();
        }

        next_frame().await;
    }
}
