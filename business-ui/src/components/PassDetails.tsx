import { useEffect, useState } from 'react';
import { useAuth } from '@clerk/clerk-react';
import { getLoyalityPass, addPoints, redeemBonus, type LoyalityPass } from '../api';

interface PassDetailsProps {
  serialNumber: string;
}

export default function PassDetails({ serialNumber }: PassDetailsProps) {
  const { getToken } = useAuth();
  const [pass, setPass] = useState<LoyalityPass | null>(null);
  const [pointsToAdd, setPointsToAdd] = useState(0);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');

  const fetchPass = async () => {
    try {
      setLoading(true);
      const token = await getToken();
      if (!token) throw new Error("Not authenticated");
      const response = await getLoyalityPass(serialNumber, token);
      setPass(response.data);
    } catch (err) {
      setError('Failed to fetch pass details.');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchPass();
  }, [serialNumber]);

  const handleAddPoints = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const token = await getToken();
      if (!token) throw new Error("Not authenticated");
      await addPoints(serialNumber, pointsToAdd, token);
      setPointsToAdd(0);
      fetchPass(); // Refresh pass details
      alert('Points added successfully!');
    } catch (err) {
      alert('Failed to add points.');
      console.error(err);
    }
  };

  const handleRedeemBonus = async () => {
    try {
      const token = await getToken();
      if (!token) throw new Error("Not authenticated");
      await redeemBonus(serialNumber, token);
      fetchPass(); // Refresh pass details
      alert('Bonus redeemed successfully!');
    } catch (err) {
      alert('Failed to redeem bonus.');
      console.error(err);
    }
  };

  if (loading) return <div className="text-center">Loading...</div>;
  if (error) return <div className="text-center text-red-500">{error}</div>;
  if (!pass) return <div className="text-center">No pass data found.</div>;

  return (
    <div className="max-w-md mx-auto bg-white rounded-lg shadow-md p-6">
      <h2 className="text-2xl font-bold mb-4">{pass.passHolderName}</h2>
      <div className="space-y-2">
        <p><strong>Serial Number:</strong> {pass.serialNumber}</p>
        <p><strong>Total Points:</strong> {pass.totalPoints}</p>
        <p><strong>Current Points:</strong> {pass.currentPoints}</p>
        <p><strong>Already Redeemed:</strong> {pass.alreadyRedeemed}</p>
        <p><strong>Last Used:</strong> {pass.lastUsedAt ? new Date(pass.lastUsedAt).toLocaleString() : 'N/A'}</p>
      </div>

      <form onSubmit={handleAddPoints} className="mt-6">
        <h3 className="text-xl font-semibold mb-2">Add Points</h3>
        <input
          type="number"
          value={pointsToAdd}
          onChange={(e) => setPointsToAdd(parseInt(e.target.value, 10))}
          className="w-full p-2 border rounded mb-2"
          min="0"
        />
        <button type="submit" className="w-full p-2 text-white bg-blue-500 rounded hover:bg-blue-600">
          Add Points
        </button>
      </form>

      <div className="mt-6">
        <h3 className="text-xl font-semibold mb-2">Redeem Bonus</h3>
        <button onClick={handleRedeemBonus} className="w-full p-2 text-white bg-green-500 rounded hover:bg-green-600">
          Redeem Bonus
        </button>
      </div>
    </div>
  );
}