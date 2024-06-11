import init, {
  Mazer,
  MazerConfiguration,
  MazerPosition,
  MazerCheckpoint,
} from 'engine';
import { useEffect, useRef, useState } from 'react';

import { MazeConfiguration, MazeMutations } from '~/types/maze';

export const useMazer = (
  mazeConfiguration: MazeConfiguration,
  mazeMutations: MazeMutations
) => {
  const initStarted = useRef(false);

  const [mazer, setMazer] = useState<Mazer | null>(null);
  const [score, setScore] = useState<number | null>(null);

  useEffect(() => {
    if (initStarted.current) return;
    initStarted.current = true;

    init().then(() => {
      const walls = mazeConfiguration.walls.map(([x, y]) =>
        MazerPosition.new(x, y)
      );
      const entrypoints = mazeConfiguration.entrypoints.map(([x, y]) =>
        MazerPosition.new(x, y)
      );
      const checkpoints = mazeConfiguration.checkpoints.map(
        ({ position: [x, y], level }) => {
          const position = MazerPosition.new(x, y);
          return MazerCheckpoint.new(position, level);
        }
      );

      const options = MazerConfiguration.new(
        mazeConfiguration.colCount,
        mazeConfiguration.rowCount,
        mazeConfiguration.maxSoftWallCount,
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

    const softWalls = mazeMutations.softWalls.map(([x, y]) =>
      MazerPosition.new(x, y)
    );
    const result = mazer.run(softWalls);
    setScore(result?.score ?? null);
  }, [JSON.stringify(mazeMutations), mazer]);

  return { score };
};
