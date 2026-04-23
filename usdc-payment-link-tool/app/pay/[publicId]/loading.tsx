import { SkeletonBlock } from '@/components/Skeleton';

export default function PayLoading() {
  return (
    <div className="grid two">
      <div className="card stack">
        <SkeletonBlock width="120px" height="28px" />
        <SkeletonBlock width="70%" height="36px" />
        <SkeletonBlock width="50%" />
        <SkeletonBlock width="40%" />
        <SkeletonBlock width="60%" />
        <SkeletonBlock width="80%" />
        <SkeletonBlock width="90%" />
      </div>
      <div className="stack">
        <div className="card stack">
          <SkeletonBlock width="100px" height="28px" />
          <SkeletonBlock width="280px" height="280px" />
        </div>
        <div className="card stack">
          <SkeletonBlock width="140px" height="28px" />
          <SkeletonBlock height="44px" />
          <SkeletonBlock height="44px" />
        </div>
      </div>
    </div>
  );
}
