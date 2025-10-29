
import React from 'react';
import { HoveredDetails } from '../types';

interface DetailsPanelProps {
  details: HoveredDetails;
  profileDescription: string;
}

const SequenceDisplay: React.FC<{ sequence: string[] | string }> = ({ sequence }) => {
    const seqArray = Array.isArray(sequence) ? sequence : [sequence];

    if (seqArray.length === 0) {
        return <div className="text-gray-500 italic">No sequence defined.</div>
    }

    return (
        <div className="flex flex-wrap gap-2">
            {seqArray.map((key, index) => (
                <kbd key={index} className="px-2 py-1.5 text-xs font-semibold text-gray-200 bg-zinc-600 border border-zinc-500 rounded-md">
                    {key}
                </kbd>
            ))}
        </div>
    );
};

const DetailsPanel: React.FC<DetailsPanelProps> = ({ details, profileDescription }) => {
  return (
    <div className="bg-zinc-800 p-6 rounded-2xl border border-zinc-700 shadow-xl w-full lg:w-96 h-full min-h-[500px] flex flex-col">
      <h2 className="text-2xl font-bold text-white mb-4 border-b-2 border-zinc-700 pb-3">Details</h2>
      {details === null ? (
        <div className="flex-grow flex flex-col justify-center text-center">
            <h3 className="text-lg font-semibold text-white mb-2">Profile Overview</h3>
            <p className="text-gray-400 mb-6">{profileDescription}</p>
            <p className="text-gray-500 text-sm">Hover over a key or knob to see specific details here.</p>
        </div>
      ) : (
        <div className="flex-grow animate-fade-in">
          {details.type === 'key' && (
            <>
              <h3 className="text-xl font-bold text-green-400 mb-2">{details.id}: <span className="text-white">{details.function}</span></h3>
              <p className="text-gray-400 mb-4 text-sm">{details.description}</p>
              <h4 className="font-semibold text-gray-200 mb-2">Key Sequence:</h4>
              <SequenceDisplay sequence={details.sequence} />
              {details.keypressCount && (
                <p className="text-sm text-gray-500 mt-4">Keypress Count: {details.keypressCount}</p>
              )}
            </>
          )}
          {details.type === 'knob' && (
             <>
              <h3 className="text-xl font-bold text-green-400 mb-2">{details.knobId}: <span className="text-white capitalize">{details.action.replace('rotate','Rotate ')}</span></h3>
              <p className="text-gray-400 mb-4 text-sm">{details.knobDescription}</p>
              <h4 className="font-semibold text-gray-200 mb-2">Action:</h4>
              <SequenceDisplay sequence={details.sequence} />
            </>
          )}
        </div>
      )}
    </div>
  );
};

export default DetailsPanel;
