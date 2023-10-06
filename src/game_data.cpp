#include "headers/game_data.hpp"

#include <ncurses.h>

#include <cstdlib>
#include <iostream>
#include <string>

GameData::GameData() {
    namespace fs = std::filesystem;
#ifdef _WIN64
    std::string home_dir = Utils::get_enviroment("USERPROFILE");
#else
    std::string home_dir = std::getenv("HOME");
#endif
    std::string config_dir = home_dir + "/.config/2048";
    this->config_path = fs::path(config_dir).append("game.progress");
    try {
        fs::create_directories(config_dir);
    } catch(fs::filesystem_error e) {
    }
    fstream.open(this->config_path, std::ios::in | std::ios::out | std::ios::binary);
    if(!fstream) {
        std::ofstream(this->config_path, std::ios::out | std::ios::binary);
    }
}

// write down game state to the progress file
void GameData::serialise(std::array<std::array<int, 4>, 4>& array) {
    fstream.seekg(0);
    for(auto& arr : array) {
        for(auto& i : arr) {
            fstream.write(reinterpret_cast<char*>(&i), sizeof(i));
        }
    }
}

// read the progress file and overwrite game state
void GameData::deserialise(std::array<std::array<int, 4>, 4>& array) {
    fstream.seekg(0);
    for(auto& arr : array) {
        for(auto& i : arr) {
            fstream.read(reinterpret_cast<char*>(&i), sizeof(i));
        }
    }
}