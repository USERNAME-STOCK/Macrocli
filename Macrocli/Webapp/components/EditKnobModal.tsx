import React, { useState, useEffect, useMemo } from 'react';
import { KnobActions, KnobId } from '../types';
import { CloseIcon } from './icons';
import KeySequenceInput from './KeySequenceInput';
import { isValidKey } from '../data/validKeys';

interface EditKnobModalProps {
    knobId: KnobId;
    profileName: string;
    knobData: KnobActions;
    onSave: (knobId: KnobId, newKnobData: KnobActions) => void;
    onClose: () => void;
    keypressLimit: number;
}

const EditKnobModal: React.FC<EditKnobModalProps> = ({ knobId, profileName, knobData, onSave, onClose, keypressLimit }) => {
    const [description, setDescription] = useState('');
    const [pressSequence, setPressSequence] = useState<string[]>([]);
    const [rotateLeftSequence, setRotateLeftSequence] = useState<string[]>([]);
    const [rotateRightSequence, setRotateRightSequence] = useState<string[]>([]);

    useEffect(() => {
        if (knobData) {
            setDescription(knobData.description || '');
            setPressSequence(knobData.press || []);
            setRotateLeftSequence(knobData.rotateLeft || []);
            setRotateRightSequence(knobData.rotateRight || []);
        }
    }, [knobData]);

    const isOverLimit = useMemo(() => 
        pressSequence.length > keypressLimit ||
        rotateLeftSequence.length > keypressLimit ||
        rotateRightSequence.length > keypressLimit,
    [pressSequence, rotateLeftSequence, rotateRightSequence, keypressLimit]);

    const validation = useMemo(() => {
        const invalid = {
            press: pressSequence.filter(k => !isValidKey(k)),
            rotateLeft: rotateLeftSequence.filter(k => !isValidKey(k)),
            rotateRight: rotateRightSequence.filter(k => !isValidKey(k)),
        };
        const allInvalid = [...invalid.press, ...invalid.rotateLeft, ...invalid.rotateRight];
        const uniqueInvalid = Array.from(new Set(allInvalid));
        return {
            hasInvalid: uniqueInvalid.length > 0,
            allInvalid: uniqueInvalid,
        };
    }, [pressSequence, rotateLeftSequence, rotateRightSequence]);

    const handleSave = (e: React.FormEvent) => {
        e.preventDefault();
        if (isOverLimit || validation.hasInvalid) return;
        
        const newKnobData: KnobActions = {
            description: description.trim(),
            press: pressSequence,
            rotateLeft: rotateLeftSequence,
            rotateRight: rotateRightSequence,
        };
        onSave(knobId, newKnobData);
    };

    const knobNumber = knobId.slice(-1);

    return (
        <div 
            className="fixed inset-0 bg-black bg-opacity-70 flex items-center justify-center z-50 animate-fade-in"
            onClick={onClose}
            role="dialog"
            aria-modal="true"
        >
            <div 
                className="bg-zinc-800 rounded-2xl border border-zinc-700 shadow-2xl p-8 w-full max-w-3xl relative"
                onClick={e => e.stopPropagation()}
            >
                <button onClick={onClose} className="absolute top-4 right-4 text-gray-400 hover:text-white transition-colors" aria-label="Close modal">
                    <CloseIcon className="w-6 h-6" />
                </button>
                <h2 className="text-3xl font-bold text-white mb-2">
                    Edit Knob <span className="text-green-400">{knobNumber}</span>
                </h2>
                <p className="text-gray-400 mb-6">Profile: {profileName}</p>
                <form onSubmit={handleSave} className="space-y-4 max-h-[70vh] overflow-y-auto pr-4">
                    <div>
                        <label htmlFor="description" className="block text-sm font-medium text-gray-300 mb-1">Description</label>
                        <textarea
                            id="description"
                            value={description}
                            onChange={e => setDescription(e.target.value)}
                            className="w-full bg-zinc-700 border border-zinc-600 text-white rounded-md px-3 py-2 h-24 focus:ring-2 focus:ring-green-500 focus:outline-none"
                            placeholder="e.g., Volume and Mute Control"
                            required
                        />
                    </div>
                    
                    <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                        {/* Rotate Left */}
                        <div>
                            <label htmlFor="rotateLeft" className="block text-sm font-medium text-gray-300 mb-1">Rotate Left Sequence</label>
                            <KeySequenceInput
                              sequence={rotateLeftSequence}
                              onChange={setRotateLeftSequence}
                              limit={keypressLimit}
                            />
                             <p className={`text-sm mt-2 font-medium ${rotateLeftSequence.length > keypressLimit ? 'text-red-500' : 'text-gray-400'}`}>
                                Keypress Count: {rotateLeftSequence.length} / {keypressLimit}
                            </p>
                        </div>
                        {/* Press */}
                        <div>
                            <label htmlFor="press" className="block text-sm font-medium text-gray-300 mb-1">Press Sequence</label>
                            <KeySequenceInput
                              sequence={pressSequence}
                              onChange={setPressSequence}
                              limit={keypressLimit}
                            />
                             <p className={`text-sm mt-2 font-medium ${pressSequence.length > keypressLimit ? 'text-red-500' : 'text-gray-400'}`}>
                                Keypress Count: {pressSequence.length} / {keypressLimit}
                            </p>
                        </div>
                        {/* Rotate Right */}
                        <div>
                            <label htmlFor="rotateRight" className="block text-sm font-medium text-gray-300 mb-1">Rotate Right Sequence</label>
                            <KeySequenceInput
                              sequence={rotateRightSequence}
                              onChange={setRotateRightSequence}
                              limit={keypressLimit}
                            />
                             <p className={`text-sm mt-2 font-medium ${rotateRightSequence.length > keypressLimit ? 'text-red-500' : 'text-gray-400'}`}>
                                Keypress Count: {rotateRightSequence.length} / {keypressLimit}
                            </p>
                        </div>
                    </div>

                    {validation.hasInvalid && (
                        <div className="mt-4 p-3 bg-red-900/50 border border-red-500/50 rounded-md">
                            <p className="text-sm font-medium text-red-300">
                                Found invalid keys in one or more sequences:
                                <span className="font-mono ml-2 text-red-200">{validation.allInvalid.join(', ')}</span>
                            </p>
                            <p className="text-xs text-red-400 mt-1">Invalid keys are highlighted with a red border. Please correct them before saving.</p>
                        </div>
                    )}

                    <div className="flex justify-end gap-4 pt-4">
                        <button type="button" onClick={onClose} className="px-6 py-2 rounded-md font-semibold text-gray-300 bg-zinc-600 hover:bg-zinc-500 transition-colors">
                            Cancel
                        </button>
                        <button 
                            type="submit" 
                            disabled={isOverLimit || validation.hasInvalid}
                            className="px-6 py-2 rounded-md font-semibold text-white bg-green-600 hover:bg-green-500 transition-colors disabled:bg-gray-500 disabled:cursor-not-allowed"
                        >
                            Save Changes
                        </button>
                    </div>
                </form>
            </div>
        </div>
    );
};

export default EditKnobModal;
