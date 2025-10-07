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
        let n = self.vertexes.len();

        if n < 3 {
            return false;
        }

        let mut sign = 0;

        for i in 0..n {
            let p1 = &self.vertexes[i];
            let p2 = &self.vertexes[(i + 1) % n];
            let p3 = &self.vertexes[(i + 2) % n];

            // векторное произведение
            let cross_product = (p2.x - p1.x) * (p3.y - p2.y) - (p2.y - p1.y) * (p3.x - p2.x);

            if cross_product != 0.0 {
                let current_sign = if cross_product > 0.0 { 1 } else { -1 };

                if sign == 0 {
                    sign = current_sign;
                } else if sign != current_sign {
                    return false;
                }
            }
        }

        true
    }

    /// Содержит ли полигон заданную точку?
    pub fn contains(&self, x: f32, y: f32) -> bool {
        let n = self.vertexes.len();

        match n {
            0 => false,
            1 => (self.vertexes[0].x - x).abs() < 1e-6 && (self.vertexes[0].y - y).abs() < 1e-6,
            2 => {
                let p1 = self.vertexes[0];
                let p2 = self.vertexes[1];

                // коллинеарны ли
                let cross = (p2.x - p1.x) * (y - p1.y) - (p2.y - p1.y) * (x - p1.x);
                if cross.abs() > 1e-6 {
                    return false;
                }

                // лежит ли точка между p1 и p2 (скалярное произведение)
                let dot = (x - p1.x) * (p2.x - p1.x) + (y - p1.y) * (p2.y - p1.y);
                dot >= 0.0 && dot <= ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2))
            }
            _ => {
                let mut inside = false;

                for i in 0..n {
                    let j = (i + 1) % n;
                    let vi = self.vertexes[i];
                    let vj = self.vertexes[j];

                    // пересекает ли луч, идущий вправо от точки, с ребром
                    if ((vi.y > y) != (vj.y > y))
                        && (x < (vj.x - vi.x) * (y - vi.y) / (vj.y - vi.y) + vi.x)
                    {
                        inside = !inside;
                    }
                }

                inside
            }
        }
    }

    /// Содержит ли полигон заданную точку?
    pub fn contains_pos(&self, pos: Pos2) -> bool {
        self.contains(pos.x, pos.y)
    }
}

// --------------------------------------------------
// Вспомогательные функции
// --------------------------------------------------
impl Polygon {
    /// Возвращает центр полигона
    pub fn get_center(&self) -> Pos2 {
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;
        for vertex in &self.vertexes {
            x += vertex.x;
            y += vertex.y;
        }
        Pos2 {
            x: x / self.vertexes.len() as f32,
            y: y / self.vertexes.len() as f32,
        }
    }
}

// --------------------------------------------------
// Рисование полигона
// --------------------------------------------------

impl Polygon {
    fn draw_vertexes(&self, painter: &egui::Painter, style: &PolygonStyle) {
        self.vertexes.iter().for_each(|vertex_pos| {
            painter.circle_filled(*vertex_pos, style.vertex_radius, style.vertex_color);
        });
    }

    fn draw_edges(&self, painter: &egui::Painter, style: &PolygonStyle) {
        let mut points = self.vertexes.clone();
        if points.len() >= 3 {
            points.push(points[0]);
        }
        painter.line(
            points,
            egui::epaint::PathStroke::new(style.edge_width, style.edge_color),
        );
    }

    pub fn draw(&self, painter: &egui::Painter, style: &PolygonStyle) {
        self.draw_vertexes(painter, style);
        self.draw_edges(painter, style);
    }
}

/// Настройка рисования полигона
pub struct PolygonStyle {
    /// Цвет вершины полигона
    vertex_color: egui::Color32,
    /// Радиус вершины полигона
    vertex_radius: f32,

    /// Цвет ребра полигона
    edge_color: egui::Color32,
    /// Толщина ребра полигона
    edge_width: f32,
}

impl PolygonStyle {
    /// Стандартный стиль полигона
    pub fn standard() -> Self {
        PolygonStyle {
            vertex_color: egui::Color32::BLACK,
            vertex_radius: 7.0,
            edge_color: egui::Color32::BLACK,
            edge_width: 5.0,
        }
    }

    /// Стиль выбранного полигона
    pub fn selected() -> Self {
        PolygonStyle {
            vertex_color: egui::Color32::LIGHT_BLUE,
            vertex_radius: 10.0,
            edge_color: egui::Color32::LIGHT_BLUE,
            edge_width: 7.0,
        }
    }
}
