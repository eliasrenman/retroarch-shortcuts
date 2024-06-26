use serde::Serialize;
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum Console {
    AMSTRAD_CPC,
    ATARI_5200,
    ATARI_7800,
    ATARI_JAGUAR,
    ATARI_LYNX,
    ATARI_ST,
    BANDAI_WONDERSWAN,
    BANDAI_WONDERSWAN_COLOR,
    CANNONBALL,
    CASIO_LOOPY,
    CASIO_PV_1000,
    CAVE_STORY,
    CHAILOVE,
    COLECO_COLECOVISION,
    COMMODORE_64,
    COMMODORE_AMIGA,
    COMMODORE_PLUS_4,
    COMMODORE_VIC_20,
    DOOM,
    DOS,
    DINOTHAWR,
    EMERSON_ARCADIA_2001,
    ENTEX_ADVENTURE_VISION,
    EPOCH_SUPER_CASSETTE_VISION,
    FBNEO_ARCADE_GAMES,
    FAIRCHILD_CHANNEL_F,
    FLASHBACK,
    FUNTECH_SUPER_ACAN,
    GCE_VECTREX,
    GAMEPARK_GP32,
    HANDHELD_ELECTRONIC_GAME,
    HARTUNG_GAME_MASTER,
    LEAPFROG_LEAPSTER_LEARNING_GAME_SYSTEM,
    LUTRO,
    MAME,
    MAME_2000,
    MAME_2003,
    MAME_2003_PLUS,
    MAME_2010,
    MAME_2015,
    MAME_2016,
    MAGNAVOX_ODYSSEY2,
    MATTEL_INTELLIVISION,
    MICROSOFT_MSX,
    MICROSOFT_MSX2,
    MICROSOFT_XBOX,
    MRBOOM,
    NEC_PC_ENGINE_TURBOGRAFX_16,
    NEC_PC_ENGINE_CD_TURBOGRAFX_CD,
    NEC_PC_ENGINE_SUPERGRAFX,
    NEC_PC_FX,
    NINTENDO_FAMILY_COMPUTER_DISK_SYSTEM,
    NINTENDO_GAME_BOY,
    NINTENDO_GAME_BOY_ADVANCE,
    NINTENDO_GAME_BOY_COLOR,
    NINTENDO_GAMECUBE,
    NINTENDO_NINTENDO_3DS,
    NINTENDO_NINTENDO_3DS_DIGITAL,
    NINTENDO_NINTENDO_64,
    NINTENDO_NINTENDO_64DD,
    NINTENDO_NINTENDO_DS,
    NINTENDO_NINTENDO_DS_DOWNLOAD_PLAY,
    NINTENDO_NINTENDO_DS_DOWNLOAD_PLAY_BETA,
    NINTENDO_NINTENDO_DSI,
    NINTENDO_NINTENDO_DSI_DIGITAL,
    NINTENDO_NINTENDO_DSI_DECRYPTED,
    NINTENDO_NINTENDO_ENTERTAINMENT_SYSTEM,
    NINTENDO_POKEMON_MINI,
    NINTENDO_SATELLAVIEW,
    NINTENDO_SUFAMI_TURBO,
    NINTENDO_SUPER_NINTENDO_ENTERTAINMENT_SYSTEM,
    NINTENDO_VIRTUAL_BOY,
    NINTENDO_WII,
    NINTENDO_WII_DIGITAL,
    NINTENDO_E_READER,
    PHILIPS_VIDEOPAC_PLUS,
    QUAKE,
    RCA_STUDIO_II,
    RPG_MAKER,
    RICK_DANGEROUS,
    SNK_NEO_GEO_CD,
    SNK_NEO_GEO_POCKET,
    SNK_NEO_GEO_POCKET_COLOR,
    SCUMMVM,
    SEGA_32X,
    SEGA_DREAMCAST,
    SEGA_GAME_GEAR,
    SEGA_MASTER_SYSTEM_MARK_III,
    SEGA_MEGA_DRIVE_GENESIS,
    SEGA_MEGA_CD_SEGA_CD,
    SEGA_PICO,
    SEGA_SG_1000,
    SEGA_SATURN,
    SHARP_X68000,
    SINCLAIR_ZX_81,
    SINCLAIR_ZX_SPECTRUM,
    SINCLAIR_ZX_SPECTRUM_PLUS_3,
    SONY_PLAYSTATION,
    SONY_PLAYSTATION_2,
    SONY_PLAYSTATION_3,
    SONY_PLAYSTATION_3_DOWNLOADABLE,
    SONY_PLAYSTATION_3_PSN_,
    SONY_PLAYSTATION_PORTABLE,
    SONY_PLAYSTATION_PORTABLE_PSN,
    SONY_PLAYSTATION_PORTABLE_PSX2PSP,
    SONY_PLAYSTATION_PORTABLE_UMD_MUSIC,
    SONY_PLAYSTATION_PORTABLE_UMD_VIDEO,
    TIC_80,
    THE_3DO_COMPANY_3DO,
    THOMSON_MOTO,
    TIGER_GAME_DOT_COM,
    TOMB_RAIDER,
    UZEBOX,
    VTECH_CREATIVISION,
    VTECH_V_DOT_SMILE,
    WATARA_SUPERVISION,
    OTHER,
}

