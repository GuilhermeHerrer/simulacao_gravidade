use macroquad::prelude::*;

const GRAV: f32 = 6.6743e-11 / 100f32; // Constante da Gravitação Universal
// Dividida por 100 para evitar tudo sair voando :)

struct Objeto {
    v: (f32, f32),
    massa: f32,
    circulo: Circle,
    cor: Color,
}

impl Objeto {
    // Cria um circulo
    pub fn new(v: (f32, f32), r: f32, pos: (f32, f32), cor: Color, massa: f32) -> Self {
        Self {
            v,
            cor,
            massa,
            circulo: Circle {
                x: pos.0,
                y: pos.1,
                r,
            },
        }
    }

    //Desenha o circulo
    pub fn desenhar(&self) {
        draw_circle(self.circulo.x, self.circulo.y, self.circulo.r, self.cor);
    }

    // Soma a aceleração a velocidade pelo dt
    pub fn acelerar(&mut self, x: f32, y: f32) {
        self.v.0 += x;
        self.v.1 += y;
    }

    // Atualiza a posição baseado na velocidade
    pub fn atualizar_pos(&mut self) {
        self.circulo.x += self.v.0;
        self.circulo.y += self.v.1;
    }

    // Calcula a força da gravidade entre 2 objetos
    pub fn calcular_gforce(&self, outro: &Objeto) -> (f32, f32) {
        let dx = outro.circulo.x - self.circulo.x;
        let dy = outro.circulo.y - self.circulo.y;
        let mut distancia = (dx * dx + dy * dy).sqrt();
        let (per_x, per_y) = (dx / distancia, dy / distancia);

        // Evita tudo sair voando pq a força seria grande demais :)
        distancia *= 105000f32;

        //  Calcula a força de aceleração.
        let g_forca = (GRAV * self.massa * outro.massa) / (distancia * distancia);

        // Aceleração em x e aceleração em y
        (
            (g_forca * per_x) / self.massa,
            (g_forca * per_y) / self.massa,
        )
    }

    // Checa a colisão entre 2 objetos, se tiver retorna duas tuplas com as acelerações.
    pub fn checar_colisao(&self, outro: &Objeto) -> Option<((f32, f32), (f32, f32))> {
        if self.circulo.overlaps(&outro.circulo) {
            return Some(((-self.v.0, -self.v.1), (-outro.v.0, -outro.v.1)));
        }
        None
    }

    // Colisões com as paredes
    pub fn colisoes_paredes(&mut self) {
        // Colisão vertical (topo)
        if self.circulo.y - self.circulo.r < 0f32 {
            self.circulo.y = self.circulo.r;
            self.v.1 = -self.v.1;
        }
        // Colisão vertical (fundo)
        else if self.circulo.y + self.circulo.r > screen_height() {
            self.circulo.y = screen_height() - self.circulo.r;
            self.v.1 = -self.v.1;
        }

        // Colisão horizontal (esquerda)
        if self.circulo.x - self.circulo.r < 0f32 {
            self.circulo.x = self.circulo.r;
            self.v.0 = -self.v.0;
        }
        // Colisão horizontal (direita)
        else if self.circulo.x + self.circulo.r > screen_width() {
            self.circulo.x = screen_width() - self.circulo.r;
            self.v.0 = -self.v.0;
        }
    }
}

#[macroquad::main("Gravidade")]
async fn main() {
    // Adicione objetos aqui!
    let mut objs = vec![
        Objeto::new(
            // Lua
            (0f32, 0.75),
            20f32,
            (screen_width() / 4f32 - 150f32, screen_height() / 2f32),
            RED,
            7.35e22,
        ),
        Objeto::new(
            // Terra
            (0f32, 0f32),
            50f32,
            (screen_width() / 2f32, screen_height() / 2f32),
            RED,
            5.97e24,
        ),
        Objeto::new(
            // Plutão
            (0f32, 0.75),
            10f32,
            (screen_width() / 4f32 + 500f32, screen_height() / 2f32),
            RED,
            1.3e22,
        ),
    ];
    loop {
        for i in 0..objs.len() {
            for n in (1 + i)..objs.len() {
                let (acc_i_x, acc_i_y) = objs[i].calcular_gforce(&objs[n]);
                let (acc_n_x, acc_n_y) = objs[n].calcular_gforce(&objs[i]);

                objs[i].acelerar(acc_i_x, acc_i_y);
                objs[n].acelerar(acc_n_x, acc_n_y);

                if let Some((v_i, v_n)) = objs[i].checar_colisao(&objs[n]) {
                    objs[i].v.0 = v_i.0;
                    objs[i].v.1 = v_i.1;

                    objs[n].v.0 = v_n.0;
                    objs[n].v.1 = v_n.1;
                } 
            }

            objs[i].desenhar();
            objs[i].colisoes_paredes();
            objs[i].atualizar_pos();
        }
        next_frame().await;
    }
}
