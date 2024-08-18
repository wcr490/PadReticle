import { invoke } from '@tauri-apps/api/tauri';

async function updateImage() {
    try {
        const base64Image = await invoke('update_crosshair_image') as string;
        const imgElement = document.getElementById('crosshair') as HTMLImageElement;
        imgElement.src = `data:image/png;base64,${base64Image}`;
    } catch (error) {
      alert('Failed to update image:');
      console.error('Failed to update image:', error);
    }
}


window.addEventListener("DOMContentLoaded", () => {
    updateImage();
});
