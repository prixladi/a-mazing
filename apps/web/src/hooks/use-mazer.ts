import init, {
  Mazer,
  MazerOptions,
  MazerPosition,
  MazerCheckpoint,
} from 'engine';
import { useEffect, useRef, useState } from 'react';

import { MazeState } from '~/types/maze';

export const useMazer = (mazeState: MazeState) => {
  const initStarted = useRef(false);

  const [mazer, setMazer] = useState<Mazer | null>(null);

  const [score, setScore] = useState<number | null>(null);

  useEffect(() => {
    if (initStarted.current) return;
    initStarted.current = true;

    init().then(() => {
      const walls = mazeState.walls.map(([x, y]) => MazerPosition.new(x, y));
      const entrypoints = mazeState.entrypoints.map(([x, y]) =>
        MazerPosition.new(x, y)
      );
      const checkpoints = mazeState.checkpoints.map(
        ({ position: [x, y], level }) => {
          const position = MazerPosition.new(x, y);
          return MazerCheckpoint.new(position, level);
        }
      );

      const options = MazerOptions.new(
        mazeState.colCount,
        mazeState.rowCount,
        999,
        entrypoints,
        checkpoints,
        walls
      );

      const mazer = Mazer.new(options);
      setMazer(mazer);
    });
  }, []);

  useEffect(() => {
    if (!mazer) return;

    const softWalls = mazeState.softWalls.map(([x, y]) =>
      MazerPosition.new(x, y)
    );
    const result = mazer.run(softWalls);
    setScore(result?.score ?? null);
  }, [JSON.stringify(mazeState), mazer]);

  return { score };
};
