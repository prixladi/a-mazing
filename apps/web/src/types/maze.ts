import { Position } from './tile';

export type MazeConfiguration = {
  colCount: number;
  rowCount: number;
  maxSoftWallCount: number;
  walls: Position[];
  entrypoints: Position[];
  checkpoints: { position: Position; level: number }[];
};

export type MazeMutations = {
  softWalls: Position[];
};

export type MazeState = MazeConfiguration & MazeMutations;
