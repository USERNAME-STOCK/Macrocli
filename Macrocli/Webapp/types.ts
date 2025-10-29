// FIX: Removed self-import of `KeyDefinition` that caused a declaration conflict.
export interface KeyDefinition {
  function: string;
  sequence: string[];
  description: string;
  keypressCount?: number;
  delay?: number;
}

export interface KnobActions {
  press: string[];
  rotateLeft: string[];
  rotateRight: string[];
  description: string;
}

export type KnobId = 'knob1' | 'knob2' | 'knob3';

export interface KnobDefinition {
  knob1: KnobActions;
  knob2: KnobActions;
  knob3: KnobActions;
}

export interface MacroProfile {
  profileName: string;
  description: string;
  keys: { [key: string]: KeyDefinition };
  knobs: KnobDefinition;
  keypressLimit: number;
  totalKeys: number;
  brokenKeys: number[];
}

export interface KeyDetails extends KeyDefinition {
    id: string;
    type: 'key';
}

export interface KnobActionDetails {
    action: 'press' | 'rotateLeft' | 'rotateRight';
    sequence: string[];
    knobDescription: string;
    knobId: string;
    type: 'knob';
}

export type HoveredDetails = KeyDetails | KnobActionDetails | null;
