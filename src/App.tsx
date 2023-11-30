import { invoke } from "@tauri-apps/api";
import { open } from "@tauri-apps/api/dialog"
import { convertFileSrc } from '@tauri-apps/api/tauri'
import { useCallback, useRef, useState } from "react"

import { Howl } from 'howler';

const App = () => {

    const [soundFile, setSoundFile] = useState<string | null>(null);

    const openSoundFile = useCallback(() => {

        open({
            multiple: false,
        }).then((res) => {
            if(res !== null) {
                setSoundFile(res.toString());
            }
        });

    }, [setSoundFile]);

    const playSound = useCallback(() => {

        if(soundFile == null)
            return; 

        const sound = new Howl({
            src: [convertFileSrc(soundFile)],
            volume: 1
        });

        sound.play();

    }, [soundFile]);

    const pauseSound = useCallback(() => {


    }, []);

    return (
        <>
            <h1>Sound File: {soundFile}</h1>
            <button onClick={openSoundFile}>Open File</button>
            <button onClick={playSound}>Play</button>
            <button onClick={pauseSound}>Pause</button>
        </>
    )

}

export default App;
