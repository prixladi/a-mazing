import { Mazer, MazerConfig, MazerPosition, MazerCheckpoint } from 'mazer';
import { useEffect, useState } from 'react';

import { MazeConfig, MazeMutations } from '~/types/maze';
import { useMazerInitialization } from './base';
import { Position } from '~/types/tile';

export const useMazer = (
  mazeConfig: MazeConfig,
  mazeMutations: MazeMutations
) => {
  const [mazer, setMazer] = useState<Mazer | null>(null);
  const [score, setScore] = useState<number | null>(null);
  const [path, setPath] = useState<Position[] | null>(null);

  const { isMazerReady } = useMazerInitialization();

  useEffect(() => {
    if (!isMazerReady) return;

    const walls = mazeConfig.walls.map(([x, y]) => MazerPosition.new(x, y));
    const entrypoints = mazeConfig.entrypoints.map(([x, y]) =>
      MazerPosition.new(x, y)
    );
    const checkpoints = mazeConfig.checkpoints.map(
      ({ position: [x, y], level }) => {
        const position = MazerPosition.new(x, y);
        return MazerCheckpoint.new(position, level);
      }
    );

    const options = MazerConfig.new(
      mazeConfig.colCount,
      mazeConfig.rowCount,
      mazeConfig.maxSoftWallCount,
      entrypoints,
      checkpoints,
      walls
    );

    const mazer = Mazer.new(options);
    setMazer(mazer);
  }, [mazeConfig, isMazerReady]);

  useEffect(() => {
    if (!mazer) return;

    const softWalls = mazeMutations.softWalls.map(([x, y]) =>
      MazerPosition.new(x, y)
    );
    const result = mazer.run(softWalls);

    setScore(result?.score ?? null);
    setPath(result?.path?.map((pos) => [pos.x, pos.y]) ?? null);
  }, [JSON.stringify(mazeMutations), mazer]);

  return { score, path };
};
