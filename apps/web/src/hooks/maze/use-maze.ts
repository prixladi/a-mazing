import { useCallback, useState } from 'react';

import { MazeConfiguration, MazeMutations } from '~/types/maze';
import { Position, TileKind } from '~/types/tile';

import { useConfiguredMazeBoard } from './use-configured-maze-board';
import { useMutatedMazeBoard } from './use-mutated-maze-board';
import { useMazeLimits } from './use-maze-limits';

export const useMaze = (initConfiguration: MazeConfiguration) => {
  const [configuration] = useState(initConfiguration);
  const [mazeMutations, setMazeMutations] = useState<MazeMutations>({
    softWalls: [],
  });

  const mazeLimits = useMazeLimits({ configuration, mazeMutations });
  const configuredBoard = useConfiguredMazeBoard(configuration);
  const mutatedMazeBoard = useMutatedMazeBoard({
    configuredBoard,
    mazeMutations,
  });

  const mutateMazePosition = useCallback(
    ([x, y]: Position, type: Extract<TileKind, 'SoftWall' | 'Empty'>) => {
      setMazeMutations((oldMutations) => {
        const softWalls = oldMutations.softWalls.filter(
          (position) => position[0] !== x || position[1] !== y
        );

        if (
          type === 'SoftWall' &&
          softWalls.length < configuration.maxSoftWallCount
        ) {
          softWalls.push([x, y]);
        }

        return { ...oldMutations, softWalls };
      });
    },
    [configuration.maxSoftWallCount]
  );

  return {
    mutateMazePosition,
    mazeBoard: mutatedMazeBoard,
    mazeMutations,
    mazeLimits,
  };
};
