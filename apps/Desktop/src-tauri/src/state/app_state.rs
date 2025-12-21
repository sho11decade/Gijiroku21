use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::error::{AppError, AppResult};

/// NPU検出結果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpuInfo {
    pub available: bool,
    pub device_name: Option<String>,
    pub driver_version: Option<String>,
}

/// アプリケーション設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// NPU使用設定
    pub use_npu: bool,
    /// 音声認識モデルサイズ (small, medium, large)
    pub asr_model_size: String,
    /// LLM使用設定
    pub use_llm: bool,
    /// 自動保存設定
    pub auto_save: bool,
    /// 保存先ディレクトリ
    pub save_directory: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            use_npu: true,
            asr_model_size: "small".to_string(),
            use_llm: true,
            auto_save: true,
            save_directory: None,
        }
    }
}

impl Settings {
    /// 設定ファイルパスを取得
    pub fn config_path() -> AppResult<std::path::PathBuf> {
        let config_dir = directories::ProjectDirs::from("com", "gijiroku21", "Gijiroku21")
            .ok_or_else(|| AppError::Config("Could not determine config directory".to_string()))?;
        
        let config_dir = config_dir.config_dir();
        std::fs::create_dir_all(config_dir)?;
        
        Ok(config_dir.join("settings.json"))
    }

    /// 設定を読み込み
    pub fn load() -> AppResult<Self> {
        let path = Self::config_path()?;
        
        if !path.exists() {
            let settings = Self::default();
            settings.save()?;
            return Ok(settings);
        }

        let content = std::fs::read_to_string(path)?;
        let settings: Settings = serde_json::from_str(&content)?;
        Ok(settings)
    }

    /// 設定を保存
    pub fn save(&self) -> AppResult<()> {
        let path = Self::config_path()?;
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

/// アプリケーション全体の状態
#[derive(Clone)]
pub struct AppState {
    /// NPU情報
    pub npu_info: Arc<RwLock<Option<NpuInfo>>>,
    /// アプリケーション設定
    pub settings: Arc<RwLock<Settings>>,
}

impl AppState {
    pub fn new() -> Self {
        let settings = Settings::load().unwrap_or_default();
        
        AppState {
            npu_info: Arc::new(RwLock::new(None)),
            settings: Arc::new(RwLock::new(settings)),
        }
    }

    /// NPU情報を初期化
    pub async fn initialize_npu(&self) -> AppResult<()> {
        // Windows での DirectML 存在チェックに基づく簡易検出
        #[cfg(target_os = "windows")]
        let npu_info = {
            use std::path::Path;

            // 環境変数 SystemRoot を優先してディレクトリを組み立て
            let system_root = std::env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".to_string());
            let candidates = [
                format!("{}\\System32\\DirectML.dll", system_root),
                format!("{}\\SysWOW64\\DirectML.dll", system_root),
            ];

            let found = candidates.iter().any(|p| Path::new(p).exists());

            if found {
                NpuInfo {
                    available: true,
                    device_name: Some("DirectML".to_string()),
                    driver_version: None,
                }
            } else {
                NpuInfo {
                    available: false,
                    device_name: None,
                    driver_version: None,
                }
            }
        };

        // Windows 以外は現時点では未対応のため CPU 扱い
        #[cfg(not(target_os = "windows"))]
        let npu_info = NpuInfo {
            available: false,
            device_name: None,
            driver_version: None,
        };
        
        let mut npu = self.npu_info.write().await;
        *npu = Some(npu_info);
        
        Ok(())
    }

    /// 設定を取得
    pub async fn get_settings(&self) -> Settings {
        self.settings.read().await.clone()
    }

    /// 設定を更新
    pub async fn update_settings(&self, new_settings: Settings) -> AppResult<()> {
        new_settings.save()?;
        
        let mut settings = self.settings.write().await;
        *settings = new_settings;
        
        Ok(())
    }

    /// NPU情報を取得
    pub async fn get_npu_info(&self) -> Option<NpuInfo> {
        self.npu_info.read().await.clone()
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
