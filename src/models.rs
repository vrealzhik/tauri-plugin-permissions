use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PingRequest {
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PingResponse {
    pub value: Option<String>,
}

/// Типы разрешений
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PermissionType {
    Location,
    Notifications,
    BluetoothScan,
    BluetoothConnect,
}

impl PermissionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            PermissionType::Location => "location",
            PermissionType::Notifications => "notifications",
            PermissionType::BluetoothScan => "bluetoothScan",
            PermissionType::BluetoothConnect => "bluetoothConnect",
        }
    }
}

/// Статус разрешения
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PermissionStatus {
    Granted,
    Denied,
    Prompt,
    PromptWithRationale,
    Limited,
    NotDetermined,
}

/// Результат для одного разрешения
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SinglePermissionResult {
    pub permission: PermissionType,
    pub status: PermissionStatus,
    pub granted: bool,
}

/// Результат для нескольких разрешений
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionsResult {
    pub permissions: Vec<SinglePermissionResult>,
    pub all_granted: bool,
}

/// Запрос на разрешения
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionRequest {
    pub permissions: Vec<PermissionType>,
}

/// Ответ от проверки разрешения
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckPermissionResponse {
    pub status: PermissionStatus,
    pub granted: bool,
}

/// Ответ от запроса разрешения
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestPermissionResponse {
    pub status: PermissionStatus,
    pub granted: bool,
    pub requested: bool,
}