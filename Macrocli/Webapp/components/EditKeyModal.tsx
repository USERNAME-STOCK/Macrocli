import React, { useState, useEffect, useMemo } from 'react';
import { KeyDefinition } from '../types';
import { CloseIcon } from './icons';
import KeySequenceInput from './KeySequenceInput';
import { isValidKey } from '../data/validKeys';

interface EditKeyModalProps {
    keyNumber: number;
    profileName: string;
    keyData?: KeyDefinition;
    onSave: (keyNumber: number, newKeyData: KeyDefinition) => void;
    onClose: () => void;
    keypressLimit: number;
}

const EditKeyModal: React.FC<EditKeyModalProps> = ({ keyNumber, profileName, keyData, onSave, onClose, keypressLimit }) => {
    const [func, setFunc] = useState('');
    const [desc, setDesc] = useState('');
    const [sequence, setSequence] = useState<string[]>([]);
    const [delay, setDelay] = useState(0);

    const currentKeypressCount = useMemo(() => sequence.length, [sequence]);
    const isOverLimit = useMemo(() => currentKeypressCount > keypressLimit, [currentKeypressCount, keypressLimit]);

    const invalidKeys = useMemo(() => sequence.filter(key => !isValidKey(key)), [sequence]);
    const hasInvalidKeys = useMemo(() => invalidKeys.length > 0, [invalidKeys]);

    useEffect(() => {
        if (keyData) {
            setFunc(keyData.function);
            setDesc(keyData.description);
            setSequence(keyData.sequence || []);
            setDelay(keyData.delay || 0);
        } else {
            setFunc('');
            setDesc('');
            setSequence([]);
            setDelay(0);
        }
    }, [keyData]);

    const handleSave = (e: React.FormEvent) => {
        e.preventDefault();
        if (isOverLimit || hasInvalidKeys) return;
        
        const newKeyData: KeyDefinition = {
            function: func.trim(),
            description: desc.trim(),
            sequence: sequence,
            keypressCount: currentKeypressCount,
            delay: delay > 0 ? delay : undefined,
        };
        onSave(keyNumber, newKeyData);
    };

    return (
        <div 
            className="fixed inset-0 bg-black bg-opacity-70 flex items-center justify-center z-50 animate-fade-in"
            onClick={onClose}
            role="dialog"
            aria-modal="true"
        >
            <div 
                className="bg-zinc-800 rounded-2xl border border-zinc-700 shadow-2xl p-8 w-full max-w-2xl relative"
                onClick={e => e.stopPropagation()}
            >
                <button onClick={onClose} className="absolute top-4 right-4 text-gray-400 hover:text-white transition-colors" aria-label="Close modal">
                    <CloseIcon className="w-6 h-6" />
                </button>
                <h2 className="text-3xl font-bold text-white mb-2">
                    Edit Key <span className="text-green-400">{keyNumber}</span>
                </h2>
                <p className="text-gray-400 mb-6">Profile: {profileName}</p>
                <form onSubmit={handleSave} className="space-y-4">
                    <div>
                        <label htmlFor="function" className="block text-sm font-medium text-gray-300 mb-1">Function Name</label>
                        <input
                            id="function"
                            type="text"
                            value={func}
                            onChange={e => setFunc(e.target.value)}
                            className="w-full bg-zinc-700 border border-zinc-600 text-white rounded-md px-3 py-2 focus:ring-2 focus:ring-green-500 focus:outline-none"
                            placeholder="e.g., Copy Selection"
                            required
                        />
                    </div>
                    <div>
                        <label htmlFor="description" className="block text-sm font-medium text-gray-300 mb-1">Description</label>
                        <textarea
                            id="description"
                            value={desc}
                            onChange={e => setDesc(e.target.value)}
                            className="w-full bg-zinc-700 border border-zinc-600 text-white rounded-md px-3 py-2 h-24 focus:ring-2 focus:ring-green-500 focus:outline-none"
                            placeholder="e.g., Standard copy command (Ctrl+C)"
                            required
                        />
                    </div>
                    <div>
                        <label htmlFor="sequence" className="block text-sm font-medium text-gray-300 mb-1">Key Sequence</label>
                        <KeySequenceInput
                          sequence={sequence}
                          onChange={setSequence}
                          limit={keypressLimit}
                        />
                        <div className="flex justify-between items-center mt-2">
                            <p className={`text-sm font-medium ${isOverLimit ? 'text-red-500' : 'text-gray-400'}`}>
                                Keypress Count: {currentKeypressCount} / {keypressLimit}
                            </p>
                            {hasInvalidKeys && (
                                <p className="text-sm font-medium text-red-500">
                                    Invalid keys: {invalidKeys.join(', ')}
                                </p>
                            )}
                        </div>
                    </div>
                    <div>
                        <label htmlFor="delay" className="block text-sm font-medium text-gray-300 mb-1">Delay (ms)</label>
                        <input
                            id="delay"
                            type="number"
                            min="0"
                            max="1000"
                            step="10"
                            value={delay}
                            onChange={e => setDelay(parseInt(e.target.value, 10) || 0)}
                            className="w-full bg-zinc-700 border border-zinc-600 text-white rounded-md px-3 py-2 focus:ring-2 focus:ring-green-500 focus:outline-none"
                        />
                    </div>
                    <div className="flex justify-end gap-4 pt-4">
                        <button type="button" onClick={onClose} className="px-6 py-2 rounded-md font-semibold text-gray-300 bg-zinc-600 hover:bg-zinc-500 transition-colors">
                            Cancel
                        </button>
                        <button 
                            type="submit" 
                            disabled={isOverLimit || hasInvalidKeys}
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

export default EditKeyModal;
