import { useState, useRef, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useCatStore } from "../../stores/catStore";
import "./Cat.css";

// ── 랜덤 행동 타입 ──
type IdleBehavior = "walk" | "sit" | "lick" | "lookAround" | "stretch" | "nap" | "wiggle";

// ── 행동별 이모지 ──
const behaviorEmoji: Record<IdleBehavior, string> = {
  walk: "🐱",
  sit: "🐱",
  lick: "😽",
  lookAround: "🙀",
  stretch: "😸",
  nap: "😴",
  wiggle: "😼",
};

// ── 행동별 지속 시간 (ms) ──
const behaviorDuration: Record<IdleBehavior, [number, number]> = {
  walk: [3000, 8000],
  sit: [2000, 5000],
  lick: [1500, 3000],
  lookAround: [1000, 2000],
  stretch: [1500, 2500],
  nap: [5000, 10000],
  wiggle: [800, 1500],
};

// ── 시간대별 행동 가중치 ──
function getBehaviorWeights(): Record<IdleBehavior, number> {
  const hour = new Date().getHours();
  const isNight = hour >= 23 || hour < 6;
  const isMorning = hour >= 6 && hour < 10;

  if (isNight) {
    return { walk: 1, sit: 3, lick: 1, lookAround: 1, stretch: 1, nap: 8, wiggle: 0 };
  }
  if (isMorning) {
    return { walk: 3, sit: 2, lick: 2, lookAround: 2, stretch: 4, nap: 1, wiggle: 2 };
  }
  // 낮 (기본)
  return { walk: 4, sit: 3, lick: 2, lookAround: 2, stretch: 2, nap: 1, wiggle: 2 };
}

function pickRandomBehavior(): IdleBehavior {
  const weights = getBehaviorWeights();
  const entries = Object.entries(weights) as [IdleBehavior, number][];
  const total = entries.reduce((sum, [, w]) => sum + w, 0);
  let rand = Math.random() * total;
  for (const [behavior, weight] of entries) {
    rand -= weight;
    if (rand <= 0) return behavior;
  }
  return "walk";
}

function randomInRange(min: number, max: number): number {
  return min + Math.random() * (max - min);
}

// ── 클릭 메시지 (연속 클릭 시 반응 변화) ──
const normalMessages = [
  "meow!", "nya~", "purr...", "mrrp?",
  "*stretch*", "code with me~", "prrrr~",
];
const happyMessages = [
  "😻 love it!", "more pets!", "purrrr~", "nya nya~!",
];
const annoyedMessages = [
  "...meow.", "okay okay!", "I'm busy!", "stahp!", "😾",
];
const autoMessages = [
  "*yawn*", "...", "💭", "hmm...", "*tail swish*",
  "commit something!", "☕", "*purr*",
];

