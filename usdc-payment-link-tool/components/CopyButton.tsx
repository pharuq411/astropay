'use client';

import { useState } from 'react';

export function CopyButton({ value }: { value: string }) {
  const [copied, setCopied] = useState(false);

  async function copy() {
    try {
      if (!navigator?.clipboard) throw new Error('Clipboard API unavailable');
      await navigator.clipboard.writeText(value);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch {
      // fallback: select a temporary textarea
      const el = document.createElement('textarea');
      el.value = value;
      el.style.position = 'fixed';
      el.style.opacity = '0';
      document.body.appendChild(el);
      el.select();
      document.execCommand('copy');
      document.body.removeChild(el);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    }
  }

  return (
    <button
      onClick={copy}
      className="button secondary"
      style={{ padding: '2px 10px', fontSize: '12px', borderRadius: '8px' }}
      title="Copy to clipboard"
    >
      {copied ? '✓ Copied' : 'Copy'}
    </button>
  );
}
