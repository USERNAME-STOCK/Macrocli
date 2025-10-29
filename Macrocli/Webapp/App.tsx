import React, { useState, useCallback, useRef } from "react";
import { defaultProfiles } from "./data/defaultProfiles";
import {
  MacroProfile,
  HoveredDetails,
  KeyDefinition,
  KnobDefinition,
  KnobId,
  KnobActions,
} from "./types";
import MacroKeyboard from "./components/MacroKeyboard";
import DetailsPanel from "./components/DetailsPanel";
import EditKeyModal from "./components/EditKeyModal";
import EditKnobModal from "./components/EditKnobModal";
import { DownloadIcon, UploadIcon } from "./components/icons";

const parseMapping = (mappingStr: string | undefined): string[] => {
  if (!mappingStr) return [];
  const parts = mappingStr.split(",");
  // Handle the case of a single empty string from an empty mapping ""
  if (parts.length === 1 && parts[0].trim() === "") return [];
  // Trim and filter out any empty parts that might result from ",," or trailing commas.
  return parts.map((p) => p.trim()).filter((p) => p);
};

const generateFunctionName = async (sequence: string[]): Promise<string> => {
  const apiKey = process.env.GEMINI_API_KEY;
  // Simple fallback if API key is missing
  if (!apiKey || apiKey === "PLACEHOLDER_API_KEY") {
    const name = sequence.join(", ");
    // Return a truncated name if it's too long for a label
    return name.length > 20 ? name.substring(0, 17) + "..." : name;
  }

  const url = `https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key=${apiKey}`;

  const prompt = `Analyze the following keyboard macro sequence and provide a short, descriptive name for its function (max 4 words).
  For example, for the sequence ["ctrl-c"], the name is "Copy". For ["ctrl-v"], it's "Paste". For ["ctrl-s"], it's "Save".
  The name should be concise and suitable for a button label. Do not include any explanation, quotes, or extra text—just the name.

  Sequence: ${JSON.stringify(sequence)}`;

  try {
    const response = await fetch(url, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        contents: [{ parts: [{ text: prompt }] }],
        generationConfig: {
          temperature: 0.2,
          maxOutputTokens: 10,
        },
      }),
    });

    if (!response.ok)
      throw new Error(`API request failed: ${response.statusText}`);

    const data = await response.json();
    if (!data.candidates || data.candidates.length === 0) {
      throw new Error("No candidates returned from API.");
    }
    const text = data.candidates[0].content.parts[0].text;
    return text.trim().replace(/["']/g, ""); // Clean up any quotes
  } catch (error) {
    console.error("Error generating function name:", error);
    const name = sequence.join(", ");
    return name.length > 20 ? name.substring(0, 17) + "..." : name;
  }
};

