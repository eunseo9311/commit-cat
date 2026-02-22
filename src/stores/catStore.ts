import { create } from "zustand";

type CatState =
  | "idle"
  | "coding"
  | "celebrating"
  | "frustrated"
  | "sleeping"
  | "tired"
  | "interaction";

type CatMood = "happy" | "sad" | "sleeping" | "focused" | "excited";

interface CatStore {
  // State
  state: CatState;
  mood: CatMood;
  level: number;
  exp: number;
  expToNext: number;
  streakDays: number;

  // Today
  todayCodingMinutes: number;
  todayCommits: number;
  todayPomodoros: number;

  // Actions
  setState: (state: string) => void;
  setLevel: (level: number, exp: number, expToNext: number) => void;
  addCommit: () => void;
  addCodingMinute: () => void;
  addPomodoro: () => void;
}

const moodFromState = (state: CatState): CatMood => {
  const map: Record<CatState, CatMood> = {
    idle: "happy",
    coding: "focused",
    celebrating: "excited",
    frustrated: "sad",
    sleeping: "sleeping",
    tired: "sad",
    interaction: "happy",
  };
  return map[state] ?? "happy";
};

export const useCatStore = create<CatStore>((set) => ({
  // Initial state
  state: "idle",
  mood: "happy",
  level: 1,
  exp: 0,
  expToNext: 60,
  streakDays: 0,

  todayCodingMinutes: 0,
  todayCommits: 0,
  todayPomodoros: 0,

  // Actions
  setState: (state) =>
    set({
      state: state as CatState,
      mood: moodFromState(state as CatState),
    }),

  setLevel: (level, exp, expToNext) =>
    set({ level, exp, expToNext }),

  addCommit: () =>
    set((s) => ({ todayCommits: s.todayCommits + 1 })),

  addCodingMinute: () =>
    set((s) => ({ todayCodingMinutes: s.todayCodingMinutes + 1 })),

  addPomodoro: () =>
    set((s) => ({ todayPomodoros: s.todayPomodoros + 1 })),
}));
