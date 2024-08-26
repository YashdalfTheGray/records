import 'preact/devtools';
import { h, render } from 'preact';
import { App } from './components/App';

const root = document.getElementById('root');

if (root) {
  // Because render appends (or really, prepends)
  // we clear out the app root and then render.
  root.innerHTML = '';
  render(<App />, root);
}
