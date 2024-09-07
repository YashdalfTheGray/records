import { h } from 'preact';
import {
  ThemeProvider,
  createTheme,
  useColorScheme,
} from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';

import { Hello } from './Hello';
import NavBar from './NavBar';

const appTheme = createTheme({
  colorSchemes: {
    dark: true,
  },
});

export const App = () => {
  const { mode, setMode } = useColorScheme();
  if (!mode) {
    setMode('system');
  }

  return (
    <ThemeProvider theme={appTheme}>
      <CssBaseline enableColorScheme />
      <NavBar />
      <h1>Welcome to Preact with TypeScript!</h1>
      <Hello name="World" />
    </ThemeProvider>
  );
};
