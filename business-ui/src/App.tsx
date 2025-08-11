import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import { SignedIn, SignedOut, SignIn } from '@clerk/clerk-react';
import HomePage from './pages/HomePage';
import ScanPage from './pages/ScanPage';
import PassPage from './pages/PassPage';
import Navbar from './components/Navbar';

function AppLayout() {
  return (
    <>
      <Navbar />
      <main className="container mx-auto p-4">
        <Routes>
          <Route path="/" element={<HomePage />} />
          <Route path="/scan" element={<ScanPage />} />
          <Route path="/pass/:serialNumber" element={<PassPage />} />
        </Routes>
      </main>
    </>
  )
}

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/login" element={<div className="flex items-center justify-center h-screen"><SignIn /></div>} />
        <Route path="/*" element={
          <>
            <SignedIn>
              <AppLayout />
            </SignedIn>
            <SignedOut>
              <Navigate to="/login" />
            </SignedOut>
          </>
        } />
      </Routes>
    </Router>
  );
}

export default App;