import { invoke } from "@tauri-apps/api";
import { open } from "@tauri-apps/api/dialog"
import { convertFileSrc } from '@tauri-apps/api/tauri'
import { useCallback, useRef, useState } from "react"

import { Howl } from 'howler';
import AudioSystem from "./api/AudioSystem";
import { C } from "@tauri-apps/api/event-41a9edf5";

const Sound = () => {

    const [soundFile, setSoundFile] = useState<string | null>(null);
    const [audioPlayerID, setAudioPlayerID] = useState<AudioSystem.AudioPlayerID | null>();

    const openSoundFile = useCallback(() => {

        open({
            multiple: false,
        }).then((res) => {
            if(res !== null) {
                const assetPath = convertFileSrc(res.toString());

                setSoundFile(assetPath);
                setAudioPlayerID(AudioSystem.createAudioPlayer(assetPath));
            }
        });

    }, [setSoundFile, setAudioPlayerID]);

    const playSound = useCallback(() => {

        audioPlayerID != null && AudioSystem.playAudioPlayer(audioPlayerID);

    }, [audioPlayerID]);

    const pauseSound = useCallback(() => {

        audioPlayerID != null && AudioSystem.pauseAudioPlayer(audioPlayerID);

    }, [audioPlayerID]);

    return (
        <>
            <h1>Sound File: {soundFile}</h1>
            <button onClick={openSoundFile}>Open File</button>
            <button onClick={playSound}>Play</button>
            <button onClick={pauseSound}>Pause</button>
            <button onClick={() => AudioSystem.printStats()}>Print Stats</button>
        </>
    )

}

export default Sound;
