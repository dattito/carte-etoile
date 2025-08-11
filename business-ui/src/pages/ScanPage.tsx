import QrScanner from '../components/QrScanner';

export default function ScanPage() {
  return (
    <div>
      <h1 className="text-3xl font-bold text-center my-8">Scan Wallet Pass QR Code</h1>
      <QrScanner />
    </div>
  );
}
