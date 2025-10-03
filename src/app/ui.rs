use crate::app::AthenianApp;

// --------------------------------------------------
// Построение UI приложения
// --------------------------------------------------

impl eframe::App for AthenianApp {
    /// Главный цикл UI.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.show_top_panel(ctx);
        self.show_left_panel(ctx);
        self.show_bottom_panel(ctx);
        self.show_cental_panel(ctx);
    }
}

impl AthenianApp {
    /// Показать верхную панель приложения.
    fn show_top_panel(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    // Создать новый холст
                    if ui.button("New (TODO)").clicked() {
                        // TODO создание области новых размеров
                        println!("Кнопка ничего не делает");
                        ui.close();
                    }

                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });
    }

    /// Показать левую панель приложения.
    fn show_left_panel(&self, ctx: &egui::Context) {
        egui::SidePanel::left("left_panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // TODO выбор цвета
                    ui.label("Выбор цвета пока не робит"); // TODO потом убрать
                    let mut tmp_color = egui::Color32::BLACK;
                    egui::color_picker::color_picker_color32(
                        ui,
                        &mut tmp_color,
                        egui::color_picker::Alpha::Opaque,
                    );

                    ui.separator();

                    ui.label("Инструменты:");

                    if ui.button("Добавление полигона (TODO)").clicked() {
                        // TODO создание области новых размеров
                        println!("Кнопка ничего не делает");
                    }

                    if ui.button("Стереть всё (TODO)").clicked() {
                        // TODO создание области новых размеров
                        println!("Кнопка ничего не делает");
                    }

                    if ui.button("Принадлежность точки полигону (TODO)").clicked()
                    {
                        // TODO создание области новых размеров
                        println!("Кнопка ничего не делает");
                    }

                    if ui.button("С какой стороны точка? (TODO)").clicked() {
                        // TODO создание области новых размеров
                        println!("Кнопка ничего не делает");
                    }
                });
            });
    }

    /// Показать нижнюю панель приложения.
    fn show_bottom_panel(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // TODO отображение имени текущего инстурмента
                ui.label(format!("инструмент: {}", "TODO"));

                ui.separator();

                // TODO отображение размеров текущей области (в логических пикселях)
                ui.label(format!("размер холста: {} x {}", -1, -1));

                ui.separator();

                // TODO отображение размера текущей области (в реальных пикселях на экране)
                ui.label(format!("отображаемый размер: {:.1} x {:.1}", -1, -1));
            });
        });
    }

    /// Показать центральную (основную) панель приложения.
    fn show_cental_panel(&self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("TODO центральная область");
            // TODO
            // // Выделить область под холст
            // let (canvas_response, painter) = self.allocate_canvas(ui);
            // let canvas_rect = canvas_response.rect;

            // self.display_canvas_width = canvas_rect.width();
            // self.display_canvas_height = canvas_rect.height();

            // // Обработать рисование
            // match self.cur_instrument {
            //     Instrument::Pencil => self.handle_pencil(canvas_rect, &canvas_response),
            //     Instrument::Bucket => self.handle_bucket(canvas_rect, &canvas_response),
            //     Instrument::ImageBucket => self.handle_image_bucket(canvas_rect, &canvas_response),
            //     Instrument::Border => self.handle_border(canvas_rect, &canvas_response),
            //     Instrument::SharpLine => self.handle_sharp_line(canvas_rect, &canvas_response),
            //     Instrument::SmoothLine => self.handle_smooth_line(canvas_rect, &canvas_response),
            //     Instrument::Triangle => self.handle_triangle(canvas_rect, &canvas_response),
            // };

            // // Вывести текущий холст на экран
            // if let Some(texture) = &self.texture_handle {
            //     painter.image(
            //         texture.id(),
            //         canvas_rect,
            //         egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
            //         egui::Color32::WHITE,
            //     );
            // }
        });
    }
}
