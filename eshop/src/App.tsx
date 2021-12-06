import { useState } from 'react';
import './App.css';
import { PatientsList } from './components/PatientsList';
import { PatientForm } from './components/PatientForm';

function App() {
  const [count, setCount] = useState(0)

  return (
    <div className="App">
      <div className="w-screen h-screen flex">
        <PatientsList />
        <PatientForm />
      </div>
    </div>
  )
}

export default App
