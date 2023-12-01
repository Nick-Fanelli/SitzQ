import { Howl } from 'howler';

namespace AudioSystem {

    export type AudioPlayerID = number;

    let audioPlayerNextID = 0;

    const audioSources = new Map<string, Howl>();
    const audioPlayers = new Map<AudioPlayerID, Howl>();

    const getAudioSource = (filepath: string) : Howl => {

        let audioSource = audioSources.get(filepath);

        if(!audioSource) {
            audioSource = new Howl({
                src: [filepath]
            });

            audioSources.set(filepath, audioSource);
        }

        return audioSource;

    }

    export const createAudioPlayer = (filepath: string) : AudioPlayerID => {

        const playerID: AudioPlayerID = audioPlayerNextID++;

        audioPlayers.set(playerID, getAudioSource(filepath));

        return playerID;

    }

    export const playAudioPlayer = (playerID: AudioPlayerID) => {

        const audioPlayer = audioPlayers.get(playerID);
        audioPlayer && audioPlayer.play();

    }

    export const pauseAudioPlayer = (playerID: AudioPlayerID) => {

        const audioPlayer = audioPlayers.get(playerID);
        audioPlayer && audioPlayer.pause();
        
    }

    export const printStats = () => {

        console.log("Num Audio Sources: " + audioSources.size);
        console.log("Num Audio Players: " + audioPlayers.size);

    }

}

export default AudioSystem;