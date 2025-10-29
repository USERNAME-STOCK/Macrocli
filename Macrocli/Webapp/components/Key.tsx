
import React from 'react';
import { KeyDefinition, HoveredDetails } from '../types';

interface KeyProps {
  keyNumber: number;
  data?: KeyDefinition;
  isBroken: boolean;
  onHover: (details: HoveredDetails) => void;
  onKeyClick: (keyNumber: number) => void;
}

const Key: React.FC<KeyProps> = ({ keyNumber, data, isBroken, onHover, onKeyClick }) => {
  const handleMouseEnter = () => {
    if (data && !isBroken) {
      onHover({ ...data, id: `Key ${keyNumber}`, type: 'key' });
    } else {
      onHover(null);
    }
  };

  const handleClick = () => {
    if (!isBroken) {
      onKeyClick(keyNumber);
    }
  };

  const baseStyle = "relative w-28 h-28 rounded-xl flex flex-col items-center justify-center p-2 text-center transition-all duration-200";
  
  const activeStyle = "bg-zinc-700 hover:bg-zinc-600 hover:shadow-lg hover:shadow-green-500/20 hover:-translate-y-1 cursor-pointer";
  const brokenStyle = "bg-zinc-800 opacity-50 cursor-not-allowed";

  return (
    <div
      className={`${baseStyle} ${isBroken ? brokenStyle : activeStyle}`}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={() => onHover(null)}
      onClick={handleClick}
      role="button"
      aria-label={`Edit Key ${keyNumber}`}
    >
      <span className="absolute top-2 left-3 text-2xl font-bold text-green-400">
        {keyNumber}
      </span>
      <div className="mt-4">
        {isBroken ? (
            <span className="text-gray-500 text-sm font-semibold">BROKEN</span>
        ) : (
          data ? (
            <span className="text-gray-200 text-sm font-semibold leading-tight">{data.function}</span>
          ) : (
            <span className="text-gray-500 text-sm font-semibold">UNASSIGNED</span>
          )
        )}
      </div>
    </div>
  );
};

export default Key;
