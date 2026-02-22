import { useEffect, useRef } from "react";
import { listen } from "@tauri-apps/api/event";
import { Cat } from "./components/cat/Cat";
import { useCatStore } from "./stores/catStore";

// 백엔드에서 오는 상태 데이터
interface ActivityStatus {
  isIdeRunning: boolean;
  activeIde: string | null;
  idleSeconds: number;
}

function App() {
  const { setState, setActiveIde, setIdleSeconds, addCodingMinute } = useCatStore();

  // celebrating/interaction 같은 임시 상태의 자동 복귀 타이머
  const tempStateTimer = useRef<ReturnType<typeof setTimeout> | null>(null);

  // 코딩 시간 카운터 (1분마다)
  const codingTimer = useRef<ReturnType<typeof setInterval> | null>(null);

  useEffect(() => {
    const unlisten = Promise.all([
      // ── IDE 감지됨 → coding ──
      listen<string>("activity:ide-detected", (event) => {
        const ideName = event.payload;
        setActiveIde(ideName);
        setState("coding");

        // 코딩 시간 카운트 시작
        if (!codingTimer.current) {
          codingTimer.current = setInterval(() => {
            addCodingMinute();
          }, 60_000);
        }
      }),

      // ── IDE 꺼짐 → idle ──
      listen("activity:ide-closed", () => {
        setActiveIde(null);
        setState("idle");

        // 코딩 시간 카운트 중지
        if (codingTimer.current) {
          clearInterval(codingTimer.current);
          codingTimer.current = null;
        }
      }),

      // ── 유휴 → idle ──
      listen<number>("activity:idle", () => {
        setState("idle");
      }),

      // ── 장시간 유휴 → sleeping ──
      listen<number>("activity:sleeping", () => {
        setState("sleeping");

        if (codingTimer.current) {
          clearInterval(codingTimer.current);
          codingTimer.current = null;
        }
      }),

      // ── 밤 코딩 → tired ──
      listen<number>("activity:late-night-coding", () => {
        // coding 상태에서만 tired로
        const current = useCatStore.getState().state;
        if (current === "coding") {
          setState("tired");
        }
      }),

      // ── 주기적 상태 보고 (초기 상태 동기화 포함) ──
      listen<ActivityStatus>("activity:status", (event) => {
        const status = event.payload;
        setIdleSeconds(status.idleSeconds);

        // 현재 상태와 IDE 상태 동기화
        const currentState = useCatStore.getState().state;
        const currentIde = useCatStore.getState().activeIde;

        if (status.isIdeRunning && !currentIde) {
          // IDE가 실행 중인데 아직 감지 안됨 → coding
          setActiveIde(status.activeIde);
          if (currentState === "idle") {
            setState("coding");
          }
        } else if (!status.isIdeRunning && currentIde) {
          // IDE가 꺼졌는데 아직 반영 안됨 → idle
          setActiveIde(null);
          if (currentState === "coding") {
            setState("idle");
          }
        }
      }),

      // ── Git 커밋 → celebrating (임시) ──
      listen("git:new-commit", () => {
        if (tempStateTimer.current) clearTimeout(tempStateTimer.current);
        setState("celebrating");
        tempStateTimer.current = setTimeout(() => {
          // 이전 상태로 복귀
          const ide = useCatStore.getState().activeIde;
          setState(ide ? "coding" : "idle");
        }, 3000);
      }),

      // ── 풀스크린 ──
      listen<boolean>("activity:fullscreen", (event) => {
        const el = document.getElementById("root");
        if (el) {
          el.style.display = event.payload ? "none" : "block";
        }
      }),
    ]);

    return () => {
      unlisten.then((fns) => fns.forEach((fn) => fn()));
      if (codingTimer.current) clearInterval(codingTimer.current);
    };
  }, [setState, setActiveIde, setIdleSeconds, addCodingMinute]);

  return <Cat />;
}

export default App;
