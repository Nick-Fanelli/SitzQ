import { invoke } from "@tauri-apps/api";
import { open } from "@tauri-apps/api/dialog"
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { useCallback, useEffect, useState } from "react"

const App = () => {

    const [soundFile, setSoundFile] = useState<string | null>(null);

    const openSoundFile = useCallback(() => {

        open({
            multiple: false,
        }).then((res) => {
            if(res !== null) {

                setSoundFile(convertFileSrc(res.toString()));

            }
        });

    }, [setSoundFile]);

    const playSound = useCallback(() => {

        if(soundFile === null)
            return;

        let audio = new Audio(soundFile);
        audio.play();

        console.log(soundFile);

    }, [soundFile]);

    useEffect(() => {


        // let a = new Audio("file://Users/nickfanelli/Downloads/audio/A Really Short Slow Dance Song (revised 2).wav");
        // a.play();

    }, [soundFile]);

    return (
        <>
            <h1>{soundFile}</h1>
            <button onClick={openSoundFile}>Open File</button>
            <button onClick={playSound}>Play Sound File</button>
        </>
    )

}

export default App;
