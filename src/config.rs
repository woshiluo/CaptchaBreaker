use std::env;
use std::path::PathBuf;
use std::sync::Mutex;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct GlobalConfig {
    root_path: PathBuf,
    pub models_config: ModelsConfig,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        let root_path = env::current_dir().unwrap();
        let models_config = ModelsConfig::new(root_path.join("models"));
        GlobalConfig {
            root_path,
            models_config,
        }
    }
}

impl GlobalConfig {
    pub fn instance() -> &'static GlobalConfig {
        static INSTANCE: Lazy<GlobalConfig> = Lazy::new(|| GlobalConfig::default());
        &INSTANCE
    }
}

/// 模型配置
/// # Fields
/// - `file_path`: 模型文件的保存路径
/// - `download_url`: 模型文件的下载链接
#[derive(Debug)]
pub struct ModelConfig {
    pub file_path: String,
    pub download_url: String,
}

impl ModelConfig {
    fn new(file_path: &str, download_url: &str) -> ModelConfig {
        ModelConfig {
            file_path: file_path.to_string(),
            download_url: download_url.to_string()
        }
    }
}

#[derive(Debug)]
pub struct ModelsConfig {
    pub yolo11n: ModelConfig,
    pub siamese: ModelConfig,
}

impl ModelsConfig {
    fn new(models_dir: PathBuf) -> Self {
        ModelsConfig {
            yolo11n: ModelConfig::new(
                models_dir.join("yolov11n_captcha.onnx").to_str().unwrap(),
                "https://www.modelscope.cn/models/Amorter/CaptchaBreakerModels/resolve/master/yolov11n_captcha.onnx",
            ),
            siamese: ModelConfig::new(
                models_dir.join("siamese.onnx").to_str().unwrap(),
                "https://www.modelscope.cn/models/Amorter/CaptchaBreakerModels/resolve/master/siamese.onnx",
            ),
        }
    }
}