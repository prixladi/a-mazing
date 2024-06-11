import { useMemo } from 'react';

import { MazeConfiguration } from '~/types/maze';
import {
  builderToTileBoard,
  createEmptyTileBoardBuilder,
} from '~/utils/tile-board';

export const useConfiguredMazeBoard = (configuration: MazeConfiguration) => {
  const { exitCheckpoints, otherCheckpoints } = useMemo(() => {
    const sortedCheckpoints = configuration.checkpoints
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
  }, [JSON.stringify(configuration.checkpoints)]);

  return useMemo(() => {
    const builder = createEmptyTileBoardBuilder(configuration);

    for (let position of configuration.walls) {
      const [x, y] = position;
      builder[x][y] = { kind: 'Wall' };
    }
    for (let position of configuration.entrypoints) {
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
  }, [configuration, exitCheckpoints, otherCheckpoints]);
};
