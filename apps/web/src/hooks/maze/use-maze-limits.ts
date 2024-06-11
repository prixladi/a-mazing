import { useMemo } from 'react';

import { MazeConfiguration, MazeMutations } from '~/types/maze';

type MazeLimits = {
  softWallsRemaining: number;
};

type Props = {
  configuration: MazeConfiguration;
  mazeMutations: MazeMutations;
};

export const useMazeLimits = ({ configuration, mazeMutations }: Props) => {
  return useMemo<MazeLimits>(
    () => ({
      softWallsRemaining:
        configuration.maxSoftWallCount - mazeMutations.softWalls.length,
    }),
    [configuration.maxSoftWallCount, mazeMutations.softWalls]
  );
};
