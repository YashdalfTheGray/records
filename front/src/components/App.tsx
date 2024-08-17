import { h } from 'preact';
import { ThemeProvider, createTheme } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';

import { Hello } from './Hello';
import NavBar from './NavBar';

const appTheme = createTheme({
  palette: {
    mode: 'dark',
  },
});

export const App = () => {
  return (
    <ThemeProvider theme={appTheme}>
      <CssBaseline enableColorScheme />
      <NavBar />
      <h1>Welcome to Preact with TypeScript!</h1>
      <Hello name="World" />
    </ThemeProvider>
  );
};
