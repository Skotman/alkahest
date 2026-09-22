use std::io::SeekFrom;

use glam::{Quat, Vec4};
use tiger_parse::{tiger_type, tiger_variant_enum, Endian, FnvHash, PackageManagerExt, TigerReadable, TigerReader, VariantEnum};
use tiger_pkg::TagHash;
use std::hash::Hasher;
use fnv::FnvHasher;
use log::info;
use crate::{
    tag::{OptionalTag, WideHash, WideTag},
    tfx::{
        atmosphere::{SAtmosphereDataComponent, SUnk80806a74},
        common::AxisAlignedBBox,
        features::{
            ao::SStaticAmbientOcclusion,
            cubemap::SCubemapComponent,
            decals::SDecalCollection,
            decorators::SDecorator,
            light::{SLightCollection, SShadowingLight},
            road_decals::SRoadDecalCollection,
            sky_objects::SSkyObjectCollection,
        },
    },
    umbra::SUmbraTomes,
};
use crate::wwise::SWwiseEvent;
// Audio reference collector
use std::sync::{OnceLock, Mutex};

static AUDIO_REFERENCES: OnceLock<Mutex<Vec<AudioRef>>> = OnceLock::new();

#[derive(Debug)]
struct AudioRef {
    component_type: &'static str,
    class_id: u32,
    event_widehash: WideHash,
    event_id: u32,
    wwise_bank: TagHash,
    wem_streams: Vec<TagHash>,
    entity_context: String,
}

fn collect_audio_ref(data: &ComponentData, context: &str) {
    let (component_type, event) = match data {
        ComponentData::SAudioPointComponent(audio) => ("Point", &audio.event),
        ComponentData::SAudioPathComponent(audio) => ("Path", &audio.event),
        _ => return,
    };

    if let Ok(wwise_event) = tiger_pkg::package_manager().read_tag_struct::<SWwiseEvent>(event.hash32()) {
        let mut refs = AUDIO_REFERENCES.get_or_init(|| Mutex::new(Vec::new())).lock().unwrap();
        refs.push(AudioRef {
            component_type,
            class_id: event.hash32().0,
            event_widehash: *event,
            event_id: wwise_event.event_id,
            wwise_bank: wwise_event.wwise_bank,
            wem_streams: wwise_event.wwise_streams.clone(),
            entity_context: context.to_string(),
        });
    }
}
#[derive(Debug)]
#[tiger_type(id = 0x8080891E, size = 0x50)]
pub struct SBubbleParentShallow {
    pub file_size: u64,

    pub definition: WideHash, // WideHash as of 8.2.0
    pub unk10: u64,
    pub map_name: FnvHash,
}

#[derive(Debug)]
#[tiger_type(id = 0x8080891E, size = 0x50)]
pub struct SBubbleParent {
    pub file_size: u64,

    pub definition: WideTag<SBubbleDefinition>, // WideHash as of 8.2.0
    // pub unkc: Padding<4>, // Removed in 8.2.0
    pub unk10: u64,
    pub map_name: FnvHash,
}

#[derive(Debug)]
#[tiger_type(id = 0x80808701, size = 0x50)]
pub struct SBubbleDefinition {
    pub file_size: u64,
    pub containers: Vec<WideTag<SMapContainer>>,
}

#[derive(Debug)]
#[tiger_type(id = 0x80808707, size = 0x38)]
pub struct SMapContainer {
    pub file_size: u64,
    #[tiger(offset = 0x28)]
    pub data_tables: Vec<TagHash>,
}

#[tiger_type(id = 0x80809883)]
pub struct SMapNodeTable {
    pub file_size: u64,
    pub nodes: Vec<SMapNodeEntry>,
}

