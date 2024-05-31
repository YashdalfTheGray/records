import { h } from 'preact';
import { Hello } from './Hello';

export const App = () => {
  return (
    <div>
      <h1>Welcome to Preact with TypeScript!</h1>
      <Hello name="World" />
    </div>
  );
};
