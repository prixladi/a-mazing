import { useCallback, useState } from 'react';

import { MazeConfig, MazeMutations } from '~/types/maze';
import { Position, TileKind } from '~/types/tile';

import { useConfiguredMazeBoard } from './use-configured-maze-board';
import { useMutatedMazeBoard } from './use-mutated-maze-board';
import { useMazeLimits } from './use-maze-limits';

const defaultMazeMutations = {
  softWalls: [],
};

export const useMaze = (config: MazeConfig) => {
  const [mazeMutations, setMazeMutations] =
    useState<MazeMutations>(defaultMazeMutations);

  const mazeLimits = useMazeLimits({ config: config, mazeMutations });
  const configuredBoard = useConfiguredMazeBoard(config);
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
          softWalls.length < config.maxSoftWallCount
        ) {
          softWalls.push([x, y]);
        }

        return { ...oldMutations, softWalls };
      });
    },
    [config.maxSoftWallCount]
  );

  const clearMutations = useCallback(() => {
    setMazeMutations(defaultMazeMutations);
  }, []);

  return {
    mutateMazePosition,
    clearMutations,
    mazeBoard: mutatedMazeBoard,
    mazeMutations,
    mazeLimits,
  };
};
