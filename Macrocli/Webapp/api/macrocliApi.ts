/**
 * API client for communicating with the Macrocli Rust backend
 */

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080/api';

export interface ApiResponse<T> {
  success: boolean;
  data: T | null;
  error: string | null;
}

export interface KeysResponse {
  modifiers: string[];
  keys: string[];
  media_keys: string[];
  mouse_actions: string[];
  custom_key_syntax: string;
}

export interface DeviceInfo {
  vendor_id: number;
  product_id: number;
  connected: boolean;
}

export interface ValidateRequest {
  config_json: string;
  product_id?: number;
  device_connected: boolean;
}

export interface ProgramRequest {
  config_json: string;
}

export interface LedRequest {
  index: number;
  layer: number;
  color?: string;
}

export interface ReadQuery {
  layer?: number;
}

/**
 * Get all supported keys and modifiers
 */
export async function getKeys(): Promise<KeysResponse> {
  const response = await fetch(`${API_BASE_URL}/keys`);
  const result: ApiResponse<KeysResponse> = await response.json();

  if (!result.success || !result.data) {
    throw new Error(result.error || 'Failed to get keys');
  }

  return result.data;
}

/**
 * Check if a device is connected
 */
export async function getDevice(): Promise<DeviceInfo> {
  const response = await fetch(`${API_BASE_URL}/device`);
  const result: ApiResponse<DeviceInfo> = await response.json();

  if (!result.success || !result.data) {
    throw new Error(result.error || 'Failed to get device info');
  }

  return result.data;
}

/**
 * Validate a configuration
 */
export async function validateConfig(request: ValidateRequest): Promise<string> {
  const response = await fetch(`${API_BASE_URL}/validate`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(request),
  });

  const result: ApiResponse<string> = await response.json();

  if (!result.success || !result.data) {
    throw new Error(result.error || 'Validation failed');
  }

  return result.data;
}

/**
 * Program the device with a configuration
 */
export async function programDevice(request: ProgramRequest): Promise<string> {
  const response = await fetch(`${API_BASE_URL}/program`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(request),
  });

  const result: ApiResponse<string> = await response.json();

  if (!result.success || !result.data) {
    throw new Error(result.error || 'Programming failed');
  }

  return result.data;
}

/**
 * Read configuration from the device
 */
export async function readConfig(query?: ReadQuery): Promise<string> {
  const params = new URLSearchParams();
  if (query?.layer !== undefined) {
    params.append('layer', query.layer.toString());
  }

  const url = `${API_BASE_URL}/read${params.toString() ? '?' + params.toString() : ''}`;
  const response = await fetch(url);
  const result: ApiResponse<string> = await response.json();

  if (!result.success || !result.data) {
    throw new Error(result.error || 'Failed to read config');
  }

  return result.data;
}

/**
 * Set LED color on the device
 */
export async function setLed(request: LedRequest): Promise<string> {
  const response = await fetch(`${API_BASE_URL}/led`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(request),
  });

  const result: ApiResponse<string> = await response.json();

  if (!result.success || !result.data) {
    throw new Error(result.error || 'Failed to set LED');
  }

  return result.data;
}
