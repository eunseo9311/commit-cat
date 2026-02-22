import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { Cat } from "./components/cat/Cat";
import { useCatStore } from "./stores/catStore";

function App() {
  const { setState } = useCatStore();

  useEffect(() => {
    // 백엔드 이벤트 구독
    const unlisten = Promise.all([
      listen("cat:state-changed", (event) => {
        setState(event.payload as string);
      }),
      listen("git:new-commit", () => {
        setState("celebrating");
      }),
      listen("activity:fullscreen", (event) => {
        // 풀스크린이면 고양이 숨김
        const el = document.getElementById("root");
        if (el) {
          el.style.display = event.payload ? "none" : "block";
        }
      }),
    ]);

    return () => {
      unlisten.then((fns) => fns.forEach((fn) => fn()));
    };
  }, [setState]);

  return <Cat />;
}

export default App;
