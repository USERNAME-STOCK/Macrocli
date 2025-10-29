
import React from 'react';
import { KnobActions, HoveredDetails, KnobId } from '../types';
import { RotateLeftIcon, RotateRightIcon, DownloadIcon } from './icons';

interface KnobProps {
  knobId: KnobId;
  data: KnobActions;
  onHover: (details: HoveredDetails) => void;
  onKnobClick: (knobId: KnobId) => void;
}

const Knob: React.FC<KnobProps> = ({ knobId, data, onHover, onKnobClick }) => {
  const knobName = `Knob ${knobId.slice(-1)}`;
  
  const handleMouseEnter = (action: 'press' | 'rotateLeft' | 'rotateRight') => {
    onHover({
      type: 'knob',
      action,
      sequence: data[action],
      knobDescription: data.description,
      knobId: knobName,
    });
  };

  const handleMouseLeave = () => onHover(null);
  
  const getActionDisplay = (action: 'press' | 'rotateLeft' | 'rotateRight'): string => {
    const sequence = data[action];
    if (!sequence || sequence.length === 0) return 'UNASSIGNED';
    if (sequence.length > 2) return `${sequence.slice(0, 2).join(', ')}...`;
    return sequence.join(', ');
  }

  return (
    <div className="flex items-center justify-center gap-2 my-2 w-full">
      {/* Rotate Left */}
      <div
        className="flex flex-col items-center text-center p-2 rounded-lg cursor-pointer hover:bg-zinc-700 transition-colors"
        onMouseEnter={() => handleMouseEnter('rotateLeft')}
        onMouseLeave={handleMouseLeave}
      >
        <RotateLeftIcon className="w-8 h-8 text-gray-400" />
        <span className="text-xs mt-1 truncate max-w-[70px]">{getActionDisplay('rotateLeft')}</span>
      </div>

      {/* Press */}
      <div
        className="group relative w-28 h-28 rounded-full flex flex-col items-center justify-center bg-zinc-700 hover:bg-zinc-600 cursor-pointer transition-all duration-200 hover:shadow-lg hover:shadow-green-500/20 hover:-translate-y-1"
        onMouseEnter={() => handleMouseEnter('press')}
        onMouseLeave={handleMouseLeave}
        onClick={() => onKnobClick(knobId)}
        role="button"
        aria-label={`Edit ${knobName}`}
      >
        <DownloadIcon className="w-10 h-10 text-gray-300" />
        <span className="text-sm font-semibold mt-1 px-2 text-center">{getActionDisplay('press')}</span>
         <div className="absolute inset-0 bg-black bg-opacity-0 group-hover:bg-opacity-40 transition-all duration-300 flex items-center justify-center rounded-full">
            <span className="text-white text-sm font-bold opacity-0 group-hover:opacity-100 transition-opacity">EDIT</span>
        </div>
      </div>

      {/* Rotate Right */}
      <div
        className="flex flex-col items-center text-center p-2 rounded-lg cursor-pointer hover:bg-zinc-700 transition-colors"
        onMouseEnter={() => handleMouseEnter('rotateRight')}
        onMouseLeave={handleMouseLeave}
      >
        <RotateRightIcon className="w-8 h-8 text-gray-400" />
        <span className="text-xs mt-1 truncate max-w-[70px]">{getActionDisplay('rotateRight')}</span>
      </div>
    </div>
  );
};

export default Knob;
