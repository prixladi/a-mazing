export type Position = [number, number];

export type TileKind =
  | 'Empty'
  | 'Wall'
  | 'SoftWall'
  | 'Entrypoint'
  | 'Checkpoint'
  | 'Exit';

export type Tile =
  | { kind: Exclude<TileKind, 'Checkpoint'> }
  | { kind: Extract<TileKind, 'Checkpoint'>; level: number };
  
export type TileBoard = Tile[][];

export type MazeState = {
  colCount: number;
  rowCount: number;
  walls: Position[];
  softWalls:  Position[];
  entrypoints: Position[];
  checkpoints: { position: Position; level: number }[];
};
