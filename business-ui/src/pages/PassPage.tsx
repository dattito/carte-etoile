import { useParams } from 'react-router-dom';
import PassDetails from '../components/PassDetails';

export default function PassPage() {
  const { serialNumber } = useParams<{ serialNumber: string }>();

  if (!serialNumber) {
    return <div className="text-center text-red-500">Serial number not found in URL.</div>;
  }

  return (
    <div>
      <h1 className="text-3xl font-bold text-center my-8">Pass Details</h1>
      <PassDetails serialNumber={serialNumber} />
    </div>
  );
}
