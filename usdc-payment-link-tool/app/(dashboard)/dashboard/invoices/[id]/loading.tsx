import { SkeletonBlock } from '@/components/Skeleton';

export default function InvoiceDetailLoading() {
  return (
    <div className="grid two">
      <div className="card stack">
        <SkeletonBlock width="80px" height="28px" />
        <SkeletonBlock width="65%" height="36px" />
        <SkeletonBlock width="40%" />
        <SkeletonBlock width="55%" />
        <SkeletonBlock width="50%" />
        <SkeletonBlock width="50%" />
        <SkeletonBlock width="80%" />
        <SkeletonBlock width="60%" />
        <SkeletonBlock width="45%" />
        <SkeletonBlock width="45%" />
        <div className="row">
          <SkeletonBlock width="130px" height="44px" />
          <SkeletonBlock width="80px" height="44px" />
        </div>
      </div>
      <div className="card stack">
        <SkeletonBlock width="40px" height="28px" />
        <SkeletonBlock width="280px" height="280px" />
      </div>
    </div>
  );
}