#[tiger_type(id = 0x80809885)]
pub struct SMapNodeEntry {
    pub rotation: Quat,
    pub translation: Vec4,
    pub entity_old: TagHash,
    pub unk24: u32,
    pub entity: WideHash,
    pub unk38: [u32; 9], //
    pub unk5c: f32,
    pub unk60: f32,
    pub unk64: TagHash,
    pub unk68: FnvHash,
    pub unk6c: u32,
    pub world_id: u64,
    pub component_data: SComponentDataListPtr,
    pub unk80: [u32; 4],
}

tiger_variant_enum! {
    [Unknown(true)]
    enum ComponentData {
        SStaticTerrainPatchesComponent,
        SStaticInstancesCollectionComponent,
        SDecalCollectionComponent,
        SSkyObjectCollectionComponent,
        SDecoratorsComponent,
        SShadowingLightComponent,
        SLightCollectionComponent,
        SCubemapComponent,
        SStaticAmbientOcclusionComponent,
        SRoadDecalCollectionComponent,
        SWaterPlaneComponent,
        SAtmosphereDataComponent,
        SSunDataComponent,
        SUmbraTomeComponent,
        SRespawnPointsComponent,
        SAudioPathComponent,
        SAudioPointComponent

        // SMaterialPermutationsComponent,
    }
}

#[tiger_type(id = 0x8080402E)]
pub struct SMaterialPermutationsComponent {
    pub config: Vec<(u32, u32)>,
}

#[tiger_type(id = 0x80806A40)]
pub struct SStaticAmbientOcclusionComponent {
    pub ao: OptionalTag<SStaticAmbientOcclusion>,
}

#[tiger_type(id = 0x80806CC9)]
pub struct SStaticInstancesCollectionComponent {
    pub instances: TagHash,
}

#[derive(Clone, Debug)]
#[tiger_type(id = 0x80806C7D)]
pub struct SStaticTerrainPatchesComponent {
    pub identifier: u64,
    pub terrain: TagHash,
    pub terrain_bounds: TagHash,
}

#[tiger_type(id = 0x80806AA3)]
pub struct SSkyObjectCollectionComponent {
    pub objects: OptionalTag<SSkyObjectCollection>,
}

#[tiger_type(id = 0x808068D4)]
pub struct SWaterPlaneComponent {
    pub model: TagHash,
    pub unk4: u32,
    pub unk8: u32,
    pub hash: FnvHash,
    pub bounds: AxisAlignedBBox,
    pub havok_file: TagHash,
    pub unk34: u32,
    pub unk38: TagHash,
}

#[tiger_type(id = 0x80806A63)]
pub struct SLightCollectionComponent {
    pub lights: OptionalTag<SLightCollection>,
}

#[tiger_type(id = 0x80806C5E)]
pub struct SShadowingLightComponent {
    pub light: OptionalTag<SShadowingLight>,
}

#[tiger_type(id = 0x80806955)]
pub struct SDecalCollectionComponent {
    pub decals: OptionalTag<SDecalCollection>,
}

#[tiger_type(id = 0x80806CC3)]
pub struct SDecoratorsComponent {
    pub decorators: OptionalTag<SDecorator>,
}

#[tiger_type(id = 0x808068E8)]
pub struct SRoadDecalCollectionComponent {
    pub tag: OptionalTag<SRoadDecalCollection>,
}

#[tiger_type(id = 0x80806A71)]
pub struct SSunDataComponent {
    pub unk0: OptionalTag<SUnk80806a74>,
    pub unk4: u32,
    pub unk8: u32,
    pub unkc: TagHash,
    pub unk10: TagHash,
    pub unk14: TagHash,
}

#[tiger_type(id = 0x80806CF1)]
pub struct SUmbraTomeComponent {
    pub tag: OptionalTag<SUmbraTomes>,
}

#[tiger_type(id = 0x80808CB5)]
pub struct SRespawnPointsComponent {
    pub tag: OptionalTag<SRespawnPoints>,
}

