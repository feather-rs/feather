use crate::devtools::VIEWER_WIDTH;
use imgui::{im_str, ComboBox, Slider};
use simdnoise::NoiseBuilder;

pub struct State {
    mode: Mode,
}

impl Default for State {
    fn default() -> Self {
        Self {
            mode: Mode {
                kind: ModeKind::DisplayNoise(NoiseSettings::Fbm(FbmSettings::default())),
                dimensions: VIEWER_WIDTH as i32,
                multiplier: 1.0,
            },
        }
    }
}

impl State {
    pub fn render(&mut self, ui: &imgui::Ui, buffer: &mut [u32], is_first_run: bool) -> bool {
        let mut current_mode = self.mode.ordinal();
        ComboBox::new(im_str!("Mode")).build_simple_string(
            ui,
            &mut current_mode,
            &[im_str!("Display noise")],
        );

        if current_mode != self.mode.ordinal() {
            self.mode = Mode::default_from_ordinal(current_mode);
        }

        self.mode.render(ui, buffer, is_first_run)
    }

    pub fn dimensions(&self) -> u32 {
        self.mode.dimensions as u32
    }
}

#[derive(Copy, Clone)]
struct Mode {
    kind: ModeKind,
    dimensions: i32,
    multiplier: f32,
}

#[derive(Copy, Clone)]
enum ModeKind {
    DisplayNoise(NoiseSettings),
}

impl Mode {
    fn ordinal(self) -> usize {
        match self.kind {
            ModeKind::DisplayNoise(_) => 0,
        }
    }

    fn default_from_ordinal(ordinal: usize) -> Self {
        Self {
            kind: match ordinal {
                0 => ModeKind::DisplayNoise(NoiseSettings::Fbm(FbmSettings::default())),
                x => panic!("invalid mode ordinal {}", x),
            },
            dimensions: VIEWER_WIDTH as i32,
            multiplier: 1.0,
        }
    }

    fn render(&mut self, ui: &imgui::Ui, buffer: &mut [u32], is_first_run: bool) -> bool {
        match &mut self.kind {
            ModeKind::DisplayNoise(settings) => {
                let prev_settings = *settings;
                let mut current_item = settings.ordinal();
                ComboBox::new(im_str!("Noise type")).build_simple_string(
                    ui,
                    &mut current_item,
                    &[
                        im_str!("Fbm"),
                        im_str!("Cellular"),
                        im_str!("Gradient"),
                        im_str!("Ridge"),
                        im_str!("Turbulence"),
                    ],
                );
                if current_item != settings.ordinal() {
                    *settings = NoiseSettings::default_from_ordinal(current_item);
                }

                let multiplier_changed = Slider::new(im_str!("Multiplier"), 1.0f32..=100.0)
                    .build(ui, &mut self.multiplier);
                let dim_changed = ui
                    .input_int(im_str!("Dimensions"), &mut self.dimensions)
                    .build();

                settings.render(ui);

                if prev_settings != *settings || dim_changed || multiplier_changed || is_first_run {
                    // Re-render noise
                    let data = SampledGrid::new(
                        settings.generate(self.dimensions as usize, self.dimensions as usize),
                        self.dimensions as u32,
                    );

                    buffer.iter_mut().enumerate().for_each(|(i, x)| {
                        let val = ((data.sample(i % VIEWER_WIDTH, i / VIEWER_WIDTH)
                            + 0.5f32.sqrt())
                            * 255.0
                            * self.multiplier)
                            .round() as u32;

                        *x = (val << 16) | (val << 8) | val;
                    });
                    true
                } else {
                    false
                }
            }
        }
    }
}

struct SampledGrid {
    dimensions: u32,
    data: Vec<f32>,
    scale: f64,
}

impl SampledGrid {
    pub fn new(data: Vec<f32>, dimensions: u32) -> Self {
        let scale = VIEWER_WIDTH as f64 / dimensions as f64;

        Self {
            dimensions,
            data,
            scale,
        }
    }

