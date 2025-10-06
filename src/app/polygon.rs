use egui::Pos2;
use transform2d::Transform2D;

pub mod transform2d;

// --------------------------------------------------
// Реализация полигона
// --------------------------------------------------

/// Представление полигона. Точка и вектор тоже считаются полигонами.
#[derive(Debug, Clone, PartialEq)]
pub struct Polygon {
    /// Точки полигона. Рёбра идут в направлении от ранних точек к поздним.
    vertexes: Vec<Pos2>,
    intersections: Vec<Pos2>,
}

// --------------------------------------------------
// Конструкторы
// --------------------------------------------------
impl Polygon {
    /// Создание полигона из одной точки
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            vertexes: vec![Pos2::new(x, y)],
            intersections: vec![],
        }
    }

    /// Создание полигона из одной точки
    pub fn from_pos(pos: Pos2) -> Self {
        Self::new(pos.x, pos.y)
    }
}

// --------------------------------------------------
// Приватные вспомогательные методы
// --------------------------------------------------

impl Polygon {
    /// Проверка пересечения двух отрезков ab и cd
    fn segments_intersect(a: Pos2, b: Pos2, c: Pos2, d: Pos2) -> Option<Pos2> {
        let ab_dir = Pos2::new(b.x - a.x, b.y - a.y);
        let cd_dir = Pos2::new(d.x - c.x, d.y - c.y);

        let n = Pos2::new(-cd_dir.y, cd_dir.x);

        let denominator = n.x * ab_dir.x + n.y * ab_dir.y;

        if denominator.abs() < 1e-12 {
            return None;
        }

        let ac = Pos2::new(a.x - c.x, a.y - c.y);
        let numerator = -(n.x * ac.x + n.y * ac.y);
        let t = numerator / denominator;

        if !(0.0..=1.0).contains(&t) {
            return None;
        }

        let intersection = Pos2::new(a.x + t * ab_dir.x, a.y + t * ab_dir.y);

        let cd_to_intersection = Pos2::new(intersection.x - c.x, intersection.y - c.y);
        let dot_product = cd_dir.x * cd_to_intersection.x + cd_dir.y * cd_to_intersection.y;
        let cd_length_squared = cd_dir.x * cd_dir.x + cd_dir.y * cd_dir.y;

        let s = dot_product / cd_length_squared;
        if !(0.0..=1.0).contains(&s) {
            return None;
        }

        Some(intersection)
    }

    /// Обновление списка пересечений при добавлении новой вершины
    fn update_intersections(&mut self) {
        self.intersections.clear();

        let n = self.vertexes.len();
        if n < 4 {
            return;
        }

        for i in 0..n {
            let a = self.vertexes[i];
            let b = self.vertexes[(i + 1) % n];

            for j in (i + 2)..n {
                if (j + 1) % n == i {
                    continue;
                }

                let c = self.vertexes[j];
                let d = self.vertexes[(j + 1) % n];

                if let Some(intersection) = Self::segments_intersect(a, b, c, d)
                    && !self.intersections.iter().any(|&p| {
                        (p.x - intersection.x).abs() < 1e-6 && (p.y - intersection.y).abs() < 1e-6
                    })
                {
                    self.intersections.push(intersection);
                }
            }
        }
    }
}

// --------------------------------------------------
// Операции над полигоном (его изменение)
// --------------------------------------------------

impl Polygon {
    /// Добавить вершину (точку) в полигон.
    pub fn add_vertex(&mut self, x: f32, y: f32) {
        self.vertexes.push(Pos2::new(x, y));
        self.update_intersections();
    }

    /// Добавить вершину (точку) в полигон.
    pub fn add_vertex_pos(&mut self, pos: Pos2) {
        self.add_vertex(pos.x, pos.y);
    }

    /// Применить аффинное преобразование.
    pub fn apply_transform(&mut self, transform: Transform2D) {
        for vertex in &mut self.vertexes {
            *vertex = transform.apply_to_pos(*vertex);
        }

        for intersection in &mut self.intersections {
            *intersection = transform.apply_to_pos(*intersection);
        }
    }
}

// --------------------------------------------------
// Проверки полигона
// --------------------------------------------------

impl Polygon {
    /// Состоит ли полигон только из одной вершины?
    pub fn is_vertex(&self) -> bool {
        self.vertexes.len() == 1
    }

    /// Состоит ли полигон только из одного ребра?
    pub fn is_edge(&self) -> bool {
        self.vertexes.len() == 2
    }

    /// Является ли полигон выпуклым?
    pub fn is_convex(&self) -> bool {
        // TODO проверка на выпуклость текущего полигона
        false
    }

    /// Содержит ли полигон заданную точку?
    pub fn contains(&self, x: f32, y: f32) -> bool {
        // TODO проверка, содержит ли полигон эту точку.
        // Текущий полигон может быть точкой, прямой, выпкулым или невыпуклым многоугольником.
        false
    }

    /// Содержит ли полигон заданную точку?
    pub fn contains_pos(&self, pos: Pos2) -> bool {
        self.contains(pos.x, pos.y)
    }
}
