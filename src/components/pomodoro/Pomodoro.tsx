import { useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useCatStore } from "../../stores/catStore";

/**
 * 🍅 뽀모도로 타이머
 */
export function Pomodoro() {
  const [isActive, setIsActive] = useState(false);
  const [remaining, setRemaining] = useState(25 * 60);
  const { addPomodoro } = useCatStore();

  useEffect(() => {
    const unlistenTick = listen("pomodoro:tick", (event) => {
      setRemaining(event.payload as number);
    });

    const unlistenComplete = listen("pomodoro:complete", () => {
      setIsActive(false);
      setRemaining(25 * 60);
      addPomodoro();
    });

    return () => {
      unlistenTick.then((fn) => fn());
      unlistenComplete.then((fn) => fn());
    };
  }, [addPomodoro]);

  const toggle = useCallback(async () => {
    if (isActive) {
      await invoke("stop_pomodoro");
      setIsActive(false);
    } else {
      await invoke("start_pomodoro");
      setIsActive(true);
    }
  }, [isActive]);

  const minutes = Math.floor(remaining / 60);
  const seconds = remaining % 60;

  return (
    <div className="pomodoro">
      <div className="pomodoro__time">
        {String(minutes).padStart(2, "0")}:{String(seconds).padStart(2, "0")}
      </div>
      <button className="pomodoro__btn" onClick={toggle}>
        {isActive ? "⏹ Stop" : "▶ Start"}
      </button>
    </div>
  );
}
