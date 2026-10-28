use ti_helper_game_data::components::planet::Planet;

/// Returns the visual offset of the planet on the system tile image, as a fraction of the tile size (x right, y up).
///
/// Planets that aren't on a system tile are placed in the center of the tile.
pub fn planet_offset(planet: &Planet) -> (f32, f32) {
    match planet {
        Planet::Jord
        | Planet::MollPrimus
        | Planet::Darien
        | Planet::Muaat
        | Planet::Nestphar
        | Planet::ZeroZeroZero
        | Planet::Winnu
        | Planet::MordaiII
        | Planet::Elysium
        | Planet::Wellon
        | Planet::VefutII
        | Planet::Thibah
        | Planet::TarMann
        | Planet::Saudor
        | Planet::MeharXull
        | Planet::Creuss
        | Planet::Ixth
        | Planet::Arcturus
        | Planet::Acheron
        | Planet::TheDark
        | Planet::ArchonVail
        | Planet::Perimiter
        | Planet::SemLore
        | Planet::Ang
        | Planet::Vorhal
        | Planet::Primor
        | Planet::HopesEnd
        | Planet::Faunus
        | Planet::Garbozia
        | Planet::Emelpar
        | Planet::Tempesta
        | Planet::Olergodt
        | Planet::ViraPicsIII
        | Planet::Lesab
        | Planet::Industrex
        | Planet::Lemox
        | Planet::MecatolRexOmega
        | Planet::MecatolRex
        | Planet::CustodiaVigilia
        | Planet::CustodiaVigiliaOmega
        | Planet::ThundersEdge
        | Planet::Avernus => (0.0, 0.0),
        Planet::Maaluuk
        | Planet::ArcPrime
        | Planet::LisisII
        | Planet::Nar
        | Planet::TrenLak
        | Planet::ArchonRen
        | Planet::Quann
        | Planet::Lodor
        | Planet::NewAlbion
        | Planet::TequRan
        | Planet::Qucenn
        | Planet::Mellon
        | Planet::Lazar
        | Planet::DalBootha
        | Planet::Corneeq
        | Planet::Centauri
        | Planet::Bereg
        | Planet::Arnor
        | Planet::Arinam
        | Planet::Abyz
        | Planet::Naazir
        | Planet::Cormund
        | Planet::Atlas
        | Planet::Everra
        | Planet::Accoen
        | Planet::Kraag
        | Planet::Bakal
        | Planet::Lisis
        | Planet::Cealdri
        | Planet::VegaMajor
        | Planet::Retillion
        | Planet::Ordinian
        | Planet::MezLoOrzFeiZsha
        | Planet::Cronos
        | Planet::CronosHollow
        | Planet::Andeara
        | Planet::NewTerra
        | Planet::Cresius
        | Planet::Tiamat
        | Planet::Kostboth
        | Planet::Bellatrix
        | Planet::Tarana => (-0.1, 0.19),
        Planet::Druaa
        | Planet::Jol
        | Planet::Quinarra
        | Planet::Starpoint
        | Planet::Torkan
        | Planet::Rarron
        | Planet::Zohbat
        | Planet::Sakulag
        | Planet::Xxehan
        | Planet::Resculon
        | Planet::Gral
        | Planet::LirtaIV
        | Planet::Lor
        | Planet::Meer
        | Planet::Fria
        | Planet::Rokha
        | Planet::JeolIr
        | Planet::Siig
        | Planet::AlioPrima
        | Planet::Velnor
        | Planet::Xanhact
        | Planet::VegaMinor
        | Planet::Shalloq
        | Planet::Tinnes
        | Planet::LazulRex
        | Planet::Hercalor
        | Planet::Capha
        | Planet::TsionStation => (0.09, -0.20),
        Planet::RigelII | Planet::Abaddon | Planet::Arretze | Planet::Ylir | Planet::Horizon => {
            (0.12, 0.21)
        }
        Planet::RigelIII | Planet::Loki | Planet::Hercant | Planet::Valk | Planet::Elnath => {
            (-0.24, 0.05)
        }
        Planet::RigelI
        | Planet::Ashtroth
        | Planet::Kamdorn
        | Planet::Avar
        | Planet::WrenTerra
        | Planet::Ragh
        | Planet::ArchonTau
        | Planet::LuthienVi => (0.15, -0.26),
        Planet::Mallice => (0.20, 0.12),
        Planet::Mirage => (0.12, -0.25),
        Planet::RepoLoOrzQet | Planet::Tallin | Planet::TallinHollow => (0.15, -0.18),
        Planet::Ikatena => (0.01, 0.06),
        Planet::Revelation => (0.16, -0.14),
        Planet::OluzStation => (0.05, -0.19),
        Planet::TheWatchtower => (0.07, -0.22),
        Planet::AhkCreuxx => (0.0, 0.11),
        Planet::Cocytus => (-0.22, -0.18),
        Planet::Lethe => (0.17, -0.04),
        Planet::Phlegethon => (0.27, -0.29),
        Planet::Styx => (0.0, 0.19),
    }
}
