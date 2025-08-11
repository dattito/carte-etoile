import { Link } from 'react-router-dom';
import { UserButton } from '@clerk/clerk-react';

export default function Navbar() {
  return (
    <nav className="bg-gray-800 p-4 text-white">
      <div className="container mx-auto flex justify-between items-center">
        <Link to="/" className="text-xl font-bold">Business UI</Link>
        <div className="flex items-center">
          <Link to="/" className="mr-4">Home</Link>
          <Link to="/scan" className="mr-4">Scan</Link>
          <UserButton afterSignOutUrl="/login" />
        </div>
      </div>
    </nav>
  );
}