import { Position } from './tile';

export type MazeConfig = {
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

export type MazeState = MazeConfig & MazeMutations;