impl Console {
    pub fn by_title(title: &str) -> Console {
        match title {
            "Amstrad - CPC" => Console::AMSTRAD_CPC,
            "Atari - 5200" => Console::ATARI_5200,
            "Atari - 7800" => Console::ATARI_7800,
            "Atari - Jaguar" => Console::ATARI_JAGUAR,
            "Atari - Lynx" => Console::ATARI_LYNX,
            "Atari - ST" => Console::ATARI_ST,
            "Bandai - WonderSwan" => Console::BANDAI_WONDERSWAN,
            "Bandai - WonderSwan Color" => Console::BANDAI_WONDERSWAN_COLOR,
            "Cannonball" => Console::CANNONBALL,
            "Casio - Loopy" => Console::CASIO_LOOPY,
            "Casio - PV-1000" => Console::CASIO_PV_1000,
            "Cave Story" => Console::CAVE_STORY,
            "ChaiLove" => Console::CHAILOVE,
            "Coleco - ColecoVision" => Console::COLECO_COLECOVISION,
            "Commodore - 64" => Console::COMMODORE_64,
            "Commodore - Amiga" => Console::COMMODORE_AMIGA,
            "Commodore - Plus-4" => Console::COMMODORE_PLUS_4,
            "Commodore - VIC-20" => Console::COMMODORE_VIC_20,
            "DOOM" => Console::DOOM,
            "DOS" => Console::DOS,
            "Dinothawr" => Console::DINOTHAWR,
            "Emerson - Arcadia 2001" => Console::EMERSON_ARCADIA_2001,
            "Entex - Adventure Vision" => Console::ENTEX_ADVENTURE_VISION,
            "Epoch - Super Cassette Vision" => Console::EPOCH_SUPER_CASSETTE_VISION,
            "FBNeo - Arcade Games" => Console::FBNEO_ARCADE_GAMES,
            "Fairchild - Channel F" => Console::FAIRCHILD_CHANNEL_F,
            "Flashback" => Console::FLASHBACK,
            "Funtech - Super Acan" => Console::FUNTECH_SUPER_ACAN,
            "GCE - Vectrex" => Console::GCE_VECTREX,
            "GamePark - GP32" => Console::GAMEPARK_GP32,
            "Handheld Electronic Game" => Console::HANDHELD_ELECTRONIC_GAME,
            "Hartung - Game Master" => Console::HARTUNG_GAME_MASTER,
            "LeapFrog - Leapster Learning Game System" => {
                Console::LEAPFROG_LEAPSTER_LEARNING_GAME_SYSTEM
            }
            "Lutro" => Console::LUTRO,
            "MAME" => Console::MAME,
            "MAME 2000" => Console::MAME_2000,
            "MAME 2003" => Console::MAME_2003,
            "MAME 2003-Plus" => Console::MAME_2003_PLUS,
            "MAME 2010" => Console::MAME_2010,
            "MAME 2015" => Console::MAME_2015,
            "MAME 2016" => Console::MAME_2016,
            "Magnavox - Odyssey2" => Console::MAGNAVOX_ODYSSEY2,
            "Mattel - Intellivision" => Console::MATTEL_INTELLIVISION,
            "Microsoft - MSX" => Console::MICROSOFT_MSX,
            "Microsoft - MSX2" => Console::MICROSOFT_MSX2,
            "Microsoft - Xbox" => Console::MICROSOFT_XBOX,
            "MrBoom" => Console::MRBOOM,
            "NEC - PC Engine - TurboGrafx 16" => Console::NEC_PC_ENGINE_TURBOGRAFX_16,
            "NEC - PC Engine CD - TurboGrafx-CD" => Console::NEC_PC_ENGINE_CD_TURBOGRAFX_CD,
            "NEC - PC Engine SuperGrafx" => Console::NEC_PC_ENGINE_SUPERGRAFX,
            "NEC - PC-FX" => Console::NEC_PC_FX,
            "Nintendo - Family Computer Disk System" => {
                Console::NINTENDO_FAMILY_COMPUTER_DISK_SYSTEM
            }
            "Nintendo - Game Boy" => Console::NINTENDO_GAME_BOY,
            "Nintendo - Game Boy Advance" => Console::NINTENDO_GAME_BOY_ADVANCE,
            "Nintendo - Game Boy Color" => Console::NINTENDO_GAME_BOY_COLOR,
            "Nintendo - GameCube" => Console::NINTENDO_GAMECUBE,
            "Nintendo - Nintendo 3DS" => Console::NINTENDO_NINTENDO_3DS,
            "Nintendo - Nintendo 3DS (Digital)" => Console::NINTENDO_NINTENDO_3DS_DIGITAL,
            "Nintendo - Nintendo 64" => Console::NINTENDO_NINTENDO_64,
            "Nintendo - Nintendo 64DD" => Console::NINTENDO_NINTENDO_64DD,
            "Nintendo - Nintendo DS" => Console::NINTENDO_NINTENDO_DS,
            "Nintendo - Nintendo DS (Download Play)" => Console::NINTENDO_NINTENDO_DS_DOWNLOAD_PLAY,
            "Nintendo - Nintendo DS (Download Play) (BETA)" => {
                Console::NINTENDO_NINTENDO_DS_DOWNLOAD_PLAY_BETA
            }
            "Nintendo - Nintendo DSi" => Console::NINTENDO_NINTENDO_DSI,
            "Nintendo - Nintendo DSi (Digital)" => Console::NINTENDO_NINTENDO_DSI_DIGITAL,
            "Nintendo - Nintendo DSi Decrypted" => Console::NINTENDO_NINTENDO_DSI_DECRYPTED,
            "Nintendo - Nintendo Entertainment System" => {
                Console::NINTENDO_NINTENDO_ENTERTAINMENT_SYSTEM
            }
            "Nintendo - Pokemon Mini" => Console::NINTENDO_POKEMON_MINI,
            "Nintendo - Satellaview" => Console::NINTENDO_SATELLAVIEW,
            "Nintendo - Sufami Turbo" => Console::NINTENDO_SUFAMI_TURBO,
            "Nintendo - Super Nintendo Entertainment System" => {
                Console::NINTENDO_SUPER_NINTENDO_ENTERTAINMENT_SYSTEM
            }
            "Nintendo - Virtual Boy" => Console::NINTENDO_VIRTUAL_BOY,
            "Nintendo - Wii" => Console::NINTENDO_WII,
            "Nintendo - Wii (Digital)" => Console::NINTENDO_WII_DIGITAL,
            "Nintendo - e-Reader" => Console::NINTENDO_E_READER,
            "Philips - Videopac+" => Console::PHILIPS_VIDEOPAC_PLUS,
            "Quake" => Console::QUAKE,
            "RCA - Studio II" => Console::RCA_STUDIO_II,
            "RPG Maker" => Console::RPG_MAKER,
            "Rick Dangerous" => Console::RICK_DANGEROUS,
            "SNK - Neo Geo CD" => Console::SNK_NEO_GEO_CD,
            "SNK - Neo Geo Pocket" => Console::SNK_NEO_GEO_POCKET,
            "SNK - Neo Geo Pocket Color" => Console::SNK_NEO_GEO_POCKET_COLOR,
            "ScummVM" => Console::SCUMMVM,
            "Sega - 32X" => Console::SEGA_32X,
            "Sega - Dreamcast" => Console::SEGA_DREAMCAST,
            "Sega - Game Gear" => Console::SEGA_GAME_GEAR,
            "Sega - Master System - Mark III" => Console::SEGA_MASTER_SYSTEM_MARK_III,
            "Sega - Mega Drive - Genesis" => Console::SEGA_MEGA_DRIVE_GENESIS,
            "Sega - Mega-CD - Sega CD" => Console::SEGA_MEGA_CD_SEGA_CD,
            "Sega - PICO" => Console::SEGA_PICO,
            "Sega - SG-1000" => Console::SEGA_SG_1000,
            "Sega - Saturn" => Console::SEGA_SATURN,
            "Sharp - X68000" => Console::SHARP_X68000,
            "Sinclair - ZX 81" => Console::SINCLAIR_ZX_81,
            "Sinclair - ZX Spectrum" => Console::SINCLAIR_ZX_SPECTRUM,
            "Sinclair - ZX Spectrum +3" => Console::SINCLAIR_ZX_SPECTRUM_PLUS_3,
            "Sony - PlayStation" => Console::SONY_PLAYSTATION,
            "Sony - PlayStation 2" => Console::SONY_PLAYSTATION_2,
            "Sony - PlayStation 3" => Console::SONY_PLAYSTATION_3,
            "Sony - PlayStation 3 (Downloadable)" => Console::SONY_PLAYSTATION_3_DOWNLOADABLE,
            "Sony - PlayStation 3 (PSN)" => Console::SONY_PLAYSTATION_3_PSN_,
            "Sony - PlayStation Portable" => Console::SONY_PLAYSTATION_PORTABLE,
            "Sony - PlayStation Portable (PSN)" => Console::SONY_PLAYSTATION_PORTABLE_PSN,
            "Sony - PlayStation Portable (PSX2PSP)" => Console::SONY_PLAYSTATION_PORTABLE_PSX2PSP,
            "Sony - PlayStation Portable (UMD Music)" => {
                Console::SONY_PLAYSTATION_PORTABLE_UMD_MUSIC
            }
            "Sony - PlayStation Portable (UMD Video)" => {
                Console::SONY_PLAYSTATION_PORTABLE_UMD_VIDEO
            }
            "TIC-80" => Console::TIC_80,
            "The 3DO Company - 3DO" => Console::THE_3DO_COMPANY_3DO,
            "Thomson - MOTO" => Console::THOMSON_MOTO,
            "Tiger - Game.com" => Console::TIGER_GAME_DOT_COM,
            "Tomb Raider" => Console::TOMB_RAIDER,
            "Uzebox" => Console::UZEBOX,
            "VTech - CreatiVision" => Console::VTECH_CREATIVISION,
            "VTech - V.Smile" => Console::VTECH_V_DOT_SMILE,
            "Watara - Supervision" => Console::WATARA_SUPERVISION,
            _ => Console::OTHER,
        }
    }
    pub fn to_title(self: Console) -> &'static str {
        match self {
            Console::AMSTRAD_CPC => "Amstrad - CPC",
            Console::ATARI_5200 => "Atari - 5200",
            Console::ATARI_7800 => "Atari - 7800",
            Console::ATARI_JAGUAR => "Atari - Jaguar",
            Console::ATARI_LYNX => "Atari - Lynx",
            Console::ATARI_ST => "Atari - ST",
            Console::BANDAI_WONDERSWAN => "Bandai - WonderSwan",
            Console::BANDAI_WONDERSWAN_COLOR => "Bandai - WonderSwan Color",
            Console::CANNONBALL => "Cannonball",
            Console::CASIO_LOOPY => "Casio - Loopy",
            Console::CASIO_PV_1000 => "Casio - PV-1000",
            Console::CAVE_STORY => "Cave Story",
            Console::CHAILOVE => "ChaiLove",
            Console::COLECO_COLECOVISION => "Coleco - ColecoVision",
            Console::COMMODORE_64 => "Commodore - 64",
            Console::COMMODORE_AMIGA => "Commodore - Amiga",
            Console::COMMODORE_PLUS_4 => "Commodore - Plus-4",
            Console::COMMODORE_VIC_20 => "Commodore - VIC-20",
            Console::DOOM => "DOOM",
            Console::DOS => "DOS",
            Console::DINOTHAWR => "Dinothawr",
            Console::EMERSON_ARCADIA_2001 => "Emerson - Arcadia 2001",
            Console::ENTEX_ADVENTURE_VISION => "Entex - Adventure Vision",
            Console::EPOCH_SUPER_CASSETTE_VISION => "Epoch - Super Cassette Vision",
            Console::FBNEO_ARCADE_GAMES => "FBNeo - Arcade Games",
            Console::FAIRCHILD_CHANNEL_F => "Fairchild - Channel F",
            Console::FLASHBACK => "Flashback",
            Console::FUNTECH_SUPER_ACAN => "Funtech - Super Acan",
            Console::GCE_VECTREX => "GCE - Vectrex",
            Console::GAMEPARK_GP32 => "GamePark - GP32",
            Console::HANDHELD_ELECTRONIC_GAME => "Handheld Electronic Game",
            Console::HARTUNG_GAME_MASTER => "Hartung - Game Master",
            Console::LEAPFROG_LEAPSTER_LEARNING_GAME_SYSTEM => {
                "LeapFrog - Leapster Learning Game System"
            }
            Console::LUTRO => "Lutro",
            Console::MAME => "MAME",
            Console::MAME_2000 => "MAME 2000",
            Console::MAME_2003 => "MAME 2003",
            Console::MAME_2003_PLUS => "MAME 2003-Plus",
            Console::MAME_2010 => "MAME 2010",
            Console::MAME_2015 => "MAME 2015",
            Console::MAME_2016 => "MAME 2016",
            Console::MAGNAVOX_ODYSSEY2 => "Magnavox - Odyssey2",
            Console::MATTEL_INTELLIVISION => "Mattel - Intellivision",
            Console::MICROSOFT_MSX => "Microsoft - MSX",
            Console::MICROSOFT_MSX2 => "Microsoft - MSX2",
            Console::MICROSOFT_XBOX => "Microsoft - Xbox",
            Console::MRBOOM => "MrBoom",
            Console::NEC_PC_ENGINE_TURBOGRAFX_16 => "NEC - PC Engine - TurboGrafx 16",
            Console::NEC_PC_ENGINE_CD_TURBOGRAFX_CD => "NEC - PC Engine CD - TurboGrafx-CD",
            Console::NEC_PC_ENGINE_SUPERGRAFX => "NEC - PC Engine SuperGrafx",
            Console::NEC_PC_FX => "NEC - PC-FX",
            Console::NINTENDO_FAMILY_COMPUTER_DISK_SYSTEM => {
                "Nintendo - Family Computer Disk System"
            }
            Console::NINTENDO_GAME_BOY => "Nintendo - Game Boy",
            Console::NINTENDO_GAME_BOY_ADVANCE => "Nintendo - Game Boy Advance",
            Console::NINTENDO_GAME_BOY_COLOR => "Nintendo - Game Boy Color",
            Console::NINTENDO_GAMECUBE => "Nintendo - GameCube",
            Console::NINTENDO_NINTENDO_3DS => "Nintendo - Nintendo 3DS",
            Console::NINTENDO_NINTENDO_3DS_DIGITAL => "Nintendo - Nintendo 3DS (Digital)",
            Console::NINTENDO_NINTENDO_64 => "Nintendo - Nintendo 64",
            Console::NINTENDO_NINTENDO_64DD => "Nintendo - Nintendo 64DD",
            Console::NINTENDO_NINTENDO_DS => "Nintendo - Nintendo DS",
            Console::NINTENDO_NINTENDO_DS_DOWNLOAD_PLAY => "Nintendo - Nintendo DS (Download Play)",
            Console::NINTENDO_NINTENDO_DS_DOWNLOAD_PLAY_BETA => {
                "Nintendo - Nintendo DS (Download Play) (BETA)"
            }
            Console::NINTENDO_NINTENDO_DSI => "Nintendo - Nintendo DSi",
            Console::NINTENDO_NINTENDO_DSI_DIGITAL => "Nintendo - Nintendo DSi (Digital)",
            Console::NINTENDO_NINTENDO_DSI_DECRYPTED => "Nintendo - Nintendo DSi Decrypted",
            Console::NINTENDO_NINTENDO_ENTERTAINMENT_SYSTEM => {
                "Nintendo - Nintendo Entertainment System"
            }
            Console::NINTENDO_POKEMON_MINI => "Nintendo - Pokemon Mini",
            Console::NINTENDO_SATELLAVIEW => "Nintendo - Satellaview",
            Console::NINTENDO_SUFAMI_TURBO => "Nintendo - Sufami Turbo",

            Console::NINTENDO_SUPER_NINTENDO_ENTERTAINMENT_SYSTEM => {
                "Nintendo - Super Nintendo Entertainment System"
            }
            Console::NINTENDO_VIRTUAL_BOY => "Nintendo - Virtual Boy",
            Console::NINTENDO_WII => "Nintendo - Wii",
            Console::NINTENDO_WII_DIGITAL => "Nintendo - Wii (Digital)",
            Console::NINTENDO_E_READER => "Nintendo - e-Reader",
            Console::PHILIPS_VIDEOPAC_PLUS => "Philips - Videopac+",
            Console::QUAKE => "Quake",
            Console::RCA_STUDIO_II => "RCA - Studio II",
            Console::RPG_MAKER => "RPG Maker",
            Console::RICK_DANGEROUS => "Rick Dangerous",
            Console::SNK_NEO_GEO_CD => "SNK - Neo Geo CD",
            Console::SNK_NEO_GEO_POCKET => "SNK - Neo Geo Pocket",
            Console::SNK_NEO_GEO_POCKET_COLOR => "SNK - Neo Geo Pocket Color",
            Console::SCUMMVM => "ScummVM",
            Console::SEGA_32X => "Sega - 32X",
            Console::SEGA_DREAMCAST => "Sega - Dreamcast",
            Console::SEGA_GAME_GEAR => "Sega - Game Gear",
            Console::SEGA_MASTER_SYSTEM_MARK_III => "Sega - Master System - Mark III",
            Console::SEGA_MEGA_DRIVE_GENESIS => "Sega - Mega Drive - Genesis",
            Console::SEGA_MEGA_CD_SEGA_CD => "Sega - Mega-CD - Sega CD",
            Console::SEGA_PICO => "Sega - PICO",
            Console::SEGA_SG_1000 => "Sega - SG-1000",
            Console::SEGA_SATURN => "Sega - Saturn",
            Console::SHARP_X68000 => "Sharp - X68000",
            Console::SINCLAIR_ZX_81 => "Sinclair - ZX 81",
            Console::SINCLAIR_ZX_SPECTRUM => "Sinclair - ZX Spectrum",
            Console::SINCLAIR_ZX_SPECTRUM_PLUS_3 => "Sinclair - ZX Spectrum +3",
            Console::SONY_PLAYSTATION => "Sony - PlayStation",
            Console::SONY_PLAYSTATION_2 => "Sony - PlayStation 2",
            Console::SONY_PLAYSTATION_3 => "Sony - PlayStation 3",
            Console::SONY_PLAYSTATION_3_DOWNLOADABLE => "Sony - PlayStation 3 (Downloadable)",
            Console::SONY_PLAYSTATION_3_PSN_ => "Sony - PlayStation 3 (PSN)",
            Console::SONY_PLAYSTATION_PORTABLE => "Sony - PlayStation Portable",
            Console::SONY_PLAYSTATION_PORTABLE_PSN => "Sony - PlayStation Portable (PSN)",
            Console::SONY_PLAYSTATION_PORTABLE_PSX2PSP => "Sony - PlayStation Portable (PSX2PSP)",
            Console::SONY_PLAYSTATION_PORTABLE_UMD_MUSIC => {
                "Sony - PlayStation Portable (UMD Music)"
            }
            Console::SONY_PLAYSTATION_PORTABLE_UMD_VIDEO => {
                "Sony - PlayStation Portable (UMD Video)"
            }
            Console::TIC_80 => "TIC-80",
            Console::THE_3DO_COMPANY_3DO => "The 3DO Company - 3DO",
            Console::THOMSON_MOTO => "Thomson - MOTO",
            Console::TIGER_GAME_DOT_COM => "Tiger - Game.com",
            Console::TOMB_RAIDER => "Tomb Raider",
            Console::UZEBOX => "Uzebox",
            Console::VTECH_CREATIVISION => "VTech - CreatiVision",
            Console::VTECH_V_DOT_SMILE => "VTech - V.Smile",
            Console::WATARA_SUPERVISION => "Watara - Supervision",
            Console::OTHER => "",
        }
    }

    pub fn core_name(&self) -> &'static str {
        match self {
            Console::AMSTRAD_CPC => "cpc_libretro.dll",
            Console::ATARI_5200 => "atari800_libretro.dll",
            Console::ATARI_7800 => "prosystem_libretro.dll",
            Console::ATARI_JAGUAR => "virtualjaguar_libretro.dll",
            Console::ATARI_LYNX => "handy_libretro.dll",
            Console::ATARI_ST => "hatari_libretro.dll",
            Console::BANDAI_WONDERSWAN => "mednafen_wswan_libretro.dll",
            Console::BANDAI_WONDERSWAN_COLOR => "mednafen_wswan_libretro.dll",
            Console::CANNONBALL => "cannonball_libretro.dll",
            Console::CASIO_LOOPY => "nxengine_libretro.dll",
            Console::CASIO_PV_1000 => "px68k_libretro.dll",
            Console::CAVE_STORY => "cavestory_libretro.dll",
            Console::CHAILOVE => "chailove_libretro.dll",
            Console::COLECO_COLECOVISION => "bluemsx_libretro.dll",
            Console::COMMODORE_64 => "vice_x64_libretro.dll",
            Console::COMMODORE_AMIGA => "puae_libretro.dll",
            Console::COMMODORE_PLUS_4 => "vice_xplus4_libretro.dll",
            Console::COMMODORE_VIC_20 => "vice_xvic_libretro.dll",
            Console::DOOM => "prboom_libretro.dll",
            Console::DOS => "dosbox_libretro.dll",
            Console::DINOTHAWR => "dinheiro_libretro.dll",
            Console::EMERSON_ARCADIA_2001 => "mame2010_libretro.dll",
            Console::ENTEX_ADVENTURE_VISION => "mame2010_libretro.dll",
            Console::EPOCH_SUPER_CASSETTE_VISION => "mame2010_libretro.dll",
            Console::FBNEO_ARCADE_GAMES => "fbneo_libretro.dll",
            Console::FAIRCHILD_CHANNEL_F => "mame2010_libretro.dll",
            Console::FLASHBACK => "mame2010_libretro.dll",
            Console::FUNTECH_SUPER_ACAN => "mame2010_libretro.dll",
            Console::GCE_VECTREX => "vecx_libretro.dll",
            Console::GAMEPARK_GP32 => "race_libretro.dll",
            Console::HANDHELD_ELECTRONIC_GAME => "mgba_libretro.dll",
            Console::HARTUNG_GAME_MASTER => "mame2010_libretro.dll",
            Console::LEAPFROG_LEAPSTER_LEARNING_GAME_SYSTEM => "mame2010_libretro.dll",
            Console::LUTRO => "lutro_libretro.dll",
            Console::MAME => "mame_libretro.dll",
            Console::MAME_2000 => "mame2000_libretro.dll",
            Console::MAME_2003 => "mame2003_libretro.dll",
            Console::MAME_2003_PLUS => "mame2003_plus_libretro.dll",
            Console::MAME_2010 => "mame2010_libretro.dll",
            Console::MAME_2015 => "mame2015_libretro.dll",
            Console::MAME_2016 => "mame2016_libretro.dll",
            Console::MAGNAVOX_ODYSSEY2 => "o2em_libretro.dll",
            Console::MATTEL_INTELLIVISION => "lto_libretro.dll",
            Console::MICROSOFT_MSX => "bluemsx_libretro.dll",
            Console::MICROSOFT_MSX2 => "bluemsx_libretro.dll",
            Console::MICROSOFT_XBOX => "xenia_libretro.dll",
            Console::MRBOOM => "mrboom_libretro.dll",
            Console::NEC_PC_ENGINE_TURBOGRAFX_16 => "mednafen_pce_fast_libretro.dll",
            Console::NEC_PC_ENGINE_CD_TURBOGRAFX_CD => "mednafen_pce_fast_libretro.dll",
            Console::NEC_PC_ENGINE_SUPERGRAFX => "mednafen_supergrafx_libretro.dll",
            Console::NEC_PC_FX => "mednafen_pcfx_libretro.dll",
            Console::NINTENDO_FAMILY_COMPUTER_DISK_SYSTEM => "nestopia_libretro.dll",
            Console::NINTENDO_GAME_BOY => "gambatte_libretro.dll",
            Console::NINTENDO_GAME_BOY_ADVANCE => "mgba_libretro.dll",
            Console::NINTENDO_GAME_BOY_COLOR => "gambatte_libretro.dll",
            Console::NINTENDO_GAMECUBE => "dolphin_libretro.dll",
            Console::NINTENDO_NINTENDO_3DS => "citra_libretro.dll",
            Console::NINTENDO_NINTENDO_3DS_DIGITAL => "citra_libretro.dll",
            Console::NINTENDO_NINTENDO_64 => "mupen64plus_next_libretro.dll",
            Console::NINTENDO_NINTENDO_64DD => "parallel_n64_libretro.dll",
            Console::NINTENDO_NINTENDO_DS => "desmume_libretro.dll",
            Console::NINTENDO_NINTENDO_DS_DOWNLOAD_PLAY => "desmume_libretro.dll",
            Console::NINTENDO_NINTENDO_DS_DOWNLOAD_PLAY_BETA => "desmume_libretro.dll",
            Console::NINTENDO_NINTENDO_DSI => "desmume_libretro.dll",
            Console::NINTENDO_NINTENDO_DSI_DIGITAL => "desmume_libretro.dll",
            Console::NINTENDO_NINTENDO_DSI_DECRYPTED => "desmume_libretro.dll",
            Console::NINTENDO_NINTENDO_ENTERTAINMENT_SYSTEM => "nestopia_libretro.dll",
            Console::NINTENDO_POKEMON_MINI => "pokemini_libretro.dll",
            Console::NINTENDO_SATELLAVIEW => "bsnes_mercury_libretro.dll",
            Console::NINTENDO_SUFAMI_TURBO => "bsnes_mercury_libretro.dll",
            Console::NINTENDO_SUPER_NINTENDO_ENTERTAINMENT_SYSTEM => "snes9x_libretro.dll",
            Console::NINTENDO_VIRTUAL_BOY => "beetle_vb_libretro.dll",
            Console::NINTENDO_WII => "dolphin_libretro.dll",
            Console::NINTENDO_WII_DIGITAL => "dolphin_libretro.dll",
            Console::NINTENDO_E_READER => "mgba_libretro.dll",
            Console::PHILIPS_VIDEOPAC_PLUS => "o2em_libretro.dll",
            Console::QUAKE => "tyrquake_libretro.dll",
            Console::RCA_STUDIO_II => "mame2010_libretro.dll",
            Console::RPG_MAKER => "easyrpg_libretro.dll",
            Console::RICK_DANGEROUS => "mame2010_libretro.dll",
            Console::SNK_NEO_GEO_CD => "fbneo_libretro.dll",
            Console::SNK_NEO_GEO_POCKET => "mednafen_ngp_libretro.dll",
            Console::SNK_NEO_GEO_POCKET_COLOR => "mednafen_ngp_libretro.dll",
            Console::SCUMMVM => "scummvm_libretro.dll",
            Console::SEGA_32X => "picodrive_libretro.dll",
            Console::SEGA_DREAMCAST => "redream_libretro.dll",
            Console::SEGA_GAME_GEAR => "genesis_plus_gx_libretro.dll",
            Console::SEGA_MASTER_SYSTEM_MARK_III => "genesis_plus_gx_libretro.dll",
            Console::SEGA_MEGA_DRIVE_GENESIS => "genesis_plus_gx_libretro.dll",
            Console::SEGA_MEGA_CD_SEGA_CD => "genesis_plus_gx_libretro.dll",
            Console::SEGA_PICO => "picodrive_libretro.dll",
            Console::SEGA_SG_1000 => "genesis_plus_gx_libretro.dll",
            Console::SEGA_SATURN => "yabause_libretro.dll",
            Console::SHARP_X68000 => "px68k_libretro.dll",
            Console::SINCLAIR_ZX_81 => "81_libretro.dll",
            Console::SINCLAIR_ZX_SPECTRUM => "81_libretro.dll",
            Console::SINCLAIR_ZX_SPECTRUM_PLUS_3 => "81_libretro.dll",
            Console::SONY_PLAYSTATION => "pcsx_rearmed_libretro.dll",
            Console::SONY_PLAYSTATION_2 => "pcsx2_libretro.dll",
            Console::SONY_PLAYSTATION_3 => "rpcs3_libretro.dll",
            Console::SONY_PLAYSTATION_3_DOWNLOADABLE => "rpcs3_libretro.dll",
            Console::SONY_PLAYSTATION_3_PSN_ => "rpcs3_libretro.dll",
            Console::SONY_PLAYSTATION_PORTABLE => "ppsspp_libretro.dll",
            Console::SONY_PLAYSTATION_PORTABLE_PSN => "ppsspp_libretro.dll",
            Console::SONY_PLAYSTATION_PORTABLE_PSX2PSP => "ppsspp_libretro.dll",
            Console::SONY_PLAYSTATION_PORTABLE_UMD_MUSIC => "ppsspp_libretro.dll",
            Console::SONY_PLAYSTATION_PORTABLE_UMD_VIDEO => "ppsspp_libretro.dll",
            Console::TIC_80 => "tic80_libretro.dll",
            Console::THE_3DO_COMPANY_3DO => "4do_libretro.dll",
            Console::THOMSON_MOTO => "mame2010_libretro.dll",
            Console::TIGER_GAME_DOT_COM => "mame2010_libretro.dll",
            Console::TOMB_RAIDER => "mame2010_libretro.dll",
            Console::UZEBOX => "uzebox_libretro.dll",
            Console::VTECH_CREATIVISION => "mame2010_libretro.dll",
            Console::VTECH_V_DOT_SMILE => "mame2010_libretro.dll",
            Console::WATARA_SUPERVISION => "mednafen_ngp_libretro.dll",
            Console::OTHER => "",
        }
    }
}
