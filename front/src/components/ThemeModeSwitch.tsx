import { useColorScheme } from '@mui/material/styles';
import IconButton from '@mui/material/IconButton';
import Tooltip from '@mui/material/Tooltip';
import Brightness4Icon from '@mui/icons-material/Brightness4';
import Brightness7Icon from '@mui/icons-material/Brightness7';

export default function ThemeModeSwitch() {
  const { mode, setMode } = useColorScheme();

  return (
    <Tooltip title={`${mode} mode`}>
      <IconButton
        aria-label={`${mode} mode`}
        onClick={() => setMode(mode === 'dark' ? 'light' : 'dark')}
      >
        {mode === 'light' ? <Brightness7Icon /> : <Brightness4Icon />}
      </IconButton>
    </Tooltip>
  );
}
