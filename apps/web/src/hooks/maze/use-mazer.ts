import {
  Mazer,
  MazerConfiguration,
  MazerPosition,
  MazerCheckpoint,
} from 'mazer';
import { useEffect, useState } from 'react';

import { MazeConfiguration, MazeMutations } from '~/types/maze';
import { useMazerInitialization } from './use-mazer-initialization';

export const useMazer = (
  mazeConfiguration: MazeConfiguration,
  mazeMutations: MazeMutations
) => {
  const [mazer, setMazer] = useState<Mazer | null>(null);
  const [score, setScore] = useState<number | null>(null);

  const { isMazerReady } = useMazerInitialization();

  useEffect(() => {
    if (!isMazerReady) return;

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
  }, [mazeConfiguration, isMazerReady]);

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
