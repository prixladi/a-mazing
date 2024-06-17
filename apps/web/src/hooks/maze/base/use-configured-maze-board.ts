import { useMemo } from 'react';

import { MazeConfig } from '~/types/maze';
import {
  builderToTileBoard,
  createEmptyTileBoardBuilder,
} from '~/utils/tile-board';

export const useConfiguredMazeBoard = (config: MazeConfig) => {
  const { exitCheckpoints, otherCheckpoints } = useMemo(() => {
    const sortedCheckpoints = config.checkpoints
      .slice()
      .sort((a, b) => b.level - a.level);

    const [first] = sortedCheckpoints;

    const exitCheckpoints = sortedCheckpoints.filter(
      ({ level }) => level === first.level
    );
    const otherCheckpoints = sortedCheckpoints.filter(
      ({ level }) => level !== first.level
    );

    return { exitCheckpoints, otherCheckpoints };
  }, [JSON.stringify(config.checkpoints)]);

  return useMemo(() => {
    const builder = createEmptyTileBoardBuilder(config);

    for (let position of config.walls) {
      const [x, y] = position;
      builder[x][y] = { kind: 'Wall' };
    }
    for (let position of config.entrypoints) {
      const [x, y] = position;
      builder[x][y] = { kind: 'Entrypoint' };
    }
    for (let { position } of exitCheckpoints) {
      const [x, y] = position;
      builder[x][y] = { kind: 'Exit' };
    }
    for (let { level, position } of otherCheckpoints) {
      const [x, y] = position;
      builder[x][y] = { kind: 'Checkpoint', level };
    }

    return builderToTileBoard(builder);
  }, [config, exitCheckpoints, otherCheckpoints]);
};
