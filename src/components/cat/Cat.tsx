import { useState, useRef, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { LogicalPosition } from "@tauri-apps/api/dpi";
import { useCatStore } from "../../stores/catStore";
import "./Cat.css";

// ── 랜덤 행동 ──
type IdleBehavior = "walk" | "sit" | "lick" | "lookAround" | "stretch" | "nap" | "wiggle";

const behaviorEmoji: Record<IdleBehavior, string> = {
  walk: "🐱", sit: "🐱", lick: "😽", lookAround: "🙀",
  stretch: "😸", nap: "😴", wiggle: "😼",
};

const behaviorDuration: Record<IdleBehavior, [number, number]> = {
  walk: [3000, 8000], sit: [2000, 5000], lick: [1500, 3000],
  lookAround: [1000, 2000], stretch: [1500, 2500], nap: [5000, 10000], wiggle: [800, 1500],
};

function getBehaviorWeights(): Record<IdleBehavior, number> {
  const hour = new Date().getHours();
  if (hour >= 23 || hour < 6) return { walk: 1, sit: 3, lick: 1, lookAround: 1, stretch: 1, nap: 8, wiggle: 0 };
  if (hour >= 6 && hour < 10) return { walk: 3, sit: 2, lick: 2, lookAround: 2, stretch: 4, nap: 1, wiggle: 2 };
  return { walk: 4, sit: 3, lick: 2, lookAround: 2, stretch: 2, nap: 1, wiggle: 2 };
}

function pickRandomBehavior(): IdleBehavior {
  const weights = getBehaviorWeights();
  const entries = Object.entries(weights) as [IdleBehavior, number][];
  const total = entries.reduce((sum, [, w]) => sum + w, 0);
  let rand = Math.random() * total;
  for (const [b, w] of entries) { rand -= w; if (rand <= 0) return b; }
  return "walk";
}

function randomInRange(min: number, max: number) { return min + Math.random() * (max - min); }

// ── 메시지 ──
const normalMessages = ["meow!", "nya~", "purr...", "mrrp?", "*stretch*", "code with me~", "prrrr~"];
const happyMessages = ["😻 love it!", "more pets!", "purrrr~", "nya nya~!"];
const annoyedMessages = ["...meow.", "okay okay!", "I'm busy!", "stahp!", "😾"];
const autoMessages = ["*yawn*", "...", "💭", "hmm...", "*tail swish*", "commit something!", "☕", "*purr*"];

// ── 윈도우 크기 ──
const WIN_W = 120;
const WIN_H = 100;

export function Cat() {
  const { state } = useCatStore();
  const appWindow = useRef(getCurrentWindow());

  // ── 윈도우 위치 (화면 좌표) ──
  const [winPos, setWinPos] = useState({ x: 300, y: 400 });
  const winPosRef = useRef({ x: 300, y: 400 });

  // ── 드래그 ──
  const [isDragging, setIsDragging] = useState(false);
  const didDrag = useRef(false);
  const dragStartMouse = useRef({ x: 0, y: 0 });
  const dragStartWin = useRef({ x: 0, y: 0 });

  // ── 행동 ──
  const [behavior, setBehavior] = useState<IdleBehavior>("sit");
  const behaviorTimeout = useRef<ReturnType<typeof setTimeout> | null>(null);
  const [direction, setDirection] = useState<"left" | "right">("right");

  // ── 말풍선 ──
  const [bubble, setBubble] = useState<string | null>(null);
  const [bubbleKey, setBubbleKey] = useState(0);
  const bubbleTimer = useRef<ReturnType<typeof setTimeout> | null>(null);

  // ── 클릭 카운터 ──
  const clickCount = useRef(0);
  const clickResetTimer = useRef<ReturnType<typeof setTimeout> | null>(null);

  // ── 이모지 ──
  const [displayEmoji, setDisplayEmoji] = useState("🐱");

  // ── 화면 크기 ──
  const screenW = useRef(window.screen.width);
  const screenH = useRef(window.screen.height);

  // ══════════════════════════════════════
  // 윈도우 위치 동기화
  // ══════════════════════════════════════
  const moveWindow = useCallback(async (x: number, y: number) => {
    winPosRef.current = { x, y };
    setWinPos({ x, y });
    try {
      await appWindow.current.setPosition(new LogicalPosition(Math.round(x), Math.round(y)));
    } catch (e) {
      // 무시 (너무 빠른 호출 시 에러 가능)
    }
  }, []);

  // 시작 시 현재 윈도우 위치 가져오기
  useEffect(() => {
    (async () => {
      try {
        const pos = await appWindow.current.outerPosition();
        winPosRef.current = { x: pos.x, y: pos.y };
        setWinPos({ x: pos.x, y: pos.y });
      } catch (e) {}
    })();
  }, []);

  // ══════════════════════════════════════
  // 말풍선
  // ══════════════════════════════════════
  const showBubble = useCallback((msg: string, duration = 2000) => {
    setBubble(msg);
    setBubbleKey(k => k + 1);
    if (bubbleTimer.current) clearTimeout(bubbleTimer.current);
    bubbleTimer.current = setTimeout(() => setBubble(null), duration);
  }, []);

  // ══════════════════════════════════════
  // 랜덤 행동 루프
  // ══════════════════════════════════════
  const scheduleBehavior = useCallback(() => {
    const next = pickRandomBehavior();
    const [minDur, maxDur] = behaviorDuration[next];
    const duration = randomInRange(minDur, maxDur);

    setBehavior(next);
    setDisplayEmoji(behaviorEmoji[next]);

    if (next === "walk") {
      setDirection(Math.random() > 0.5 ? "right" : "left");
    }

    behaviorTimeout.current = setTimeout(() => scheduleBehavior(), duration);
  }, []);

  useEffect(() => {
    if (state !== "idle" || isDragging) {
      if (behaviorTimeout.current) clearTimeout(behaviorTimeout.current);
      return;
    }
    scheduleBehavior();
    return () => { if (behaviorTimeout.current) clearTimeout(behaviorTimeout.current); };
  }, [state, isDragging, scheduleBehavior]);

  // 상태별 이모지 오버라이드
  useEffect(() => {
    const m: Record<string, string> = {
      coding: "😺", celebrating: "🎉", frustrated: "😿",
      sleeping: "😴", tired: "🥱", interaction: "😻",
    };
    if (state !== "idle") setDisplayEmoji(m[state] ?? "🐱");
  }, [state]);

  // ── 상태 전환 시 말풍선 ──
  const prevState = useRef(state);
  useEffect(() => {
    if (prevState.current === state) return;
    const prev = prevState.current;
    prevState.current = state;

    const transitions: Record<string, string[]> = {
      coding: ["let's code! 💻", "focus mode!", "coding time~", "⌨️ *tap tap*"],
      sleeping: ["zzz...", "so sleepy...", "💤 good night...", "*curls up*"],
      tired: ["it's late... 🌙", "*yawn* still coding?", "go to bed!"],
      celebrating: ["🎉 commit!", "nice commit!", "woohoo!"],
      idle: prev === "coding"
        ? ["break time~", "done coding?", "*stretch*"]
        : prev === "sleeping"
        ? ["*wakes up* 🥱", "morning!", "I'm up!"]
        : [],
    };

    const msgs = transitions[state];
    if (msgs && msgs.length > 0) {
      const msg = msgs[Math.floor(Math.random() * msgs.length)];
      showBubble(msg, 2500);
    }
  }, [state, showBubble]);

  // ══════════════════════════════════════
  // Walk: 윈도우 자체를 이동
  // ══════════════════════════════════════
  useEffect(() => {
    if (state !== "idle" || behavior !== "walk" || isDragging) return;

    const interval = setInterval(() => {
      const pos = winPosRef.current;
      const speed = 1.2 + Math.random() * 0.6;
      let newX = pos.x + (direction === "right" ? speed : -speed);

      // 화면 경계 체크
      const maxX = screenW.current - WIN_W;
      if (newX > maxX) {
        setDirection("left");
        newX = maxX;
      } else if (newX < 0) {
        setDirection("right");
        newX = 0;
      }

      moveWindow(newX, pos.y);
    }, 30);

    return () => clearInterval(interval);
  }, [state, behavior, direction, isDragging, moveWindow]);

  // ══════════════════════════════════════
  // 자동 혼잣말
  // ══════════════════════════════════════
  useEffect(() => {
    const interval = setInterval(() => {
      if (state !== "idle" || isDragging || bubble) return;
      if (Math.random() < 0.15) {
        const msg = autoMessages[Math.floor(Math.random() * autoMessages.length)];
        showBubble(msg, 2500);
      }
    }, 8000);
    return () => clearInterval(interval);
  }, [state, isDragging, bubble, showBubble]);

  // ══════════════════════════════════════
  // 드래그: 윈도우 자체를 이동
  // ══════════════════════════════════════
  const handleMouseDown = useCallback((e: React.MouseEvent) => {
    setIsDragging(true);
    didDrag.current = false;
    dragStartMouse.current = { x: e.screenX, y: e.screenY };
    dragStartWin.current = { ...winPosRef.current };
  }, []);

  useEffect(() => {
    if (!isDragging) return;

    const handleMove = (e: MouseEvent) => {
      didDrag.current = true;
      const dx = e.screenX - dragStartMouse.current.x;
      const dy = e.screenY - dragStartMouse.current.y;
      moveWindow(dragStartWin.current.x + dx, dragStartWin.current.y + dy);
    };

    const handleUp = () => {
      setIsDragging(false);
      if (didDrag.current) {
        showBubble("wheee~!", 1500);
      }
    };

    window.addEventListener("mousemove", handleMove);
    window.addEventListener("mouseup", handleUp);
    return () => {
      window.removeEventListener("mousemove", handleMove);
      window.removeEventListener("mouseup", handleUp);
    };
  }, [isDragging, moveWindow, showBubble]);

  // ══════════════════════════════════════
  // 클릭 반응
  // ══════════════════════════════════════
  const handleClick = async () => {
    if (didDrag.current) return;
    try { await invoke<string>("click_cat"); } catch (e) { console.error(e); }

    clickCount.current += 1;
    const count = clickCount.current;

    if (clickResetTimer.current) clearTimeout(clickResetTimer.current);
    clickResetTimer.current = setTimeout(() => { clickCount.current = 0; }, 3000);

    let msg: string;
    if (count <= 2) {
      msg = normalMessages[Math.floor(Math.random() * normalMessages.length)];
      setDisplayEmoji("😻");
    } else if (count <= 5) {
      msg = happyMessages[Math.floor(Math.random() * happyMessages.length)];
      setDisplayEmoji("🥰");
    } else {
      msg = annoyedMessages[Math.floor(Math.random() * annoyedMessages.length)];
      setDisplayEmoji("😾");
    }
    showBubble(msg);

    setTimeout(() => {
      if (state === "idle") setDisplayEmoji(behaviorEmoji[behavior]);
    }, 2000);
  };

  // ══════════════════════════════════════
  // 렌더
  // ══════════════════════════════════════
  const isFlipped = direction === "left";

  return (
    <div className="cat-window">
      {/* 말풍선 (상단) */}
      <div className="bubble-area">
        {bubble && (
          <div className="cat__bubble" key={bubbleKey}>
            {bubble}
          </div>
        )}
      </div>

      {/* 고양이 (하단) */}
      <div
        className={`cat cat--${state} cat--${behavior} ${isDragging ? "cat--dragging" : ""}`}
        onMouseDown={handleMouseDown}
        onClick={handleClick}
      >
        <div
          className="cat__sprite"
          style={{ transform: isFlipped ? "scaleX(-1)" : "scaleX(1)" }}
        >
          <span className="cat__emoji">{displayEmoji}</span>
        </div>

        {behavior === "nap" && state === "idle" && <div className="cat__zzz">z z z</div>}
        {behavior === "lick" && state === "idle" && <div className="cat__effect">✨</div>}
        {state === "celebrating" && <div className="cat__particles">✨🎉✨</div>}
      </div>
    </div>
  );
}
