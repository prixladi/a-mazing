// This is ugly as fuck but 'className' variable forces tailwind extension to work
export const getCheckpointColors = (level: number) => {
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
