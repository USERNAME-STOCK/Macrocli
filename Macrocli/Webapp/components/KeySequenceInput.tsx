import React, {
  useState,
  useMemo,
  KeyboardEvent,
  useRef,
  useEffect,
  ClipboardEvent,
} from "react";
import { CloseIcon } from "./icons";
import { isValidKey, generateKeySuggestions } from "../data/validKeys";

const KEY_SUGGESTIONS = generateKeySuggestions();

interface KeySequenceInputProps {
  sequence: string[];
  onChange: (newSequence: string[]) => void;
  limit: number;
}

const KeySequenceInput: React.FC<KeySequenceInputProps> = ({
  sequence,
  onChange,
  limit,
}) => {
  const [inputValue, setInputValue] = useState("");
  const [suggestions, setSuggestions] = useState<string[]>([]);
  const [activeIndex, setActiveIndex] = useState(0);
  const inputRef = useRef<HTMLInputElement>(null);

  const isOverLimit = useMemo(
    () => sequence.length >= limit,
    [sequence, limit]
  );

  useEffect(() => {
    if (inputValue) {
      const filtered = KEY_SUGGESTIONS.filter((k) =>
        k.startsWith(inputValue.toLowerCase())
      ).slice(0, 5);
      setSuggestions(filtered);
      setActiveIndex(0);
    } else {
      setSuggestions([]);
    }
  }, [inputValue]);

  const addKey = (key: string) => {
    const trimmedKey = key.trim();
    if (trimmedKey && !isOverLimit) {
      onChange([...sequence, trimmedKey]);
      setInputValue("");
      setSuggestions([]);
    }
  };

  const addMultipleKeys = (keys: string[]) => {
    const keysToAdd = keys
      .filter((k) => k.trim())
      .slice(0, limit - sequence.length);
    if (keysToAdd.length > 0) {
      onChange([...sequence, ...keysToAdd]);
    }
    setInputValue("");
    setSuggestions([]);
  };

  const removeKey = (indexToRemove: number) => {
    onChange(sequence.filter((_, index) => index !== indexToRemove));
  };

  const handleKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter" || e.key === "Tab") {
      e.preventDefault();
      const keyToAdd = suggestions[activeIndex] || inputValue;
      if (keyToAdd) addKey(keyToAdd);
    } else if (e.key === "," || e.key === " ") {
      if (inputValue) {
        e.preventDefault();
        addKey(inputValue);
      }
    } else if (
      e.key === "Backspace" &&
      inputValue === "" &&
      sequence.length > 0
    ) {
      removeKey(sequence.length - 1);
    } else if (e.key === "ArrowDown") {
      e.preventDefault();
      setActiveIndex((prev) => (prev + 1) % suggestions.length);
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      setActiveIndex(
        (prev) => (prev - 1 + suggestions.length) % suggestions.length
      );
    }
  };

  const handlePaste = (e: ClipboardEvent<HTMLInputElement>) => {
    const pastedText = e.clipboardData.getData("text");
    const keys = pastedText
      .split(/[,\s]+/)
      .map((k) => k.trim())
      .filter(Boolean);
    if (keys.length > 1) {
      e.preventDefault();
      addMultipleKeys(keys);
    }
  };

  return (
    <div className="relative">
      <div
        className="flex flex-wrap items-center gap-2 p-2 rounded-md bg-zinc-700 border border-zinc-600 focus-within:ring-2 focus-within:ring-green-500"
        onClick={() => inputRef.current?.focus()}
      >
        {sequence.map((key, index) => {
          const isValid = isValidKey(key);
          return (
            <span
              key={index}
              className={`flex items-center gap-1.5 bg-zinc-600 text-gray-200 text-sm font-semibold px-2 py-1 rounded ${
                !isValid ? "ring-2 ring-red-500" : ""
              }`}
            >
              {key}
              <button
                type="button"
                onClick={() => removeKey(index)}
                className="text-gray-400 hover:text-white"
                aria-label={`Remove ${key}`}
              >
                <CloseIcon className="w-3 h-3" />
              </button>
            </span>
          );
        })}
        <input
          ref={inputRef}
          type="text"
          value={inputValue}
          onChange={(e) => setInputValue(e.target.value)}
          onKeyDown={handleKeyDown}
          onPaste={handlePaste}
          className="flex-grow bg-transparent focus:outline-none text-white p-1 min-w-[100px]"
          placeholder={isOverLimit ? "Limit reached" : "Type a key..."}
          disabled={isOverLimit}
          aria-label="Key sequence input"
        />
      </div>
      {suggestions.length > 0 && (
        <ul className="absolute z-10 w-full bg-zinc-600 border border-zinc-500 rounded-md mt-1 max-h-48 overflow-y-auto shadow-lg">
          {suggestions.map((suggestion, index) => (
            <li
              key={suggestion}
              onClick={() => addKey(suggestion)}
              onMouseEnter={() => setActiveIndex(index)}
              className={`px-3 py-2 cursor-pointer text-white ${
                index === activeIndex ? "bg-green-600" : "hover:bg-zinc-500"
              }`}
            >
              {suggestion}
            </li>
          ))}
        </ul>
      )}
    </div>
  );
};

export default KeySequenceInput;
