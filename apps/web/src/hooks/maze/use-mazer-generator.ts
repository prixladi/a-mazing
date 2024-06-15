import { useCallback } from 'react';

import { useMazerInitialization } from './use-mazer-initialization';
import { Mazer } from 'mazer';
import { MazeConfig } from '~/types/maze';

export const useMazerGenerator = () => {
  const { isMazerReady } = useMazerInitialization();

  const generateConfig = useCallback((): MazeConfig => {
    const mazerConfig = Mazer.generateConfig();

    return {
      colCount: mazerConfig.colCount,
      rowCount: mazerConfig.rowCount,
      maxSoftWallCount: mazerConfig.maxSoftWallCount,
      entrypoints: mazerConfig.entrypoints.map((position) => [
        position.x,
        position.y,
      ]),
      checkpoints: mazerConfig.checkpoints.map((checkpoint) => ({
        position: [checkpoint.position.x, checkpoint.position.y],
        level: checkpoint.level,
      })),
      walls: mazerConfig.walls.map((position) => [position.x, position.y]),
    };
  }, [isMazerReady]);

  return { generateConfig };
};
