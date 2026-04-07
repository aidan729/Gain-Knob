use nih_plug::prelude::*;
use nih_plug_slint::SlintEditor;
use std::sync::Arc;

mod gui;

#[derive(Params)]
pub struct GainKnobParams {
    #[id = "gain"]
    pub gain: FloatParam,
}

impl Default for GainKnobParams {
    fn default() -> Self {
        Self {
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-60.0),
                    max: util::db_to_gain(6.0),
                    factor: FloatRange::gain_skew_factor(-60.0, 6.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
        }
    }
}

pub struct GainKnob {
    params: Arc<GainKnobParams>,
}

impl Default for GainKnob {
    fn default() -> Self {
        Self {
            params: Arc::new(GainKnobParams::default()),
        }
    }
}

impl Plugin for GainKnob {
    const NAME: &'static str = "Gain Knob";
    const VENDOR: &'static str = "AmbiosDSP";
    const URL: &'static str = "www.ambiosdsp.com";
    const EMAIL: &'static str = "info@ambiosdsp.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        Some(Box::new(
            SlintEditor::with_factory(|| gui::AppWindow::new(), (300, 360))
                .with_setup({
                    let params = self.params.clone();

                    move |window_handler, _window| {
                        let component = window_handler.component();
                        let context = window_handler.context().clone();

                        // UI -> Plugin: knob drag callbacks (registered once on open)
                        {
                            let params = params.clone();
                            let context = context.clone();
                            component.on_gain_begin_drag(move || {
                                let setter = ParamSetter::new(&*context);
                                setter.begin_set_parameter(&params.gain);
                            });
                        }

                        {
                            let params = params.clone();
                            let context = context.clone();
                            component.on_gain_changed(move |value| {
                                let setter = ParamSetter::new(&*context);
                                setter.set_parameter_normalized(&params.gain, value);
                            });
                        }

                        {
                            let params = params.clone();
                            let context = context.clone();
                            component.on_gain_changed_from_string(move |value| {
                                let setter = ParamSetter::new(&*context);
                                let normalized_value =
                                    params.gain.string_to_normalized_value(&value);
                                if let Some(normalized_value) = normalized_value {
                                    setter.set_parameter_normalized(&params.gain, normalized_value);
                                }
                            });
                        }

                        {
                            let params = params.clone();
                            component.on_gain_end_drag(move || {
                                let setter = ParamSetter::new(&*context);
                                setter.end_set_parameter(&params.gain);
                            });
                        }
                    }
                })
                .with_event_loop({
                    let params = self.params.clone();

                    move |window_handler, _setter, _window| {
                        let component = window_handler.component();

                        // Pass the keyboard_input_is_enabled state to the window handler
                        window_handler.set_keyboard_input_is_enabled(
                            component.get_keyboard_input_is_enabled(),
                        );

                        // Plugin -> UI: sync gain knob position
                        component.set_gain_value(params.gain.unmodulated_normalized_value());

                        // Plugin -> UI: format dB string for readout
                        component.set_gain_db(
                            params
                                .gain
                                .normalized_value_to_string(
                                    params.gain.unmodulated_normalized_value(),
                                    true,
                                )
                                .into(),
                        );
                    }
                }),
        ))
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            let gain = self.params.gain.smoothed.next();
            for sample in channel_samples {
                *sample *= gain;
            }
        }
        ProcessStatus::Normal
    }
}

impl ClapPlugin for GainKnob {
    const CLAP_ID: &'static str = "com.ambiosdsp.gain-knob";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A clean single-knob gain plugin");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Utility,
    ];
}

impl Vst3Plugin for GainKnob {
    const VST3_CLASS_ID: [u8; 16] = *b"GainKnobPlugABCD";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

nih_export_clap!(GainKnob);
nih_export_vst3!(GainKnob);
