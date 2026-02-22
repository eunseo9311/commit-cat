import { useEffect } from "react";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

/**
 * Tauri 이벤트를 React 라이프사이클에 바인딩하는 훅
 */
export function useTauriEvent<T>(
  eventName: string,
  handler: (payload: T) => void
) {
  useEffect(() => {
    let unlisten: UnlistenFn;

    listen<T>(eventName, (event) => {
      handler(event.payload);
    }).then((fn) => {
      unlisten = fn;
    });

    return () => {
      if (unlisten) unlisten();
    };
  }, [eventName, handler]);
}
