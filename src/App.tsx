import { invoke } from "@tauri-apps/api";
import { open } from "@tauri-apps/api/dialog"
import { P } from "@tauri-apps/api/event-41a9edf5";
import { useCallback, useRef, useState } from "react"
import BackendAPI from "./api/BackendAPI";

const App = () => {

    const [soundFile, setSoundFile] = useState<string | null>(null);
    const [audioPlayerID, setAudioPlayerID] = useState<BackendAPI.Audio.AudioPlayerID | null>(null);

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

        if(soundFile != null) {
            BackendAPI.Audio.playSound(soundFile).then((res) => {

                console.log("Audio Player ID: ", res);
                setAudioPlayerID(res);

            });

        }

    }, [soundFile, setAudioPlayerID]);

    const pauseSound = useCallback(() => {

        audioPlayerID != null && BackendAPI.Audio.pauseSound(audioPlayerID);

    }, [audioPlayerID]);

    return (
        <>
            <h1>Sound File: {soundFile}</h1>
            <h2>Audio Player ID: {audioPlayerID}</h2>
            <button onClick={openSoundFile}>Open File</button>
            <button onClick={playSound}>Play Sound File</button>
            <button onClick={pauseSound}>Pause</button>
            <button onClick={() => invoke('print_stats')}>Print Stats</button>
        </>
    )

}

export default App;
