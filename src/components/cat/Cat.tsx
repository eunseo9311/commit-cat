import { useState, useRef, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useCatStore } from "../../stores/catStore";
import "./Cat.css";

/**
 * 🐱 데스크탑 고양이 컴포넌트
 *
 * 기능:
 * - 화면 위를 돌아다님 (idle 모드)
 * - 드래그로 위치 이동
 * - 클릭 시 반응
 * - 상태에 따라 스프라이트/애니메이션 변경
 */
export function Cat() {
  const { state, mood } = useCatStore();

  // ── 위치 & 드래그 ──
  const [position, setPosition] = useState({ x: 100, y: 100 });
  const [isDragging, setIsDragging] = useState(false);
  const dragOffset = useRef({ x: 0, y: 0 });

  // ── Idle 이동 (자동 배회) ──
  const [direction, setDirection] = useState<"left" | "right">("right");

  // ── 말풍선 ──
  const [bubble, setBubble] = useState<string | null>(null);

  useEffect(() => {
    if (state !== "idle" || isDragging) return;

    const interval = setInterval(() => {
      setPosition((prev) => {
        const speed = 1;
        let newX = prev.x + (direction === "right" ? speed : -speed);

        // 화면 경계에서 방향 전환
        if (newX > window.innerWidth - 64) {
          setDirection("left");
          newX = window.innerWidth - 64;
        } else if (newX < 0) {
          setDirection("right");
          newX = 0;
        }

        return { ...prev, x: newX };
      });
    }, 50);

    return () => clearInterval(interval);
  }, [state, direction, isDragging]);

  // ── 드래그 핸들러 ──
  const handleMouseDown = useCallback((e: React.MouseEvent) => {
    setIsDragging(true);
    dragOffset.current = {
      x: e.clientX - position.x,
      y: e.clientY - position.y,
    };
  }, [position]);

  useEffect(() => {
    if (!isDragging) return;

    const handleMove = (e: MouseEvent) => {
      setPosition({
        x: e.clientX - dragOffset.current.x,
        y: e.clientY - dragOffset.current.y,
      });
    };

    const handleUp = () => setIsDragging(false);

    window.addEventListener("mousemove", handleMove);
    window.addEventListener("mouseup", handleUp);
    return () => {
      window.removeEventListener("mousemove", handleMove);
      window.removeEventListener("mouseup", handleUp);
    };
  }, [isDragging]);

  // ── 클릭 반응 ──
  const handleClick = async () => {
    if (isDragging) return;
    try {
      const response = await invoke<string>("click_cat");
      setBubble(response);
      setTimeout(() => setBubble(null), 1500);
    } catch (e) {
      console.error(e);
    }
  };

  // ── 상태별 이모지 (스프라이트 대체용 - MVP) ──
  const stateEmoji: Record<string, string> = {
    idle: "🐱",
    coding: "😺",
    celebrating: "🎉",
    frustrated: "😿",
    sleeping: "😴",
    tired: "🥱",
    interaction: "😻",
  };

  return (
    <div
      className={`cat cat--${state} ${isDragging ? "cat--dragging" : ""}`}
      style={{
        left: `${position.x}px`,
        top: `${position.y}px`,
      }}
      onMouseDown={handleMouseDown}
      onClick={handleClick}
    >
      <div className="cat__sprite">
        {/* MVP: 이모지 → v2: 실제 스프라이트 시트 */}
        <span className="cat__emoji">{stateEmoji[state] ?? "🐱"}</span>
      </div>
      {state === "sleeping" && <div className="cat__zzz">z z z</div>}
      {state === "celebrating" && <div className="cat__particles">✨</div>}
      {bubble && <div className="cat__bubble">{bubble}</div>}
    </div>
  );
}
