type BlockProps = { width?: string; height?: string };

export function SkeletonBlock({ width = '100%', height = '18px' }: BlockProps) {
  return <div className="skeleton" style={{ width, height }} />;
}