#[derive(Clone, Debug)]
#[tiger_type(id = 0x80808CB7)]
pub struct SRespawnPoints {
    pub file_size: u64,
    pub unk8: Vec<SRespawnPoint>,
}

#[derive(Clone, Debug)]
#[tiger_type(id = 0x80808CB9)]
pub struct SRespawnPoint {
    pub rotation: Quat,
    pub translation: Vec4,
    pub unk20: u32,
    // cohae: Probably padding
    pub unk24: [u32; 3],
}

#[derive(Clone, Debug)]
#[tiger_type(id = 0x8080666D)]
pub struct SAudioPathComponent {
    pub event: WideHash,
    pub unk10: u32,
    pub unk14: u32,
    pub unk18: f32,
    pub unk1c: FnvHash,
    pub nodes: Vec<Vec4>,
}

#[derive(Clone, Debug)]
#[tiger_type(id = 0x8080666F)]
pub struct SAudioPointComponent {
    pub event: WideHash,
    pub unk10: u32,
    pub unk14: u32,
    pub unk18: f32,
    pub unk1c: FnvHash,
}

pub struct SComponentDataNode {
    next: Option<Box<SComponentDataNode>>,
    data: ComponentData,
}

impl SComponentDataNode {
    pub fn next(&self) -> Option<&SComponentDataNode> {
        self.next.as_deref()
    }

    pub fn data(&self) -> &ComponentData {
        &self.data
    }
}

pub struct SComponentDataListPtr(Option<SComponentDataNode>);

impl SComponentDataListPtr {
    pub fn iter<'a>(&'a self) -> ComponentDataListIter<'a> {
        ComponentDataListIter {
            current: self.0.as_ref(),
        }
    }

    pub fn first(&self) -> Option<&SComponentDataNode> {
        self.0.as_ref()
    }

    pub fn get_by_class(&self, class_id: u32) -> Option<&ComponentData> {
        self.iter().find(|c| c.class_id() == class_id)
    }
}

impl TigerReadable for SComponentDataListPtr {
    fn read_ds_endian(reader: &mut dyn TigerReader, endian: Endian) -> tiger_parse::Result<Self> {
        let offset_base = reader.stream_position()?;
        let offset: i64 = TigerReadable::read_ds_endian(reader, endian)?;
        if offset == 0 || offset == i64::MAX {
            return Ok(Self(None));
        }

        let offset_save = reader.stream_position()?;

        reader.seek(SeekFrom::Start(offset_base))?;
        reader.seek(SeekFrom::Current(offset - 4))?;
        let resource_type: u32 = TigerReadable::read_ds_endian(reader, endian)?;

        reader.seek(SeekFrom::Start(offset_base))?;
        reader.seek(SeekFrom::Current(offset))?;
        let next_unboxed = SComponentDataListPtr::read_ds_endian(reader, endian)?;
        let next = next_unboxed.0.map(Box::new);

        reader.seek(SeekFrom::Start(offset_base))?;
        reader.seek(SeekFrom::Current(offset + 0x10))?;
        let data = ComponentData::read_variant_endian(reader, endian, resource_type)?;

        // Log + collect
        match &data {
            ComponentData::SAudioPointComponent(audio) => {
                info!("SPAWN AUDIO POINT: event={} class=0x{:08X}", audio.event.hash32(), 0x8080666Fu32);
                collect_audio_ref(&data, "Verity map");//broken context
            }
            ComponentData::SAudioPathComponent(audio) => {
                info!("PATH AUDIO: event={} class=0x{:08X} nodes={}", audio.event.hash32(), 0x8080666Du32, audio.nodes.len());
                collect_audio_ref(&data, "Verity map");// broken context
            }
            _ => {}
        }

        reader.seek(SeekFrom::Start(offset_save))?;

        Ok(Self(Some(SComponentDataNode { next, data })))
    }

    const ID: Option<u32> = None;

    const SIZE: usize = 8;
}

