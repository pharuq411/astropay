'use client';

import { useEffect, useState } from 'react';
import { isConnected, requestAccess, signTransaction } from '@stellar/freighter-api';
import { PendingSettlementBanner } from './PendingSettlementBanner';

type Props = { invoiceId: string; status: string; };

export function PayWithFreighter({ invoiceId, status: initialStatus }: Props) {
  const [address, setAddress] = useState('');
  const [status, setStatus] = useState(initialStatus);
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    let timer: ReturnType<typeof setTimeout> | null = null;
    async function poll() {
      const res = await fetch(`/api/invoices/${invoiceId}/status`, { cache: 'no-store' });
      const data = await res.json();
      if (res.ok) {
        setStatus(data.status);
        if (['pending', 'paid', 'processing'].includes(data.status)) timer = setTimeout(poll, 5000);
      }
    }
    poll();
    return () => { if (timer) clearTimeout(timer); };
  }, [invoiceId]);

  async function connect() {
    const connected = await isConnected();
    if (!connected.isConnected) {
      setError('Freighter is not connected in this browser.');
      return '';
    }
    const res = await requestAccess();
    if ('address' in res && res.address) {
      setAddress(res.address);
      return res.address;
    }
    const message = 'error' in res && res.error ? String((res as any).error?.message || res.error) : 'Unable to access Freighter';
    setError(message);
    return '';
  }

  async function pay() {
    setLoading(true); setError('');
    try {
      const payer = address || await connect();
      if (!payer) throw new Error('Missing payer public key');
      const buildRes = await fetch(`/api/invoices/${invoiceId}/checkout`, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ mode: 'build-xdr', payer }) });
      const buildData = await buildRes.json();
      if (!buildRes.ok) throw new Error(buildData.error || 'Failed to build transaction');
      const signed = await signTransaction(buildData.xdr, { networkPassphrase: buildData.networkPassphrase });
      const signedXdr = 'signedTxXdr' in signed ? signed.signedTxXdr : '';
      if (!signedXdr) throw new Error('Freighter did not return a signed transaction');
      const submitRes = await fetch(`/api/invoices/${invoiceId}/checkout`, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ mode: 'submit-xdr', signedXdr }) });
      const submitData = await submitRes.json();
      if (!submitRes.ok) throw new Error(submitData.error || 'Failed to submit transaction');
      setStatus('processing');
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Payment failed');
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="card stack">
      <div className="badge">Freighter checkout</div>
      <p className="muted">Status: <strong>{status}</strong></p>
      <div className="row">
        <button className="button secondary" onClick={connect}>Connect Freighter</button>
        <button className="button" onClick={pay} disabled={loading || ['paid','settled','expired'].includes(status)}>{loading ? 'Processing...' : 'Pay now'}</button>
      </div>
      {address ? <p className="muted">Payer: <span className="mono">{address}</span></p> : null}
      {error ? <p className="error">{error}</p> : null}
      <PendingSettlementBanner status={status} />
    </div>
  );
}
