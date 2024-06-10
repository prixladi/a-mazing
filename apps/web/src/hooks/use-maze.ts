import { useCallback, useMemo, useState } from 'react';

import { MazeState, Position, Tile, TileBoard } from '~/types/maze';

export const useMaze = (initialState: MazeState) => {
  const [mazeState, setMazeState] = useState(initialState);

  const mazeTiles = useMemo(() => {
    const tileBoard: TileBoard = [];
    for (let x = 0; x < mazeState.colCount; x++) {
      const row: Tile[] = [];
      for (let y = 0; y < mazeState.rowCount; y++) {
        row.push({ kind: 'Empty' });
      }
      tileBoard.push(row);
    }

    for (let [x, y] of mazeState.walls) {
      tileBoard[x][y] = { kind: 'Wall' };
    }
    for (let [x, y] of mazeState.softWalls) {
      tileBoard[x][y] = { kind: 'SoftWall' };
    }
    for (let [x, y] of mazeState.entrypoints) {
      tileBoard[x][y] = { kind: 'Entrypoint' };
    }

    const sortedCheckpoints = mazeState.checkpoints
      .slice()
      .sort((a, b) => b.level - a.level);

    if (sortedCheckpoints.length > 0) {
      const [first] = sortedCheckpoints;

      const exitCheckpoints = sortedCheckpoints.filter(
        ({ level }) => level === first.level
      );
      for (let {
        position: [x, y],
      } of exitCheckpoints) {
        tileBoard[x][y] = { kind: 'Exit' };
      }

      const otherCheckpoints = sortedCheckpoints.filter(
        ({ level }) => level !== first.level
      );
      for (let {
        level,
        position: [x, y],
      } of otherCheckpoints) {
        tileBoard[x][y] = { kind: 'Checkpoint', level };
      }
    }
    return tileBoard;
  }, [JSON.stringify(mazeState)]);

  const setTileOnPosition = useCallback(([x, y]: Position, tile: Tile) => {
    setMazeState((oldOptions) => {
      const walls = oldOptions.walls.filter(
        (position) => position[0] !== x || position[1] !== y
      );
      const entrypoints = oldOptions.entrypoints.filter(
        (position) => position[0] !== x || position[1] !== y
      );
      const checkpoints = oldOptions.checkpoints.filter(
        ({ position }) => position[0] !== x || position[1] !== y
      );
      const softWalls = oldOptions.softWalls.filter(
        (position) => position[0] !== x || position[1] !== y
      );

      if (tile.kind === 'Wall') walls.push([x, y]);
      if (tile.kind === 'Entrypoint') entrypoints.push([x, y]);
      if (tile.kind === 'Checkpoint')
        checkpoints.push({ position: [x, y], level: tile.level });
      if (tile.kind === 'SoftWall') softWalls.push([x, y]);

      return {
        ...oldOptions,
        walls,
        softWalls,
        entrypoints,
        checkpoints,
      };
    });
  }, []);

  return { setTileOnPosition, mazeTiles, mazeState };
};