    pub fn sample(&self, x: usize, y: usize) -> f32 {
        let x = (x as f64 / self.scale).floor() as usize;
        let y = (y as f64 / self.scale).floor() as usize;

        let index = x + y * self.dimensions as usize;

        if index >= self.data.len() {
            0.0
        } else {
            self.data[index]
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum NoiseSettings {
    Fbm(FbmSettings),
    Cellular(CellularSettings),
    Gradient(GradientSettings),
    Ridge(RidgeSettings),
    Turbulence(TurbulenceSettings),
}

impl NoiseSettings {
    fn ordinal(self) -> usize {
        match self {
            NoiseSettings::Fbm(_) => 0,
            NoiseSettings::Cellular(_) => 1,
            NoiseSettings::Gradient(_) => 2,
            NoiseSettings::Ridge(_) => 3,
            NoiseSettings::Turbulence(_) => 4,
        }
    }

    fn default_from_ordinal(ordinal: usize) -> Self {
        match ordinal {
            0 => NoiseSettings::Fbm(FbmSettings::default()),
            1 => NoiseSettings::Cellular(CellularSettings::default()),
            2 => NoiseSettings::Gradient(GradientSettings::default()),
            3 => NoiseSettings::Ridge(RidgeSettings::default()),
            4 => NoiseSettings::Turbulence(TurbulenceSettings::default()),
            x => panic!("invalid noise setting ordinal {}", x),
        }
    }

    fn generate(self, width: usize, height: usize) -> Vec<f32> {
        match self {
            NoiseSettings::Fbm(settings) => NoiseBuilder::fbm_2d(width, height)
                .with_freq(settings.freq)
                .with_gain(settings.gain)
                .with_lacunarity(settings.lacunarity)
                .with_octaves(settings.octaves)
                .with_seed(settings.seed)
                .generate(),
            NoiseSettings::Cellular(settings) => NoiseBuilder::cellular_2d(width, height)
                .with_freq(settings.freq)
                .with_seed(settings.seed)
                .generate(),
            NoiseSettings::Gradient(settings) => NoiseBuilder::gradient_2d(width, height)
                .with_freq(settings.freq)
                .with_seed(settings.seed)
                .generate(),
            NoiseSettings::Ridge(settings) => NoiseBuilder::ridge_2d(width, height)
                .with_freq(settings.freq)
                .with_gain(settings.gain)
                .with_lacunarity(settings.lacunarity)
                .with_octaves(settings.octaves)
                .with_seed(settings.seed)
                .generate(),
            NoiseSettings::Turbulence(settings) => NoiseBuilder::turbulence_2d(width, height)
                .with_freq(settings.freq)
                .with_gain(settings.gain)
                .with_lacunarity(settings.lacunarity)
                .with_octaves(settings.octaves)
                .with_seed(settings.seed)
                .generate(),
        }
        .0
    }

    fn render(&mut self, ui: &imgui::Ui) {
        match self {
            NoiseSettings::Fbm(settings) => {
                Slider::new(im_str!("Seed"), 0..=1_000_000).build(ui, &mut settings.seed);
                Slider::new(im_str!("Frequency"), 0.0f32..=1.0).build(ui, &mut settings.freq);
                Slider::new(im_str!("Lacunarity"), 0.0..=5.0).build(ui, &mut settings.lacunarity);
                Slider::new(im_str!("Gain"), 0.0..=20.0).build(ui, &mut settings.gain);
                Slider::new(im_str!("Octaves"), 1u8..=16).build(ui, &mut settings.octaves);
            }
            NoiseSettings::Cellular(settings) => {
                Slider::new(im_str!("Seed"), 0..=1_000_000).build(ui, &mut settings.seed);
                Slider::new(im_str!("Frequency"), 0.0f32..=1.0).build(ui, &mut settings.freq);
            }
            NoiseSettings::Gradient(settings) => {
                Slider::new(im_str!("Seed"), 0..=1_000_000).build(ui, &mut settings.seed);
                Slider::new(im_str!("Frequency"), 0.0f32..=1.0).build(ui, &mut settings.freq);
            }
            NoiseSettings::Ridge(settings) => {
                Slider::new(im_str!("Seed"), 0..=1_000_000).build(ui, &mut settings.seed);
                Slider::new(im_str!("Frequency"), 0.0f32..=1.0).build(ui, &mut settings.freq);
                Slider::new(im_str!("Lacunarity"), 0.0..=5.0).build(ui, &mut settings.lacunarity);
                Slider::new(im_str!("Gain"), 0.0..=20.0).build(ui, &mut settings.gain);
                Slider::new(im_str!("Octaves"), 1u8..=16).build(ui, &mut settings.octaves);
            }
            NoiseSettings::Turbulence(settings) => {
                Slider::new(im_str!("Seed"), 0..=1_000_000).build(ui, &mut settings.seed);
                Slider::new(im_str!("Frequency"), 0.0f32..=1.0).build(ui, &mut settings.freq);
                Slider::new(im_str!("Lacunarity"), 0.0..=5.0).build(ui, &mut settings.lacunarity);
                Slider::new(im_str!("Gain"), 0.0..=20.0).build(ui, &mut settings.gain);
                Slider::new(im_str!("Octaves"), 1u8..=16).build(ui, &mut settings.octaves);
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct FbmSettings {
    seed: i32,
    freq: f32,
    lacunarity: f32,
    gain: f32,
    octaves: u8,
}

impl Default for FbmSettings {
    fn default() -> Self {
        Self {
            seed: 0,
            freq: 0.02,
            lacunarity: 0.5,
            gain: 2.0,
            octaves: 3,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct CellularSettings {
    seed: i32,
    freq: f32,
    // todo: distance function
}

impl Default for CellularSettings {
    fn default() -> Self {
        Self {
            seed: 0,
            freq: 0.02,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct GradientSettings {
    seed: i32,
    freq: f32,
}

impl Default for GradientSettings {
    fn default() -> Self {
        Self {
            seed: 0,
            freq: 0.02,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct RidgeSettings {
    seed: i32,
    freq: f32,
    lacunarity: f32,
    gain: f32,
    octaves: u8,
}

impl Default for RidgeSettings {
    fn default() -> Self {
        Self {
            seed: 0,
            freq: 0.02,
            lacunarity: 0.5,
            gain: 2.0,
            octaves: 3,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct TurbulenceSettings {
    seed: i32,
    freq: f32,
    lacunarity: f32,
    gain: f32,
    octaves: u8,
}

impl Default for TurbulenceSettings {
    fn default() -> Self {
        Self {
            seed: 0,
            freq: 0.02,
            lacunarity: 0.5,
            gain: 2.0,
            octaves: 3,
        }
    }
}
