import { MacroProfile } from "../types";

const layer1: MacroProfile = {
  profileName: "General - Editing",
  description:
    "Basic text editing commands like copy, paste, and undo, plus simple navigation.",
  keys: {
    "1": {
      function: "Cut",
      sequence: ["ctrl-x"],
      description: "Cuts the selected content.",
    },
    "2": {
      function: "Copy",
      sequence: ["ctrl-c"],
      description: "Copies the selected content.",
    },
    "3": {
      function: "Paste",
      sequence: ["ctrl-v"],
      description: "Pastes content from the clipboard.",
    },
    "4": {
      function: "Save",
      sequence: ["ctrl-s"],
      description: "Saves the current file.",
    },
    "5": {
      function: "Undo",
      sequence: ["ctrl-z"],
      description: "Undoes the last action.",
    },
    "6": {
      function: "Redo",
      sequence: ["ctrl-y"],
      description: "Redoes the last undone action.",
    },
    "7": {
      function: "Select All",
      sequence: ["ctrl-a"],
      description: "Selects all content.",
    },
    "8": {
      function: "Find",
      sequence: ["ctrl-f"],
      description: "Opens the find dialog.",
    },
    "9": {
      function: "Up Arrow",
      sequence: ["up"],
      description: "Navigates up one line.",
    },
    "10": {
      function: "Down Arrow",
      sequence: ["down"],
      description: "Navigates down one line.",
    },
    "11": {
      function: "Left Arrow",
      sequence: ["left"],
      description: "Navigates left one character.",
    },
    "12": {
      function: "Right Arrow",
      sequence: ["right"],
      description: "Navigates right one character.",
    },
  },
  knobs: {
    knob1: {
      rotateLeft: ["wheelup"],
      press: ["mclick"],
      rotateRight: ["wheeldown"],
      description: "Vertical Scroll",
    },
    knob2: {
      rotateLeft: ["volumedown"],
      press: ["mute"],
      rotateRight: ["volumeup"],
      description: "Volume Control",
    },
    knob3: {
      rotateLeft: ["home"],
      press: ["end"],
      rotateRight: ["pagedown"],
      description: "Page Navigation",
    },
  },
  keypressLimit: 17,
  totalKeys: 12,
  brokenKeys: [],
};

const layer2: MacroProfile = {
  profileName: "General - Web & Apps",
  description:
    "Commands for managing browser tabs and switching between applications.",
  keys: {
    "1": {
      function: "New Tab",
      sequence: ["ctrl-t"],
      description: "Opens a new browser tab.",
    },
    "2": {
      function: "Close Tab",
      sequence: ["ctrl-w"],
      description: "Closes the current tab.",
    },
    "3": {
      function: "Reopen Tab",
      sequence: ["ctrl-shift-t"],
      description: "Reopens the last closed tab.",
    },
    "4": {
      function: "Refresh",
      sequence: ["f5"],
      description: "Refreshes the current page.",
    },
    "5": {
      function: "Next Tab",
      sequence: ["ctrl-tab"],
      description: "Switches to the next tab.",
    },
    "6": {
      function: "Prev Tab",
      sequence: ["ctrl-shift-tab"],
      description: "Switches to the previous tab.",
    },
    "7": {
      function: "Switch App",
      sequence: ["alt-tab"],
      description: "Switches to the next application.",
    },
    "8": {
      function: "Close App",
      sequence: ["alt-f4"],
      description: "Closes the current application.",
    },
    "9": {
      function: "Browser Back",
      sequence: ["alt-left"],
      description: "Goes to the previous page.",
    },
    "10": {
      function: "Browser Fwd",
      sequence: ["alt-right"],
      description: "Goes to the next page.",
    },
    "11": {
      function: "Bookmark",
      sequence: ["ctrl-d"],
      description: "Bookmarks the current page.",
    },
    "12": {
      function: "Dev Tools",
      sequence: ["f12"],
      description: "Opens browser developer tools.",
    },
  },
  knobs: {
    knob1: {
      rotateLeft: ["ctrl-shift-tab"],
      press: ["ctrl-w"],
      rotateRight: ["ctrl-tab"],
      description: "Tab Navigation",
    },
    knob2: {
      rotateLeft: ["alt-shift-tab"],
      press: ["alt-f4"],
      rotateRight: ["alt-tab"],
      description: "App Switching",
    },
    knob3: {
      rotateLeft: ["alt-left"],
      press: ["f5"],
      rotateRight: ["alt-right"],
      description: "Browser History",
    },
  },
  keypressLimit: 17,
  totalKeys: 12,
  brokenKeys: [],
};

const layer3: MacroProfile = {
  profileName: "General - Media",
  description: "Media playback controls and system-level shortcuts.",
  keys: {
    "1": {
      function: "Play/Pause",
      sequence: ["play"],
      description: "Toggles media playback.",
    },
    "2": {
      function: "Next Track",
      sequence: ["next"],
      description: "Skips to the next track.",
    },
    "3": {
      function: "Prev Track",
      sequence: ["previous"],
      description: "Goes to the previous track.",
    },
    "4": {
      function: "Stop",
      sequence: ["stop"],
      description: "Stops media playback.",
    },
    "5": {
      function: "Volume Up",
      sequence: ["volumeup"],
      description: "Increases system volume.",
    },
    "6": {
      function: "Volume Down",
      sequence: ["volumedown"],
      description: "Decreases system volume.",
    },
    "7": {
      function: "Mute",
      sequence: ["mute"],
      description: "Mutes system volume.",
    },
    "8": {
      function: "Calculator",
      sequence: ["calculator"],
      description: "Opens the calculator app.",
    },
    "9": {
      function: "Screenshot",
      sequence: ["printscreen"],
      description: "Takes a screenshot.",
    },
    "10": {
      function: "Lock Screen",
      sequence: ["win-l"],
      description: "Locks the computer.",
    },
    "11": {
      function: "File Explorer",
      sequence: ["win-e"],
      description: "Opens the file explorer.",
    },
    "12": {
      function: "Show Desktop",
      sequence: ["win-d"],
      description: "Minimizes all windows.",
    },
  },
  knobs: {
    knob1: {
      rotateLeft: ["previous"],
      press: ["play"],
      rotateRight: ["next"],
      description: "Media Playback",
    },
    knob2: {
      rotateLeft: ["volumedown"],
      press: ["mute"],
      rotateRight: ["volumeup"],
      description: "Volume Control",
    },
    knob3: {
      rotateLeft: ["screenbrightnessdown"],
      press: ["calculator"],
      rotateRight: ["screenbrightnessup"],
      description: "System & Brightness",
    },
  },
  keypressLimit: 17,
  totalKeys: 12,
  brokenKeys: [],
};

export const defaultProfiles: MacroProfile[] = [layer1, layer2, layer3];
