import { h } from 'preact';
import { Hello } from './Hello';
import CssBaseline from '@mui/material/CssBaseline';

export const App = () => {
  return (
    <div>
      <CssBaseline />
      <h1>Welcome to Preact with TypeScript!</h1>
      <Hello name="World" />
    </div>
  );
};
