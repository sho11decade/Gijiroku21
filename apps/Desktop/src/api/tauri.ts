// Tauri APIの型定義
import { invoke } from "@tauri-apps/api/core";

export interface NpuInfo {
  available: boolean;
  device_name: string | null;
  driver_version: string | null;
}

export interface Settings {
  use_npu: boolean;
  asr_model_size: string;
  use_llm: boolean;
  auto_save: boolean;
  save_directory: string | null;
  model_directory?: string | null;
  tokenizer_directory?: string | null;
}

export interface SystemInfo {
  npu_info: NpuInfo | null;
  os: string;
  arch: string;
  app_version: string;
}

// システム情報API
export async function getSystemInfo(): Promise<SystemInfo> {
  return await invoke<SystemInfo>("get_system_info");
}

export async function getSettings(): Promise<Settings> {
  return await invoke<Settings>("get_settings");
}

export async function updateSettings(settings: Settings): Promise<void> {
  await invoke("update_settings", { settings });
}

export async function getNpuInfo(): Promise<NpuInfo | null> {
  return await invoke<NpuInfo | null>("get_npu_info");
}

export async function detectNpu(): Promise<NpuInfo> {
  return await invoke<NpuInfo>("detect_npu");
}

// モデル存在チェック
export interface ModelCheck {
  ok: boolean;
  model_dir: string;
  tokenizer_dir: string;
  required: string[];
  missing: string[];
}

export async function checkModels(): Promise<ModelCheck> {
  return await invoke<ModelCheck>("check_models");
}

// モデルダウンロード
export interface DownloadResult {
  ok: boolean;
  downloaded: string[];
  failed: string[];
}

export async function downloadModels(): Promise<DownloadResult> {
  return await invoke<DownloadResult>("download_models");
}

// 録音状態
export interface RecordingStatus {
  status: "idle" | "recording" | "paused" | "processing";
  meeting_id?: string;
}

// 録音を開始
export async function startRecording(title: string): Promise<string> {
  return await invoke<string>("start_recording", { title });
}

// 録音を停止
export async function stopRecording(): Promise<void> {
  await invoke("stop_recording");
}

// 録音を一時停止
export async function pauseRecording(): Promise<void> {
  await invoke("pause_recording");
}

// 録音を再開
export async function resumeRecording(): Promise<void> {
  await invoke("resume_recording");
}

// 録音状態を取得
export async function getRecordingStatus(): Promise<RecordingStatus> {
  const statusJson = await invoke<string>("get_recording_status");
  return JSON.parse(statusJson) as RecordingStatus;
}

// 音声デバイス一覧を取得
export async function listAudioDevices(): Promise<string[]> {
  return await invoke<string[]>("list_audio_devices");
}
