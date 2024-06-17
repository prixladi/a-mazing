import { useMemo } from 'react';

import { MazeConfig, MazeMutations } from '~/types/maze';

type MazeLimits = {
  softWallsRemaining: number;
};

type Props = {
  config: MazeConfig;
  mazeMutations: MazeMutations;
};

export const useMazeLimits = ({ config, mazeMutations }: Props) => {
  return useMemo<MazeLimits>(
    () => ({
      softWallsRemaining:
        config.maxSoftWallCount - mazeMutations.softWalls.length,
    }),
    [config.maxSoftWallCount, mazeMutations.softWalls]
  );
};
