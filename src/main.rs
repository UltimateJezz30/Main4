pub struct Estudiante {
    nombre: String,
    puntaje_acumulado: i32,
    asistencia: i32,
    puntos_apreciacion: i32,
}

impl Estudiante {
    pub fn nuevo(nombre: &str, puntaje_acumulado: i32, asistencia: i32, puntos_apreciacion: i32) -> Estudiante {
        Estudiante {
            nombre: nombre.to_string(),
            puntaje_acumulado,
            asistencia,
            puntos_apreciacion,
        }
    }

    pub fn calcular_nota_definitiva(&self) -> i32 {
        let mut nota_definitiva = self.puntaje_acumulado;
        if self.asistencia < 75 {
            nota_definitiva -= 1;
        }
        if self.asistencia < 50 {
            nota_definitiva -= 2;
        }
        if self.asistencia < 25 {
            nota_definitiva -= 3;
        }
        nota_definitiva += self.puntos_apreciacion;
        nota_definitiva
    }

    pub fn mostrar(&self) {
        println!("{}: nota definitiva {}", self.nombre, self.calcular_nota_definitiva());
    }
}

fn main() {
    let mut estudiantes = Vec::new();
    estudiantes.push(Estudiante::nuevo("Maria Gonzalez", 9, 100, 1));
    estudiantes.push(Estudiante::nuevo("Juan Perez", 8, 80, 0));
    // Agrega más estudiantes aquí si lo deseas

    for estudiante in &estudiantes {
        estudiante.mostrar();
    }
}