const parseRon = async (content: string): Promise<MacroProfile[]> => {
  const cleanedContent = content.replace(/\s+/g, " ");
  const layersMatch = cleanedContent.match(/layers: \s*\[(.*)\]\s*,?\s*\)/);
  if (!layersMatch)
    throw new Error(
      "Could not find a valid `layers: [...]` array in the RON file."
    );

  let layersContent = layersMatch[1].trim();
  const layerStrings: string[] = [];
  let depth = 0;
  let start = -1;
  for (let i = 0; i < layersContent.length; i++) {
    if (layersContent[i] === "(") {
      if (depth === 0) start = i;
      depth++;
    } else if (layersContent[i] === ")") {
      depth--;
      if (depth === 0 && start !== -1) {
        layerStrings.push(layersContent.substring(start, i + 1));
        start = -1;
      }
    }
  }

  const profilePromises = layerStrings.map(async (layerStr, index) => {
    const buttonsMatch = layerStr.match(/buttons: \s*\[(.*)\]\s*,\s*knobs:/);
    if (!buttonsMatch)
      throw new Error(`Layer ${index + 1}: could not find buttons.`);
    const buttonsContent = buttonsMatch[1];

    const rowsMatch = buttonsContent.match(/\[(.*?)\]/g);
    const keys: { [key: string]: KeyDefinition } = {};
    let keyCounter = 1;
    const keyPromises: Promise<{
      keyId: string;
      keyDef: KeyDefinition;
    } | null>[] = [];

    if (rowsMatch) {
      rowsMatch.forEach((rowStr) => {
        const buttonMatches = rowStr.match(
          /\(delay: (\d+), mapping: "(.*?)"\)/g
        );
        if (buttonMatches) {
          buttonMatches.forEach((buttonStr) => {
            const currentKeyId = keyCounter.toString();
            const buttonDataMatch = buttonStr.match(
              /\(delay: (\d+), mapping: "(.*?)"\)/
            );
            if (buttonDataMatch) {
              const delay = parseInt(buttonDataMatch[1], 10);
              const mappingStr = buttonDataMatch[2];
              const sequence = parseMapping(mappingStr);

              if (sequence.length > 0) {
                const promise = generateFunctionName(sequence).then((name) => ({
                  keyId: currentKeyId,
                  keyDef: {
                    function: name,
                    description: `Sequence: ${sequence.join(", ")}`,
                    sequence: sequence,
                    keypressCount: sequence.length,
                    delay: delay > 0 ? delay : undefined,
                  },
                }));
                keyPromises.push(promise);
              }
            }
            keyCounter++;
          });
        }
      });
    }

    const resolvedKeys = await Promise.all(keyPromises);
    resolvedKeys.forEach((result) => {
      if (result) {
        keys[result.keyId] = result.keyDef;
      }
    });

    const knobsMatch = layerStr.match(/knobs: \s*\[(.*?)\]/);
    if (!knobsMatch)
      throw new Error(`Layer ${index + 1}: could not find knobs.`);
    const knobsContent = knobsMatch[1];

    const knobDefs: KnobDefinition = {
      knob1: {
        press: [],
        rotateLeft: [],
        rotateRight: [],
        description: "Imported",
      },
      knob2: {
        press: [],
        rotateLeft: [],
        rotateRight: [],
        description: "Imported",
      },
      knob3: {
        press: [],
        rotateLeft: [],
        rotateRight: [],
        description: "Imported",
      },
    };

    const knobActionRegex = /\(delay: \d+, mapping: "(.*?)"\)/;
    const knobRegex = /\(ccw: (\(.*?\)), press: (\(.*?\)), cw: (\(.*?\))\)/g;
    let knobMatches;
    let knobIndex = 1;
    while (
      (knobMatches = knobRegex.exec(knobsContent)) !== null &&
      knobIndex <= 3
    ) {
      const [, ccw, press, cw] = knobMatches;
      const knobId = `knob${knobIndex}` as KnobId;

      const ccwMatch = ccw.match(knobActionRegex);
      if (ccwMatch) knobDefs[knobId].rotateLeft = parseMapping(ccwMatch[1]);

      const pressMatch = press.match(knobActionRegex);
      if (pressMatch) knobDefs[knobId].press = parseMapping(pressMatch[1]);

      const cwMatch = cw.match(knobActionRegex);
      if (cwMatch) knobDefs[knobId].rotateRight = parseMapping(cwMatch[1]);

      knobIndex++;
    }

    return {
      profileName: `Layer ${index + 1}`,
      description: `Imported from .ron file.`,
      keys,
      knobs: knobDefs,
      keypressLimit: 17,
      totalKeys: 12,
      brokenKeys: [],
    };
  });

  const profiles = await Promise.all(profilePromises);

  if (profiles.length === 0) {
    throw new Error("No valid layers found in the RON file.");
  }
  return profiles;
};

