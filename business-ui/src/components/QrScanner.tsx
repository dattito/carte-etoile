import { useEffect, useRef } from 'react';
import { Html5QrcodeScanner } from 'html5-qrcode';
import { useNavigate } from 'react-router-dom';

export default function QrScanner() {
  const scannerRef = useRef<HTMLDivElement>(null);
  const navigate = useNavigate();

  useEffect(() => {
    if (!scannerRef.current) return;

    const scanner = new Html5QrcodeScanner(
      scannerRef.current.id,
      {
        fps: 10,
        qrbox: { width: 250, height: 250 },
      },
      false
    );

    const onScanSuccess = (decodedText: string) => {
      // Assuming the decoded text is the serial number
      // In a real scenario, you might need to parse a URL to get the serial number
      scanner.clear();
      navigate(`/pass/${decodedText}`);
    };

    scanner.render(onScanSuccess, undefined);

    return () => {
      scanner.clear();
    };
  }, [navigate]);

  return <div id="qr-reader" ref={scannerRef} className="w-full md:w-1/2 mx-auto"></div>;
}