export function Cat() {
  const { state } = useCatStore();

  // ── 위치 & 드래그 ──
  const [position, setPosition] = useState({ x: 100, y: 100 });
  const [isDragging, setIsDragging] = useState(false);
  const didDrag = useRef(false);
  const dragOffset = useRef({ x: 0, y: 0 });

  // ── 방향 ──
  const [direction, setDirection] = useState<"left" | "right">("right");

  // ── 현재 행동 ──
  const [behavior, setBehavior] = useState<IdleBehavior>("sit");
  const behaviorTimeout = useRef<ReturnType<typeof setTimeout> | null>(null);

  // ── 말풍선 ──
  const [bubble, setBubble] = useState<string | null>(null);
  const [bubbleKey, setBubbleKey] = useState(0);
  const bubbleTimer = useRef<ReturnType<typeof setTimeout> | null>(null);

  // ── 클릭 카운터 (연속 클릭 감지) ──
  const clickCount = useRef(0);
  const clickResetTimer = useRef<ReturnType<typeof setTimeout> | null>(null);

  // ── 현재 표시할 이모지 ──
  const [displayEmoji, setDisplayEmoji] = useState("🐱");

  // ── 말풍선 표시 헬퍼 ──
  const showBubble = useCallback((msg: string, duration = 2000) => {
    setBubble(msg);
    setBubbleKey((k) => k + 1);
    if (bubbleTimer.current) clearTimeout(bubbleTimer.current);
    bubbleTimer.current = setTimeout(() => setBubble(null), duration);
  }, []);

  // ══════════════════════════════════════
  // 랜덤 행동 루프 (idle일 때만)
  // ══════════════════════════════════════
  const scheduleBehavior = useCallback(() => {
    const next = pickRandomBehavior();
    const [minDur, maxDur] = behaviorDuration[next];
    const duration = randomInRange(minDur, maxDur);

    setBehavior(next);
    setDisplayEmoji(behaviorEmoji[next]);

    // walk일 때 랜덤 방향
    if (next === "walk") {
      setDirection(Math.random() > 0.5 ? "right" : "left");
    }

    behaviorTimeout.current = setTimeout(() => {
      scheduleBehavior();
    }, duration);
  }, []);

  useEffect(() => {
    if (state !== "idle" || isDragging) {
      if (behaviorTimeout.current) clearTimeout(behaviorTimeout.current);
      return;
    }
    scheduleBehavior();
    return () => {
      if (behaviorTimeout.current) clearTimeout(behaviorTimeout.current);
    };
  }, [state, isDragging, scheduleBehavior]);

  // ── 상태별 이모지 오버라이드 ──
  useEffect(() => {
    const emojiMap: Record<string, string> = {
      coding: "😺",
      celebrating: "🎉",
      frustrated: "😿",
      sleeping: "😴",
      tired: "🥱",
      interaction: "😻",
    };
    if (state !== "idle") {
      setDisplayEmoji(emojiMap[state] ?? "🐱");
    }
  }, [state]);

  // ══════════════════════════════════════
  // 이동 (walk 행동일 때만) - requestAnimationFrame 사용
  // ══════════════════════════════════════
  const positionRef = useRef(position);
  positionRef.current = position;
  const directionRef = useRef(direction);
  directionRef.current = direction;

  useEffect(() => {
    if (state !== "idle" || behavior !== "walk" || isDragging) return;

    let animationId: number;
    let lastTime = performance.now();
    const baseSpeed = 40; // pixels per second

    const animate = (currentTime: number) => {
      const deltaTime = (currentTime - lastTime) / 1000; // 초 단위
      lastTime = currentTime;

      const speed = baseSpeed * deltaTime;
      const currentDir = directionRef.current;
      let newX = positionRef.current.x + (currentDir === "right" ? speed : -speed);

      // 경계 체크
      if (newX > window.innerWidth - 64) {
        setDirection("left");
        newX = window.innerWidth - 64;
      } else if (newX < 0) {
        setDirection("right");
        newX = 0;
      }

      setPosition((prev) => ({ ...prev, x: newX }));
      animationId = requestAnimationFrame(animate);
    };

    animationId = requestAnimationFrame(animate);
    return () => cancelAnimationFrame(animationId);
  }, [state, behavior, isDragging]);

  // ══════════════════════════════════════
  // 자동 말풍선 (가끔 혼잣말)
  // ══════════════════════════════════════
  const bubbleRef = useRef(bubble);
  bubbleRef.current = bubble;

  useEffect(() => {
    const interval = setInterval(() => {
      if (state !== "idle" || isDragging || bubbleRef.current) return;
      // 15% 확률로 혼잣말
      if (Math.random() < 0.15) {
        const msg = autoMessages[Math.floor(Math.random() * autoMessages.length)];
        showBubble(msg, 2500);
      }
    }, 8000);

    return () => clearInterval(interval);
  }, [state, isDragging, showBubble]);

  // ══════════════════════════════════════
  // 드래그 핸들러
  // ══════════════════════════════════════
  const handleMouseDown = useCallback((e: React.MouseEvent) => {
    setIsDragging(true);
    didDrag.current = false;
    dragOffset.current = {
      x: e.clientX - position.x,
      y: e.clientY - position.y,
    };
  }, [position]);

  useEffect(() => {
    if (!isDragging) return;

    const handleMove = (e: MouseEvent) => {
      didDrag.current = true;
      setPosition({
        x: e.clientX - dragOffset.current.x,
        y: e.clientY - dragOffset.current.y,
      });
    };

    const handleUp = () => {
      setIsDragging(false);
      // 드래그 놓을 때 반응
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
  }, [isDragging, showBubble]);

  // ══════════════════════════════════════
  // 클릭 반응 (연속 클릭 감지)
  // ══════════════════════════════════════
  const handleClick = async () => {
    if (didDrag.current) return;

    try {
      await invoke<string>("click_cat");
    } catch (e) {
      console.error(e);
    }

    clickCount.current += 1;
    const count = clickCount.current;

    // 연속 클릭 리셋 타이머
    if (clickResetTimer.current) clearTimeout(clickResetTimer.current);
    clickResetTimer.current = setTimeout(() => {
      clickCount.current = 0;
    }, 3000);

    // 클릭 횟수에 따른 반응
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

    // 표정 복구
    setTimeout(() => {
      if (state === "idle") {
        setDisplayEmoji(behaviorEmoji[behavior]);
      }
    }, 2000);
  };

  // ══════════════════════════════════════
  // 렌더
  // ══════════════════════════════════════
  const isFlipped = direction === "left";

  return (
    <div
      className={`cat cat--${state} cat--${behavior} ${isDragging ? "cat--dragging" : ""}`}
      style={{
        transform: `translate3d(${position.x}px, ${position.y}px, 0)`,
      }}
      onMouseDown={handleMouseDown}
      onClick={handleClick}
    >
      <div
        className="cat__sprite"
        style={{ transform: isFlipped ? "scaleX(-1)" : "scaleX(1)" }}
      >
        <span className="cat__emoji">{displayEmoji}</span>
      </div>

      {/* 행동별 이펙트 */}
      {behavior === "nap" && state === "idle" && (
        <div className="cat__zzz">z z z</div>
      )}
      {behavior === "lick" && state === "idle" && (
        <div className="cat__effect">✨</div>
      )}
      {state === "celebrating" && <div className="cat__particles">✨🎉✨</div>}

      {/* 말풍선 */}
      {bubble && (
        <div className="cat__bubble" key={bubbleKey}>
          {bubble}
        </div>
      )}
    </div>
  );
}