const App: React.FC = () => {
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [profiles, setProfiles] = useState<MacroProfile[]>(() => {
    try {
      return JSON.parse(JSON.stringify(defaultProfiles));
    } catch {
      return [];
    }
  });
  const [selectedProfileName, setSelectedProfileName] = useState<string>(
    defaultProfiles[0].profileName
  );
  const selectedProfile = profiles.find(
    (p) => p.profileName === selectedProfileName
  )!;

  const fileInputRef = useRef<HTMLInputElement>(null);

  const [hoveredDetails, setHoveredDetails] = useState<HoveredDetails>(null);
  const [editingKey, setEditingKey] = useState<number | null>(null);
  const [editingKnob, setEditingKnob] = useState<KnobId | null>(null);

  const handleProfileChange = useCallback((profile: MacroProfile) => {
    setSelectedProfileName(profile.profileName);
    setHoveredDetails(null);
    setEditingKey(null);
    setEditingKnob(null);
  }, []);

  const handleKeyClick = useCallback((keyNumber: number) => {
    setEditingKey(keyNumber);
  }, []);

  const handleKnobClick = useCallback((knobId: KnobId) => {
    setEditingKnob(knobId);
  }, []);

  const handleSaveKey = useCallback(
    (keyNumber: number, newKeyData: KeyDefinition) => {
      setProfiles((currentProfiles) =>
        currentProfiles.map((profile) => {
          if (profile.profileName === selectedProfileName) {
            const updatedKeys = { ...profile.keys, [keyNumber]: newKeyData };
            return { ...profile, keys: updatedKeys };
          }
          return profile;
        })
      );
      setEditingKey(null);
    },
    [selectedProfileName]
  );

  const handleSaveKnob = useCallback(
    (knobId: KnobId, newKnobData: KnobActions) => {
      setProfiles((currentProfiles) =>
        currentProfiles.map((profile) => {
          if (profile.profileName === selectedProfileName) {
            const updatedKnobs = { ...profile.knobs, [knobId]: newKnobData };
            return { ...profile, knobs: updatedKnobs };
          }
          return profile;
        })
      );
      setEditingKnob(null);
    },
    [selectedProfileName]
  );

  const handleImportClick = () => {
    fileInputRef.current?.click();
  };

  const handleFileSelected = async (
    event: React.ChangeEvent<HTMLInputElement>
  ) => {
    const file = event.target.files?.[0];
    if (!file) return;

    setIsLoading(true);
    const reader = new FileReader();
    reader.onload = async (e) => {
      const content = e.target?.result as string;
      try {
        const newProfiles = await parseRon(content);
        setProfiles(newProfiles);
        setSelectedProfileName(newProfiles[0]?.profileName || "");
        setHoveredDetails(null);
        setEditingKey(null);
        alert("Configuration imported and analyzed successfully!");
      } catch (error) {
        console.error("Failed to parse .ron file:", error);
        alert(
          `Failed to parse .ron file: ${
            error instanceof Error ? error.message : String(error)
          }`
        );
      } finally {
        setIsLoading(false);
        if (event.target) event.target.value = "";
      }
    };
    reader.readAsText(file);
  };

  const handleExport = useCallback(() => {
    const formatMapping = (seq: string | string[]) => {
      if (!seq || seq.length === 0) return "";
      const sequence = Array.isArray(seq) ? seq : [seq];
      return sequence
        .map((k) => k.toLowerCase().replace(/\s/g, "").replace(/\+/g, "-"))
        .join(",");
    };

    const formatButton = (key: KeyDefinition | undefined) => {
      const mapping = key ? formatMapping(key.sequence) : "";
      const delay = key?.delay || 0;
      return `(delay: ${delay}, mapping: "${mapping}")`;
    };

    const formatKnob = (actions: KnobActions) => {
      const ccw = `(delay: 0, mapping: "${formatMapping(actions.rotateLeft)}")`;
      const press = `(delay: 0, mapping: "${formatMapping(actions.press)}")`;
      const cw = `(delay: 0, mapping: "${formatMapping(actions.rotateRight)}")`;
      return `(ccw: ${ccw}, press: ${press}, cw: ${cw})`;
    };

    let ronString = `(\n    device: (\n        orientation: Normal,\n        rows: 3,\n        cols: 4,\n        knobs: 3,\n    ),\n    layers: [\n`;

    profiles.forEach((profile, index) => {
      ronString += `        (\n`; // start layer
      ronString += `            buttons: [\n`;

      for (let i = 0; i < 3; i++) {
        const rowButtons = [];
        for (let j = 0; j < 4; j++) {
          const keyNumber = i * 4 + j + 1;
          const keyData = profile.keys[keyNumber.toString()];
          rowButtons.push(formatButton(keyData));
        }
        ronString += `                [${rowButtons.join(", ")}]`;
        if (i < 2) ronString += `,\n`;
        else ronString += `\n`;
      }

      ronString += `            ],\n`;
      ronString += `            knobs: [\n`;

      const knobIds: KnobId[] = ["knob1", "knob2", "knob3"];
      const knobStrings: string[] = [];
      knobIds.forEach((knobId) => {
        const knobData = profile.knobs[knobId];
        knobStrings.push(`                ${formatKnob(knobData)}`);
      });
      ronString += knobStrings.join(",\n");
      ronString += `\n`;

      ronString += `            ],\n`;
      ronString += `        )`; // end layer
      if (index < profiles.length - 1) ronString += `,\n`;
      else ronString += `\n`;
    });

    ronString += `    ],\n)\n`;

    const blob = new Blob([ronString], { type: "text/plain;charset=utf-8" });
    const url = URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.setAttribute("href", url);
    link.setAttribute("download", "macropad_config.ron");
    link.style.visibility = "hidden";
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  }, [profiles]);

  if (!selectedProfile) {
    return (
      <main className="bg-zinc-900 text-gray-300 min-h-screen flex flex-col items-center justify-center p-4 font-sans">
        {isLoading && (
          <div className="fixed inset-0 bg-black bg-opacity-60 flex items-center justify-center z-50">
            <div className="text-white text-2xl font-bold animate-pulse">
              Importing & Analyzing...
            </div>
          </div>
        )}
        <h1 className="text-4xl font-bold text-white mb-2">
          No Profile Loaded
        </h1>
        <p className="text-lg text-gray-400">Import a .ron file to begin.</p>
        <input
          type="file"
          ref={fileInputRef}
          onChange={handleFileSelected}
          style={{ display: "none" }}
          accept=".ron"
        />
        <button
          onClick={handleImportClick}
          className="mt-4 flex items-center gap-2 px-4 py-2 rounded-md text-sm font-medium transition-colors duration-200 bg-green-600 text-white hover:bg-green-500 shadow-lg"
          title="Import a .ron configuration file"
        >
          <UploadIcon className="w-4 h-4" />
          Import .ron
        </button>
      </main>
    );
  }

  return (
    <main className="bg-zinc-900 text-gray-300 min-h-screen flex flex-col items-center justify-center p-4 font-sans">
      {isLoading && (
        <div className="fixed inset-0 bg-black bg-opacity-60 flex items-center justify-center z-50">
          <div className="text-white text-2xl font-bold animate-pulse">
            Importing & Analyzing...
          </div>
        </div>
      )}
      <div className="w-full max-w-7xl">
        <header className="mb-6 text-center">
          <h1 className="text-4xl font-bold text-white mb-2">
            Macro Keyboard Visualizer
          </h1>
          <p className="text-lg text-gray-400">
            Click a key or knob to edit. Select a profile to visualize its
            layout.
          </p>
        </header>

        <div className="mb-8 flex justify-center items-center gap-3 bg-zinc-800 p-3 rounded-lg">
          <span className="font-semibold text-white">Select Profile:</span>
          {profiles.map((profile) => (
            <button
              key={profile.profileName}
              onClick={() => handleProfileChange(profile)}
              className={`px-4 py-2 rounded-md text-sm font-medium transition-colors duration-200 ${
                selectedProfile.profileName === profile.profileName
                  ? "bg-green-600 text-white shadow-lg"
                  : "bg-zinc-700 text-gray-300 hover:bg-zinc-600"
              }`}
            >
              {profile.profileName}
            </button>
          ))}
          <input
            type="file"
            ref={fileInputRef}
            onChange={handleFileSelected}
            style={{ display: "none" }}
            accept=".ron"
          />
          <button
            onClick={handleImportClick}
            className="flex items-center gap-2 px-4 py-2 rounded-md text-sm font-medium transition-colors duration-200 bg-green-600 text-white hover:bg-green-500 shadow-lg"
            title="Import a .ron configuration file"
          >
            <UploadIcon className="w-4 h-4" />
            Import .ron
          </button>
          <button
            onClick={handleExport}
            className="flex items-center gap-2 px-4 py-2 rounded-md text-sm font-medium transition-colors duration-200 bg-blue-600 text-white hover:bg-blue-500 shadow-lg"
            title="Export all layers to a .ron configuration file"
          >
            <DownloadIcon className="w-4 h-4" />
            Export .ron
          </button>
        </div>

        <div className="flex flex-col lg:flex-row gap-8 justify-center">
          <MacroKeyboard
            profile={selectedProfile}
            onHover={setHoveredDetails}
            onKeyClick={handleKeyClick}
            onKnobClick={handleKnobClick}
          />
          <DetailsPanel
            details={hoveredDetails}
            profileDescription={selectedProfile.description}
          />
        </div>
      </div>
      {editingKey !== null && (
        <EditKeyModal
          keyNumber={editingKey}
          profileName={selectedProfile.profileName}
          keyData={selectedProfile.keys[editingKey.toString()]}
          onSave={handleSaveKey}
          onClose={() => setEditingKey(null)}
          keypressLimit={selectedProfile.keypressLimit}
        />
      )}
      {editingKnob !== null && (
        <EditKnobModal
          knobId={editingKnob}
          profileName={selectedProfile.profileName}
          knobData={selectedProfile.knobs[editingKnob]}
          onSave={handleSaveKnob}
          onClose={() => setEditingKnob(null)}
          keypressLimit={selectedProfile.keypressLimit}
        />
      )}
    </main>
  );
};

export default App;
