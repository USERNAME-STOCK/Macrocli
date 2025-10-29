

import React from 'react';
import { MacroProfile, HoveredDetails, KnobId } from '../types';
import Key from './Key';
import Knob from './Knob';

interface MacroKeyboardProps {
  profile: MacroProfile;
  onHover: (details: HoveredDetails) => void;
  onKeyClick: (keyNumber: number) => void;
  onKnobClick: (knobId: KnobId) => void;
}

const MacroKeyboard: React.FC<MacroKeyboardProps> = ({ profile, onHover, onKeyClick, onKnobClick }) => {
  const keyNumbers = Array.from({ length: 12 }, (_, i) => i + 1);

  return (
    <div className="bg-zinc-800 p-6 rounded-2xl border border-zinc-700 shadow-2xl flex gap-6">
      <div className="grid grid-cols-4 gap-4">
        {keyNumbers.map((num) => (
          <Key
            key={num}
            keyNumber={num}
            data={profile.keys[num]}
            isBroken={profile.brokenKeys.includes(num)}
            onHover={onHover}
            onKeyClick={onKeyClick}
          />
        ))}
      </div>
      <div className="flex flex-col justify-around items-center border-l-2 border-zinc-700 pl-6">
        {(Object.keys(profile.knobs) as KnobId[]).map((knobId) => (
            <Knob key={knobId} knobId={knobId} data={profile.knobs[knobId]} onHover={onHover} onKnobClick={onKnobClick} />
        ))}
      </div>
    </div>
  );
};

export default MacroKeyboard;
