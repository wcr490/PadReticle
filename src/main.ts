import { listen, Event} from "@tauri-apps/api/event";
import { invoke } from '@tauri-apps/api/tauri';

async function updateImage() {
    try {
        const base64Image = await invoke('update_crosshair_image') as string;
        const imgElement = document.getElementById('dynamic-image') as HTMLImageElement;
        imgElement.src = `data:image/png;base64,${base64Image}`;
    } catch (error) {
      alert('Failed to update image:');
      console.error('Failed to update image:', error);
    }
}
let is_crosshair_open = false;
window.addEventListener("DOMContentLoaded", () => {
  updateImage();
  const connection = document.querySelector.bind(document)("keep-alive")! as HTMLElement;
  const slider = document.getElementById('colorSlider') as HTMLInputElement;
  const bg = document.querySelector('.right-side') as HTMLInputElement;
  listen("keep-alive", function(event: Event<any>) {
    if (event.payload == true) {
      connection.classList.add('on');
    }
    if (event.payload == false) {
      connection.classList.remove('on');
    }
  });
  slider.addEventListener('input', () => {
    const hue = slider.value;
    bg.style.backgroundColor = `hsl(0, 0%, ${hue}%)`;
  });
  const click = document.getElementById("crosshair") as HTMLElement;
  click.addEventListener('click', () => {
    if (is_crosshair_open)
    {
      is_crosshair_open = false;
      invoke('close_crosshair');
    }
    else {
      is_crosshair_open = true;
      invoke('minimize_main_window');
      invoke('open_crosshair');
    }

  });
  const aim = document.querySelector.bind(document)("keep-aim")! as HTMLElement;
  listen("keep-aim", function(event: Event<any>) {
    if (event.payload == true) {
      if(is_crosshair_open) {
        invoke('close_crosshair');
      }
      aim.classList.add('on');
    }
    if (event.payload == false) {
      if(is_crosshair_open) {
        invoke('open_crosshair');
      }
      aim.classList.remove('on');
    }
  })

});
