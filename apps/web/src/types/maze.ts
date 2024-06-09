export type Position = [number, number];

export type TileType =
  | 'Empty'
  | 'Wall'
  | 'SoftWall'
  | 'Entrypoint'
  | 'Checkpoint'
  | 'Exit';

export type Tile =
  | { type: Exclude<TileType, 'Checkpoint'> }
  | { type: Extract<TileType, 'Checkpoint'>; level: number };
  
export type TileBoard = Tile[][];

export type MazeState = {
  colCount: number;
  rowCount: number;
  walls: Position[];
  softWalls:  Position[];
  entrypoints: Position[];
  checkpoints: { position: Position; level: number }[];
};
