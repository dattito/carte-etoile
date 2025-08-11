import { Link } from 'react-router-dom';
import { useAuth } from '@clerk/clerk-react';
import { createPass } from '../api';

export default function HomePage() {
  const { getToken } = useAuth();

  const handleCreatePass = async () => {
    try {
      const token = await getToken();
      if (!token) throw new Error("Not authenticated");
      const response = await createPass(token);
      const url = window.URL.createObjectURL(new Blob([response.data]));
      const link = document.createElement('a');
      link.href = url;
      link.setAttribute('download', 'pass.pkpass');
      document.body.appendChild(link);
      link.click();
      link.remove();
    } catch (error) {
      console.error('Error creating pass:', error);
      alert('Failed to create pass.');
    }
  };

  const logToken = async () => {
    const token = await getToken();
    console.log(token);
    alert('Token has been logged to the console.');
  };

  return (
    <div className="text-center">
      <h1 className="text-4xl font-bold my-8">Business Dashboard</h1>
      <div className="space-x-4">
        <Link to="/scan" className="px-6 py-3 text-white bg-green-500 rounded-lg hover:bg-green-600">
          Scan QR Code
        </Link>
        <button onClick={handleCreatePass} className="px-6 py-3 text-white bg-blue-500 rounded-lg hover:bg-blue-600">
          Create New Pass
        </button>
        <button onClick={logToken} className="px-6 py-3 text-white bg-gray-500 rounded-lg hover:bg-gray-600">
          Log JWT Token
        </button>
      </div>
    </div>
  );
}
