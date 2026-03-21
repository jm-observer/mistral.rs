use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CliConfig {
    pub command: String,
    pub models: Vec<ModelEntry>,
}

#[derive(Deserialize, Debug)]
pub struct ModelEntry {
    pub kind: String,
    pub model_id: String,
    #[serde(default)]
    pub quantization: QuantizationOptions,
}

#[derive(Deserialize, Debug, Default)]
pub struct QuantizationOptions {
    pub in_situ_quant: Option<String>,
}

fn main() {
    let toml_str = r#"
command = "run"

[[models]]
kind = "auto"
model_id = "gpt-oss-120b"

[models.quantization]
in_situ_quant = "fp4"
"#;

    let config: Result<CliConfig, _> = toml::from_str(toml_str);
    println!("{:#?}", config);
}
