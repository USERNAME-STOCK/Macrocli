import { MacroProfile } from '../types';

export const VALID_MODIFIERS: Set<string> = new Set([
  'ctrl', 'shift', 'alt', 'opt', 'win', 'cmd',
  'rctrl', 'rshift', 'ralt', 'ropt', 'rwin', 'rcmd'
]);

// All non-modifier keys
export const VALID_KEYS: Set<string> = new Set([
  // Letters
  'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
  'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
  // Numbers
  '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
  // Standard Keys
  'enter', 'escape', 'backspace', 'tab', 'space', 'minus', 'equal',
  'leftbracket', 'rightbracket', 'backslash', 'nonushash', 'semicolon',
  'quote', 'grave', 'comma', 'dot', 'slash', 'capslock',
  // Function Keys
  'f1', 'f2', 'f3', 'f4', 'f5', 'f6', 'f7', 'f8', 'f9', 'f10', 'f11', 'f12',
  'f13', 'f14', 'f15', 'f16', 'f17', 'f18', 'f19', 'f20', 'f21', 'f22', 'f23', 'f24',
  // Control Keys
  'printscreen', 'scrolllock', 'pause', 'insert', 'home', 'pageup',
  'delete', 'end', 'pagedown', 'right', 'left', 'down', 'up',
  // Numpad
  'numlock', 'numpadslash', 'numpadasterisk', 'numpadminus', 'numpadplus',
  'numpadenter', 'numpad1', 'numpad2', 'numpad3', 'numpad4', 'numpad5',
  'numpad6', 'numpad7', 'numpad8', 'numpad9', 'numpad0', 'numpaddot',
  'nonusbackslash', 'application', 'power', 'numpadequal',
  // Media Keys
  'next', 'previous', 'prev', 'stop', 'play', 'mute', 'volumeup', 'volumedown',
  'favorites', 'calculator', 'screenlock', 'screenbrightnessup', 'screenbrightnessdown',
  'webpagehome', 'webpageback', 'webpageforward',
  // Mouse Actions
  'wheeldown', 'wheelup', 'click', 'rclick', 'mclick'
]);

/**
 * Validates a single keypress string.
 * A valid key can be:
 * 1. A single key from VALID_KEYS or VALID_MODIFIERS.
 * 2. A combination of unique modifiers from VALID_MODIFIERS, followed by a key from VALID_KEYS, separated by '-'. (e.g., "ctrl-shift-c")
 * 3. A custom key code in the format "<ddd>" where d is a digit. (e.g., "<110>")
 */
export function isValidKey(key: string): boolean {
  if (typeof key !== 'string' || !key) return false;

  const lowerKey = key.toLowerCase();

  // Rule 3: Custom key code
  if (lowerKey.match(/^<\d+>$/)) {
    return true;
  }

  const parts = lowerKey.split('-');

  // Rule 1: Single key
  if (parts.length === 1) {
    return VALID_KEYS.has(lowerKey) || VALID_MODIFIERS.has(lowerKey);
  }

  // Rule 2: Modifier combination
  const mainKey = parts[parts.length - 1];
  if (!VALID_KEYS.has(mainKey)) {
    return false; // The last part must be a valid, non-modifier key.
  }

  const modifiers = parts.slice(0, -1);
  const uniqueModifiers = new Set(modifiers);
  if (uniqueModifiers.size !== modifiers.length) {
    return false; // Duplicate modifiers are not allowed (e.g., "ctrl-ctrl-c")
  }

  return modifiers.every(mod => VALID_MODIFIERS.has(mod));
}


export const generateKeySuggestions = (): string[] => {
    const suggestions = new Set<string>();

    VALID_KEYS.forEach(k => suggestions.add(k));
    VALID_MODIFIERS.forEach(m => suggestions.add(m));

    // Common Modifier Combinations
    const MODIFIERS_FOR_COMBO = ['ctrl', 'alt', 'shift'];
    const COMMON_KEYS_FOR_COMBO = [
        ...'abcdefghijklmnopqrstuvwxyz0123456789'.split(''),
        'tab', 'enter', 'semicolon', 'minus', 'equal', 'space', 'slash',
        'leftbracket', 'rightbracket', 'home', 'end',
    ];
    
    MODIFIERS_FOR_COMBO.forEach(mod => {
        COMMON_KEYS_FOR_COMBO.forEach(key => {
            suggestions.add(`${mod}-${key}`);
        });
    });

    // Add common double-modifier combos
    suggestions.add('ctrl-shift-c');
    suggestions.add('ctrl-shift-v');
    suggestions.add('ctrl-shift-z');
    suggestions.add('ctrl-shift-tab');
    suggestions.add('ctrl-shift-enter');
    suggestions.add('ctrl-shift-escape');

    return Array.from(suggestions).sort();
};