pub fn dump_audio_references(map_name: &str) {
    if let Some(refs) = AUDIO_REFERENCES.get() {
        let refs = refs.lock().unwrap();
        if refs.is_empty() {
            info!("No audio references collected");
            return;
        }

        let safe_name = map_name.replace(['\\', '/', ':', '*', '?', '"', '<', '>', '|'], "_");
        let base = format!("Media/{}_audio", safe_name);
        std::fs::create_dir_all(&base).unwrap();

        // TSV inventory
        let mut output = String::new();
        output.push_str("component_type\tclass_id\tevent_id\twwise_bank\twem_count\twem_hashes\tcontext\n");
        for r in refs.iter() {
            let wem_hashes = r.wem_streams.iter().map(|h| format!("0x{:08X}", h.0)).collect::<Vec<_>>().join(",");
            output.push_str(&format!("{}\t0x{:08X}\t{}\t{:?}\t{}\t{}\t{}\n",
                                     r.component_type, r.class_id, r.event_id, r.wwise_bank, r.wem_streams.len(), wem_hashes, r.entity_context));
        }
        let tsv_path = format!("{}/{}_inventory.tsv", base, safe_name);
        std::fs::write(&tsv_path, output).unwrap();
        info!("Dumped {} audio references to {}", refs.len(), tsv_path);

        // Extract .wem files
        for r in refs.iter() {
            for (i, stream) in r.wem_streams.iter().enumerate() {
                if let Ok(wem) = tiger_pkg::package_manager().read_tag(*stream) {
                    let fname = format!("{}/{}_{}_{}.wem", base, r.component_type, r.event_id, i);
                    std::fs::write(&fname, wem).unwrap();
                }
            }
        }
        info!("Extracted .wem files to {}", base);

        // Cross-reference: load global string container for name resolution
        let strings = crate::strings::StringContainer::load_all_global();
        let mut string_lookup = String::new();
        string_lookup.push_str("hash_name\tstring_text\n");
fn fnv_hash_u32(val: u32) -> u32 {
    let mut hasher = FnvHasher::default();
    hasher.write_u32(val);
    hasher.finish() as u32
}

        for r in refs.iter() {
            // Try to resolve event_id/name via string container (using FnvHash conversion)
            let event_hash_fnv = fnv_hash_u32(r.event_id);
            let event_name = strings.try_get(event_hash_fnv).unwrap_or_else(|| format!("<EVENT_ID({})>", r.event_id));
            string_lookup.push_str(&format!("{}\t{}\t{}\n", r.event_id, "event_name", event_name));
            // Also resolve class_id (as u32 FnvHash) if present in string container
            let class_hash_fnv = fnv_hash_u32(r.class_id);
            let class_name = strings.try_get(class_hash_fnv).unwrap_or_else(|| format!("<CLASS({})>", r.class_id));
            string_lookup.push_str(&format!("{}\t{}\t{}\n", r.class_id, "component_class", class_name));
        }
        let string_lookup_path = format!("{}/{}_string_lookup.tsv", base, safe_name);
        std::fs::write(&string_lookup_path, string_lookup).unwrap();
        info!("Dumped string cross-reference to {}", string_lookup_path);

        // TagHash lookup file (definitions / package mappings)
        let mut lookup = String::new();
        lookup.push_str("hash\tpkg_id\tentry_index\tpackage_path\tcontext_note\tentry_ref_type\n");
        let mut seen = std::collections::HashSet::new();
        for r in refs.iter() {
            for h in [&r.wwise_bank].into_iter().chain(&r.wem_streams) {
                let key = format!("{}:{}", h.pkg_id(), h.entry_index());
                if !seen.insert(key.clone()) {
                    continue; // skip duplicates
                }
                let pkg_path = tiger_pkg::package_manager().package_paths.get(&h.pkg_id())
                    .map(|p| p.name.as_str().to_string())
                    .unwrap_or_else(|| "unknown".to_string());
                let entry_info = tiger_pkg::package_manager().get_entry(*h)
                    .map(|e| format!("ref=0x{:08X};ftype={};fsub={}", e.reference, e.file_type, e.file_subtype)).unwrap_or("unknown".to_string());
                lookup.push_str(&format!("0x{:08X}\t{}\t{}\t{}\t{}\t{}\n",
                                       h.0, h.pkg_id(), h.entry_index(), pkg_path,
                                       format!("bank={:?}; streams={:?}", r.wwise_bank, r.wem_streams.len()),
                                       entry_info));
            }
        }
        let lookup_path = format!("{}/{}_taghash_lookup.tsv", base, safe_name);
        std::fs::write(&lookup_path, lookup.as_bytes()).unwrap();
        info!("Dumped TagHash lookup to {}", lookup_path);

        // Wordlist cross-reference (Option D) — define before index loop
        let wordlist_path_str = "F:\\d2\\alkahest-0.6-main\\wordlist.txt".to_string();
        let word_ref_note = format!("Wordlist reference file: {}; searched terms include 'spawn', 'hive', '.fa_enemy', '.o_spawner', '.o_boss_spawner'; names unavailable directly but terms present for cross-check.", wordlist_path_str);
        let mut lookup_ext = String::new();
        lookup_ext.push_str(&format!("word_reference_file\t{}\n", wordlist_path_str));
        std::fs::write(format!("{}/{}_wordlist_ref.tsv", base, safe_name), lookup_ext).unwrap();
        info!("Dumped wordlist reference note to {}/{}_wordlist_ref.tsv", base, safe_name);

        // Unified JSON index linking inventory + lookup + string lookup
        let mut index_entries = Vec::new();
        for r in refs.iter() {
            let event_name = strings.try_get(r.event_id).unwrap_or_else(|| format!("<EVENT_ID({})>", r.event_id));
            let class_name = strings.try_get(r.class_id).unwrap_or_else(|| format!("<CLASS({})>", r.class_id));
            let bank_path = lookup.split("\n").find(|line| line.starts_with(&format!("0x{:08X}", r.wwise_bank.0))).map(|line| line.split("\t").nth(3).unwrap_or("unknown").to_string()).unwrap_or("unknown".to_string());
            index_entries.push(format!(
                "{{\"component_type\":\"{}\",\"class_id\":\"0x{:08X}\",\"event_id\":{},\"event_name\":\"{}\",\"component_name\":\"{}\",\"context\":\"{}\",\"wwise_bank\":\"{}\",\"bank_path\":\"{}\",\"wem_count\":{},\"wem_streams\":\"{}\",\"word_ref\":\"{}\" }}",
                r.component_type,
                r.class_id,
                r.event_id,
                event_name.replace('"', "\\\""),
                class_name.replace('"', "\\\""),
                r.entity_context.replace('"', "\\\""),
                format!("TagHash {{ pkg_id: {}, entry_index: {} }}", r.wwise_bank.pkg_id(), r.wwise_bank.entry_index()),
                bank_path.replace('"', "\\\""),
                r.wem_streams.len(),
                r.wem_streams.iter().map(|h| format!("0x{:08X}", h.0)).collect::<Vec<_>>().join(","),
                word_ref_note.replace('"', "\\\"")
            ));
        }
        let index_json = format!("[{}]", index_entries.join(","));
        let index_path = format!("{}/{}_index.json", base, safe_name);
        std::fs::write(&index_path, index_json).unwrap();
        // Raw hash lookup attempt (user request)
        let raw_hashes = [
            ("0x80A683DF", "3398v1?"),
            ("0x80A71ABD", "3397v1?"),
            ("0x80F1E9B5", "unknown"),
            ("0xF169FB151CE7AE33", "64-bit hash"),
        ];
        let mut raw_lookup = String::new();
        raw_lookup.push_str("raw_hash\tlabel\tinterpretation\tpackage_lookup_result\tstring_lookup_result\n");
        for (hash_str, label) in raw_hashes {
            let lookup_result = if hash_str.starts_with("0xF") || hash_str.len() > 10 {
                "64-bit: not TagHash format (pkg_id+entry_index)".to_string()
            } else if let Ok(val) = u32::from_str_radix(hash_str.trim_start_matches("0x"), 16) {
                // Try as entry_index with common pkg_ids (679 from inventory, 32 mentioned)
                let with_679 = tiger_pkg::package_manager().get_entry(TagHash::new(679, val as u16)).map(|_| "found_679".to_string()).unwrap_or("not_found_679".to_string());
                let with_32 = tiger_pkg::package_manager().get_entry(TagHash::new(32, val as u16)).map(|_| "found_32".to_string()).unwrap_or("not_found_32".to_string());
                format!("entry_index={}; with_679={}; with_32={}", val, with_679, with_32)
            } else {
                "invalid_hex".to_string()
            };
            let string_result = if hash_str.starts_with("0xF") {
                "FnvHash/64-bit: no direct StringContainer match (keys use different encoding)".to_string()
            } else {
                "decimal_id: StringContainer expects FnvHash; mismatch".to_string()
            };
            raw_lookup.push_str(&format!("{}\t{}\t{}\t{}\t{}\n", hash_str, label, lookup_result, string_result, "see wordlist.txt for reference terms".to_string()));
        }
        let raw_lookup_path = format!("{}/{}_raw_hash_lookup.tsv", base, safe_name);
        std::fs::write(&raw_lookup_path, raw_lookup).unwrap();
        info!("Dumped raw hash lookup to {}", raw_lookup_path);

        info!("Dumped unified JSON index to {}", index_path);

        // Deep package lookup/index: scan all unique audio-related TagHash instances
        let mut pkg_lookup = String::new();
        pkg_lookup.push_str("hash\tpackage_path\tentry_reference\tfile_type\tfile_subtype\tpackage_lookup_result\n");
        let mut pkg_seen = std::collections::HashSet::new();
        for r in refs.iter() {
            for h in [&r.wwise_bank].into_iter().chain(&r.wem_streams) {
                let key = format!("{}:{}", h.pkg_id(), h.entry_index());
                if !pkg_seen.insert(key.clone()) {
                    continue;
                }
                let pkg_path = tiger_pkg::package_manager().package_paths.get(&h.pkg_id())
                    .map(|p| p.name.as_str().to_string())
                    .unwrap_or_else(|| format!("unknown_pkg({})", h.pkg_id()));
                let entry_info = tiger_pkg::package_manager().get_entry(*h)
                    .map(|e| format!("ref=0x{:08X};ftype={};fsub={}", e.reference, e.file_type, e.file_subtype))
                    .unwrap_or_else(|| "not_found".to_string());
                pkg_lookup.push_str(&format!("TagHash {{ pkg_id: {}, entry_index: {} }}\t{}\t{}\t{}\t{}\t{}\n",
                                       h.pkg_id(), h.entry_index(), pkg_path,
                                       format!("0x{:08X}:{:?}", h.pkg_id(), h.entry_index()),
                                       entry_info.split(";").next().unwrap_or("unknown"),
                                       entry_info.split(";").nth(1).unwrap_or("unknown"),
                                       "see package_manager().package_paths + get_entry()".to_string()));
            }
        }
        let pkg_lookup_path = format!("{}/{}_package_index.tsv", base, safe_name);
        std::fs::write(&pkg_lookup_path, pkg_lookup).unwrap();
        info!("Dumped deep package lookup/index to {}", pkg_lookup_path);
    }
}
pub struct ComponentDataListIter<'a> {
    current: Option<&'a SComponentDataNode>,
}

impl<'a> Iterator for ComponentDataListIter<'a> {
    type Item = &'a ComponentData;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_deref();
            &node.data
        })
    }
}
