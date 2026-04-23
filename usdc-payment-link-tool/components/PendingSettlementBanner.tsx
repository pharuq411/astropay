type Props = { status: string };

export function PendingSettlementBanner({ status }: Props) {
  if (status === 'paid') {
    return (
      <div className="card stack" style={{ borderLeft: '4px solid #f59e0b' }}>
        <p><strong>Payment received — settlement pending</strong></p>
        <p className="muted">
          Your USDC payment has been detected on Stellar. The merchant will receive
          their net payout in a separate settlement transaction, typically within
          minutes. This invoice will update to <strong>settled</strong> once that
          transaction confirms.
        </p>
      </div>
    );
  }

  if (status === 'settled') {
    return (
      <div className="card stack" style={{ borderLeft: '4px solid #22c55e' }}>
        <p><strong>Settled</strong></p>
        <p className="muted">
          Payment has been received and the merchant payout has been confirmed on-chain.
        </p>
      </div>
    );
  }

  return null;
}
