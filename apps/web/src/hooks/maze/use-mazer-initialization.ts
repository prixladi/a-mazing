import init from 'mazer';
import { useEffect, useRef, useState } from 'react';

export const useMazerInitialization = () => {
  const initStarted = useRef(false);

  const [isMazerReady, setIsMazerReady] = useState(false);

  useEffect(() => {
    if (initStarted.current) return;
    initStarted.current = true;

    init().then(() => {
      setIsMazerReady(true);
    });
  }, []);

  return { isMazerReady };
};
