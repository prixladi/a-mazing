import { useMemo } from 'react';

import { MazeMutations } from '~/types/maze';
import { MazeTileBoard } from '~/types/tile';
import { builderToTileBoard, createTileBoardBuilder } from '~/utils/tile-board';

type Props = {
  configuredBoard: MazeTileBoard;
  mazeMutations: MazeMutations;
};

export const useMutatedMazeBoard = ({ configuredBoard, mazeMutations }: Props) =>
  useMemo(() => {
    const builder = createTileBoardBuilder(configuredBoard);

    for (let position of mazeMutations.softWalls) {
      const [x, y] = position;
      builder[x][y] = { kind: 'SoftWall' };
    }

    for (let { position, significancy } of mazeMutations.highlighted) {
      const [x, y] = position;
      builder[x][y] = { ...builder[x][y], highlighted: { significancy } };
    }

    return builderToTileBoard(builder);
  }, [configuredBoard, mazeMutations]);
