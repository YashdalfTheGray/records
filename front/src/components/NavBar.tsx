import { h } from 'preact';
import AppBar from '@mui/material/AppBar';
import Box from '@mui/material/Box';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import ThemeModeSwitch from './ThemeModeSwitch';

function NavBar() {
  return (
    <Box sx={{ flexGrow: 1 }}>
      <AppBar position="static">
        <Toolbar>
          <Typography variant="h6" sx={{ flexGrow: 1 }}>
            Records
          </Typography>
          <ThemeModeSwitch />
        </Toolbar>
      </AppBar>
    </Box>
  );
}

export default NavBar;
