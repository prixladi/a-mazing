import { MazeConfig } from '~/types/maze';
import { Tile, MazeTileBoard, MazeTileBoardBuilder } from '~/types/tile';

export const createEmptyTileBoardBuilder = ({
  colCount,
  rowCount,
}: Pick<MazeConfig, 'colCount' | 'rowCount'>): MazeTileBoardBuilder => {
  const builder: MazeTileBoardBuilder = [];
  for (let x = 0; x < colCount; x++) {
    const row: Tile[] = [];
    for (let y = 0; y < rowCount; y++) {
      row.push({ kind: 'Empty' });
    }
    builder.push(row);
  }

  return builder;
};

export const createTileBoardBuilder = (
  tileBoard: MazeTileBoard
): MazeTileBoardBuilder => tileBoard.map((row) => row.map((tile) => ({ ...tile })));

export const builderToTileBoard = (builder: MazeTileBoardBuilder): MazeTileBoard =>
  Object.freeze(builder.map((row) => Object.freeze(row)));
