import { wrapConsole } from '../utils/ConsoleWrapper';
import { PhysicalPosition, PhysicalSize } from '@tauri-apps/api/dpi';
import { emitTo } from '@tauri-apps/api/event';
import { getCurrent } from '@tauri-apps/api/webviewWindow';

import './index.css';

async function Main() {
  wrapConsole();
  let view = getCurrent();

  view.listen('init', () => {
    getCurrent().show();
    document.body.addEventListener('mousemove', () => {
      emitTo('seelenweg', 'mouseenter');
    });
  });

  view.listen('resize', (event) => {
    const { width, height } = event.payload as any;
    getCurrent().setSize(new PhysicalSize(width, height));
  });

  view.listen('move', (event) => {
    const { x, y } = event.payload as any;
    getCurrent().setPosition(new PhysicalPosition(x, y));
  });
}

Main();
