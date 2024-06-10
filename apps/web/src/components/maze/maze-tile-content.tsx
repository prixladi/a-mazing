import clsx from 'clsx';
import { LogIn, LogOut } from 'lucide-react';

import { Tile } from '~/types/maze';

type Props = { tile: Tile };

export const MazeTileContent: React.FC<Props> = ({ tile }) => {
  if (tile.kind === 'Entrypoint') return <EntrypointTile />;
  if (tile.kind === 'Exit') return <ExitTile />;
  if (tile.kind === 'Wall') return <WallTile />;
  if (tile.kind === 'SoftWall') return <SoftWallTile />;
  if (tile.kind === 'Checkpoint') return <CheckpointTile level={tile.level} />;
  if (tile.kind === 'Empty') return <EmptyTile />;
  throw new Error(`Unable to render content for '${tile.kind}' tile.`);
};

const EntrypointTile: React.FC = () => (
  <div className='w-full flex h-full justify-center items-center bg-emerald-50 shadow-md'>
    <LogIn className='text-emerald-500' />
  </div>
);

const ExitTile: React.FC = () => (
  <div className='w-full flex h-full justify-center items-center bg-red-100 shadow-md'>
    <LogOut className='text-red-600' />
  </div>
);

const WallTile: React.FC = () => (
  <div className='w-full h-full bg-amber-900 rounded-md' />
);

const SoftWallTile: React.FC = () => (
  <div className='w-full h-full bg-yellow-600 rounded-md' />
);

const CheckpointTile: React.FC<{ level: number }> = ({ level }) => (
  <div className='w-full flex h-full bg-slate-100'>
    <span
      className={clsx(
        'w-full flex h-full justify-center items-center rounded-full font-bold text-lg',
        getCheckpointColor(level)
      )}
    >
      {level}
    </span>
  </div>
);

const EmptyTile: React.FC = () => (
  <div className='w-full h-full bg-slate-100' />
);

// This is ugly as fuck but 'className' variable forces tailwind extension to work
const getCheckpointColor = (level: number) => {
  let className: string;
  if (level === 1) className = 'text-white bg-emerald-300';
  else if (level === 2) className = 'text-white bg-yellow-400';
  else if (level === 3) className = 'text-white bg-red-600';
  else if (level === 4) className = 'text-white bg-blue-400';
  else if (level === 5) className = 'text-white bg-slate-600';
  else if (level === 6) className = 'text-white bg-teal-700';
  else if (level === 7) className = 'text-white bg-teal-200';
  else if (level === 8) className = 'text-white bg-sky-500';
  else if (level === 9) className = 'text-white bg-blue-900';
  else if (level === 10) className = 'text-white bg-violet-600';
  else className = 'text-white bg-black';

  return className;
};
