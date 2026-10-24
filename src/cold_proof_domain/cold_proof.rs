#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[cfg(test)] mod native_binary_tests {
struct Fixture(std::path::PathBuf);
impl Fixture { fn new() -> Self {
let stamp=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
let root=std::env::temp_dir().join(format!("eoie cold binary ü {} {stamp}",std::process::id()));
std::fs::create_dir_all(root.join("state")).unwrap();
std::fs::create_dir_all(root.join("src")).unwrap(); Self(root) } }
impl Drop for Fixture { fn drop(&mut self) { let _=std::fs::remove_dir_all(&self.0); } }
#[test] fn cold_binary_selection_accepts_either_name_and_rejects_ambiguity() { let f=Fixture::new(); assert_eq!(super::eoie_test_cold_selection(f.0.to_str().unwrap()),0); }
#[test] fn windows_cold_proof_binds_the_selected_name_and_preserves_hash_checks() { let f=Fixture::new(); assert_eq!(super::eoie_test_cold_binary_gate(f.0.to_str().unwrap()),0); }
#[test] fn cold_census_rejects_counts_that_wrap_the_signed_binding() { let f=Fixture::new(); assert_eq!(super::eoie_test_cold_census_bounds(f.0.to_str().unwrap()),0); }
#[test] fn cold_proof_renewal_binds_the_platform_binary_and_current_lock_and_rolls_back_without_formal_authority() { let f=Fixture::new(); assert_eq!(super::eoie_test_cold_renewal(f.0.to_str().unwrap()),0); }
#[test] fn family_contract_renewal_rechecks_the_catalog_settlement_and_rolls_back_on_drift() { let f=Fixture::new(); assert_eq!(super::eoie_test_family_renewal(f.0.to_str().unwrap()),0); }
#[test] fn differential_catalog_renewal_requires_the_public_route_and_rolls_back() { let f=Fixture::new(); assert_eq!(super::eoie_test_differential_renewal(f.0.to_str().unwrap()),0); }
}

fn cold_public_binary(root: &Path) -> Result<&'static str, String> {
    let names = eoie_rust_std_fs::rooted_list_names(root)?;
    let presence = i32::from(names.iter().any(|name| name == "eoie")) + 2 * i32::from(names.iter().any(|name| name == "eoie.exe"));
    match eoie_cold_proof_binary_binding(presence) {
        1 => Ok("eoie"),
        2 => Ok("eoie.exe"),
        _ => Err("ColdProofV4 requires exactly one root binary: eoie or eoie.exe".to_owned()),
    }
}
use std::collections::BTreeSet;
use std::path::Path;

fn cold_quoted_fields(line:&str)->Vec<String>{line.split('\"').skip(1).step_by(2).map(str::to_owned).collect()}
fn cold_gate(name:&str,binary:&'static str)->Option<(i32,&'static str)>{match name{"census"=>Some((1,"state/authority_census.spi")),"migration"=>Some((2,"state/migration.spi")),"workspace"=>Some((4,"src/Cargo.lock")),"state"=>Some((8,"state/core.spi")),"binary"=>Some((16,binary)),_=>None}}
fn cold_replay_flags(source:&str)->Result<i32,String>{let line=source.lines().find(|line|line.contains("inl current_cold_replay_flags ()")).ok_or_else(||"ColdProofV4 replay flags are missing".to_owned())?;let value=line.split('=').nth(1).ok_or_else(||"ColdProofV4 replay flags are malformed".to_owned())?.trim().trim_end_matches("u32");value.parse::<i32>().map_err(|_|format!("ColdProofV4 replay flags are malformed: {value}"))}
fn cold_census_values(source:&str)->Result<[usize;10],String>{let line=source.lines().find(|line|line.contains("AuthorityCensus (")).ok_or_else(||"authority census payload is missing".to_owned())?;let body=line.split("AuthorityCensus (").nth(1).ok_or_else(||"authority census payload is malformed".to_owned())?.split(')').next().ok_or_else(||"authority census terminator is missing".to_owned())?;let parsed=body.split(',').map(|value|value.trim().trim_end_matches("u32").parse::<usize>().map_err(|_|format!("authority census value is malformed: {value}"))).collect::<Result<Vec<_>,_>>()?;if parsed.len()!=10{return Err(format!("authority census expects ten values, observed {}",parsed.len()))}if parsed.iter().any(|value|i32::try_from(*value).is_err()){return Err("authority census value exceeds signed comparison range".to_owned())}let mut out=[0usize;10];out.copy_from_slice(&parsed);Ok(out)}

pub fn verify_cold_proof(root:&Path)->Result<(),String>{let root_meta=std::fs::symlink_metadata(root).map_err(|e|format!("cold proof root metadata {}: {e}",root.display()))?;if root_meta.file_type().is_symlink()||!root_meta.is_dir(){return Err(format!("cold proof root must be a real directory: {}",root.display()))}let proof_path=root.join(eoie_rust_std_fs::safe_relative_path("state/cold_proof.spi")?);let source=eoie_rust_std_fs::read_regular_text_limited(&proof_path,128*1024)?;let schema=i32::from(source.lines().any(|line|line.contains("inl current_cold_schema () : cold_receipt_schema = ColdReceiptV4")))*4;let status=i32::from(source.lines().any(|line|line.contains("inl current_cold_proof_status () : cold_receipt_status = ColdReceiptClosed")));if eoie_cold_proof_equal_binding(4,schema)!=1||eoie_cold_proof_equal_binding(1,status)!=1{return Err(format!("ColdProofV4 header rejected schema={schema} status={status}"))}let replay_flags=cold_replay_flags(&source)?;if eoie_cold_proof_equal_binding(15,replay_flags)!=1{return Err(format!("ColdProofV4 replay rejected flags={replay_flags} expected=15"))}let binary=cold_public_binary(root)?;let mut mask=0i32;let mut names=BTreeSet::new();let mut paths=BTreeSet::new();let mut verified=0usize;for name in ["census","migration","workspace","state","binary"]{let(bit,expected_relative)=cold_gate(name,binary).ok_or_else(||format!("unknown cold proof gate: {name}"))?;let needle=format!("inl {name}_gate ()");let line=source.lines().find(|line|line.contains(&needle)).ok_or_else(||format!("ColdProofV4 gate missing: {name}"))?;let fields=cold_quoted_fields(line);if fields.len()!=2{return Err(format!("ColdProofV4 gate malformed: {name}"))}let relative=&fields[0];let expected_hash=&fields[1];let unique=names.insert(name.to_owned())&&paths.insert(relative.clone());if relative!=expected_relative{return Err(format!("ColdProofV4 gate path mismatch name={name} expected={expected_relative} observed={relative}"))}
if expected_hash.len()!=64||!expected_hash.bytes().all(|byte|byte.is_ascii_hexdigit()){return Err(format!("ColdProofV4 gate hash malformed: {name}"))}let path=root.join(eoie_rust_std_fs::safe_relative_path(relative)?);let bytes=eoie_rust_std_fs::read_regular_limited(&path,128*1024*1024)?;let observed=eoie_proxy_search::inspection_file_sha256(&path)?;let evidence_mask=1+2*i32::from(observed==*expected_hash)+4*i32::from(unique);if eoie_cold_proof_equal_binding(7,evidence_mask)!=1{return Err(format!("ColdProofV4 gate rejected name={name} path={} expected={expected_hash} observed={observed} bytes={}",path.display(),bytes.len()))}mask|=bit;verified+=1}if eoie_cold_proof_equal_binding(31,mask)!=1||eoie_cold_proof_equal_binding(5,verified as i32)!=1{return Err(format!("ColdProofV4 gate shape rejected mask={mask} count={verified}"))}let census_path=root.join(eoie_rust_std_fs::safe_relative_path("state/authority_census.spi")?);let census_text=eoie_rust_std_fs::read_regular_text_limited(&census_path,64*1024)?;let expected_counts=cold_census_values(&census_text)?;let observed_counts=eoie_source_topology::source_topology_counts(root)?;for(index,(expected,observed))in expected_counts.iter().zip(observed_counts.iter()).enumerate(){let expected_value=i32::try_from(*expected).map_err(|_|format!("ColdProofV4 census count out of range index={index} expected={expected}"))?;let observed_value=i32::try_from(*observed).map_err(|_|format!("ColdProofV4 census count out of range index={index} observed={observed}"))?;if eoie_cold_proof_equal_binding(expected_value,observed_value)!=1{return Err(format!("ColdProofV4 census drift index={index} expected={expected} observed={observed}"))}}verify_differential_catalog(root)?;verify_family_contracts(root)?;Ok(())}

pub fn verify_differential_catalog(root:&Path)->Result<i32,String>{let receipt=eoie_rust_std_fs::read_regular_text_limited(&root.join("state/differential_catalog.spi"),128*1024)?;if!receipt.contains("current_differential_catalog_schema () : differential_catalog_schema = DifferentialCatalogV1")||!receipt.contains("current_differential_catalog_status () : differential_catalog_status = DifferentialCatalogClosed")||!receipt.contains("current_differential_semantic_flags () : u32 = 31u32"){return Err("differential catalog header rejected".to_owned())}for(name,expected)in[("settlement","state/differential.spi"),("catalog","state/legacy_catalog.spi"),("contract","src/differential_normalization_contract/model.spi"),("route","src/legacy_operations_entry/legacy_terminal.spi")]{let needle=format!("inl {name}_gate ()");let line=receipt.lines().find(|line|line.contains(&needle)).ok_or_else(||format!("differential catalog gate missing: {name}"))?;let fields=cold_quoted_fields(line);if fields.len()!=2||fields[0]!=expected{return Err(format!("differential catalog gate shape rejected: {name}"))}let path=root.join(eoie_rust_std_fs::safe_relative_path(expected)?);let observed=eoie_proxy_search::inspection_file_sha256(&path)?;if fields[1]!=observed{return Err(format!("differential catalog hash drift name={name} expected={} observed={observed}",fields[1]))}}let settlement=eoie_rust_std_fs::read_regular_text_limited(&root.join("state/differential.spi"),256*1024)?;if!settlement.contains("DifferentialSettlement (10u32, 9u32"){return Err("differential settlement 10/9 shape missing".to_owned())}for marker in["NormalizedParityObserved","DeterministicReceiptObserved","NormalizedMismatchObserved","MissingInputRejected","EscapingReceiptRejected"]{if!settlement.contains(marker){return Err(format!("differential settlement evidence missing: {marker}"))}}let catalog=eoie_rust_std_fs::read_regular_text_limited(&root.join("state/legacy_catalog.spi"),256*1024)?;for marker in["cataloged_families () : u32 = 36u32","decided_families () : u32 = 36u32","catalog_complete () : bool = true"]{if!catalog.contains(marker){return Err(format!("legacy catalog invariant missing: {marker}"))}}let contract=eoie_rust_std_fs::read_regular_text_limited(&root.join("src/differential_normalization_contract/model.spi"),256*1024)?;for marker in["public_differential_plan","NormalizeText","CompareNormalized","CommitDifferentialReceipt"]{if!contract.contains(marker){return Err(format!("differential contract marker missing: {marker}"))}}let route=eoie_rust_std_fs::read_regular_text_limited(&root.join("src/legacy_operations_entry/legacy_terminal.spi"),2*1024*1024)?;for marker in["fn eoie_normalized_text","fn eoie_fnv1a","eoie_differential_compare"]{if!route.contains(marker){return Err(format!("public differential route marker missing: {marker}"))}}Ok(1)}

pub fn renew_cold_proof_text(root:&Path,original:&str)->Result<String,String>{let binary=cold_public_binary(root)?;let mut updated=original.to_owned();for name in["census","migration","workspace","state","binary"]{let(_,relative)=cold_gate(name,binary).ok_or_else(||format!("unknown cold proof gate: {name}"))?;let needle=format!("inl {name}_gate ()");let line=original.lines().find(|line|line.contains(&needle)).ok_or_else(||format!("ColdProofV4 gate missing during renewal: {name}"))?;let fields=cold_quoted_fields(line);if fields.len()!=2{return Err(format!("ColdProofV4 gate malformed during renewal: {name}"))}let observed=eoie_proxy_search::inspection_file_sha256(&root.join(eoie_rust_std_fs::safe_relative_path(relative)?))?;let renewed=line.replacen(&format!("{:?}",fields[0]),&format!("{relative:?}"),1).replacen(&format!("{:?}",fields[1]),&format!("{observed:?}"),1);updated=updated.replacen(line,&renewed,1)}Ok(updated)}
pub fn renew_cold_proof(root:&Path)->Result<(),String>{let proof_path=root.join(eoie_rust_std_fs::safe_relative_path("state/cold_proof.spi")?);let original=eoie_rust_std_fs::read_regular_text_limited(&proof_path,128*1024)?;let updated=renew_cold_proof_text(root,&original)?;eoie_rust_std_fs::atomic_write(&proof_path,updated.as_bytes())?;let verdict=verify_formal_authority(root).and_then(|_|{let binary=cold_public_binary(root)?;let observed=eoie_proxy_search::inspection_file_sha256(&root.join(binary))?;let receipt=eoie_rust_std_fs::read_regular_text_limited(&root.join("state/cold_rebuild.spi"),128*1024)?;let product=cold_rebuild_text(&receipt,"current_product_a_sha256")?;if observed==product{Ok(())}else{Err(format!("ColdProofV4 binary {binary} sha256={observed} is not the cold rebuild product {product}"))}});if let Err(error)=verdict{let _=eoie_rust_std_fs::atomic_write(&proof_path,original.as_bytes());return Err(format!("ColdProofV4 renewal rolled back: {error}"))}Ok(())}

pub fn verify_family_contracts(root:&Path)->Result<i32,String>{let receipt=eoie_rust_std_fs::read_regular_text_limited(&root.join("state/family_contracts.spi"),128*1024)?;for marker in["current_family_contract_schema () : family_contract_schema = FamilyContractsV1","current_family_contract_status () : family_contract_status = FamilyContractsClosed","current_family_contract_semantic_flags () : u32 = 31u32","current_family_count () : u32 = 36u32","current_family_decision_count () : u32 = 36u32","current_unsettled_family_count () : u32 = 0u32"]{if!receipt.contains(marker){return Err(format!("family contract header rejected: {marker}"))}}for(name,expected)in[("catalog","state/legacy_catalog.spi"),("migration","state/migration.spi"),("runtime","state/runtime_drift.spi"),("adapter","state/adapter_inventory.spi")]{let needle=format!("inl {name}_gate ()");let line=receipt.lines().find(|line|line.contains(&needle)).ok_or_else(||format!("family contract gate missing: {name}"))?;let fields=cold_quoted_fields(line);if fields.len()!=2||fields[0]!=expected{return Err(format!("family contract gate shape rejected: {name}"))}let path=root.join(eoie_rust_std_fs::safe_relative_path(expected)?);let observed=eoie_proxy_search::inspection_file_sha256(&path)?;if fields[1]!=observed{return Err(format!("family contract hash drift name={name} expected={} observed={observed}",fields[1]))}}let catalog=eoie_rust_std_fs::read_regular_text_limited(&root.join("state/legacy_catalog.spi"),512*1024)?;let family_names=catalog.lines().filter_map(|line|line.split("CapabilityFamily (\"").nth(1).and_then(|tail|tail.split('\"').next()).map(str::to_owned)).collect::<Vec<_>>();let decision_names=catalog.lines().filter_map(|line|line.split("LegacyDecision (\"").nth(1).and_then(|tail|tail.split('\"').next()).map(str::to_owned)).collect::<Vec<_>>();let family_set=family_names.iter().cloned().collect::<BTreeSet<_>>();let decision_set=decision_names.iter().cloned().collect::<BTreeSet<_>>();let unsettled=catalog.lines().filter(|line|line.contains("CapabilityFamily (\"")&&(line.contains(", Deferred,")||line.contains(", NeedsEvidence,"))).count();if family_names.len()!=36||decision_names.len()!=36||family_set.len()!=36||decision_set.len()!=36||family_set!=decision_set||unsettled!=0{return Err(format!("family contract catalog mismatch families={} unique_families={} decisions={} unique_decisions={} unsettled={unsettled}",family_names.len(),family_set.len(),decision_names.len(),decision_set.len()))}for line in catalog.lines().filter(|line|line.contains("CapabilityFamily (\"")){let name=line.split("CapabilityFamily (\"").nth(1).and_then(|tail|tail.split('\"').next()).ok_or_else(||"family contract name parse failed".to_owned())?;if line.contains(", DeliberatelyPruned,"){let marker=format!("LegacyDecision (\"{name}\", PruneNow");if!catalog.contains(&marker){return Err(format!("family contract prune decision mismatch: {name}"))}}else if line.contains(", Absorbed,"){let now=format!("LegacyDecision (\"{name}\", PortNow");let after=format!("LegacyDecision (\"{name}\", PortAfterEvidence");if!catalog.contains(&now)&&!catalog.contains(&after){return Err(format!("family contract absorbed decision mismatch: {name}"))}}else{return Err(format!("family contract unsettled disposition: {name}"))}}Ok(1)}

fn cold_renew_gates(root:&Path,relative:&str,gates:&[(&str,&str)],verify:fn(&Path)->Result<i32,String>)->Result<Option<String>,String>{if verify(root).is_ok(){return Ok(None)}let path=root.join(eoie_rust_std_fs::safe_relative_path(relative)?);let original=eoie_rust_std_fs::read_regular_text_limited(&path,128*1024)?;let mut updated=original.clone();for(name,expected)in gates{let needle=format!("inl {name}_gate ()");let line=original.lines().find(|line|line.contains(&needle)).ok_or_else(||format!("{relative} gate missing during renewal: {name}"))?;let fields=cold_quoted_fields(line);if fields.len()!=2||fields[0]!=*expected{return Err(format!("{relative} gate shape rejected during renewal: {name}"))}let observed=eoie_proxy_search::inspection_file_sha256(&root.join(eoie_rust_std_fs::safe_relative_path(expected)?))?;updated=updated.replacen(line,&line.replacen(&format!("{:?}",fields[1]),&format!("{observed:?}"),1),1)}eoie_rust_std_fs::atomic_write(&path,updated.as_bytes())?;if let Err(error)=verify(root){let _=eoie_rust_std_fs::atomic_write(&path,original.as_bytes());return Err(format!("{relative} renewal rolled back: {error}"))}Ok(Some(original))}
pub fn observe_differential_route(root:&Path)->Result<u32,String>{let binary=root.join(cold_public_binary(root)?);let work=root.join(".cache").join(format!("differential-renewal-{}",std::process::id()));let _=std::fs::remove_dir_all(&work);std::fs::create_dir_all(&work).map_err(|e|format!("differential route workspace {}: {e}",work.display()))?;let observed=(||{for(name,text)in[("oracle.txt","route  \r\nparity\n"),("direct.txt","route\nparity"),("drift.txt","route\ndrift")]{std::fs::write(work.join(name),text).map_err(|e|format!("differential route fixture {name}: {e}"))?}let run=|oracle:&str,direct:&str,receipt:&str|std::process::Command::new(&binary).args(["proxy","differential-compare"]).arg(&work).args([oracle,direct,receipt]).output().map_err(|e|format!("differential route {}: {e}",binary.display()));let parity=run("oracle.txt","direct.txt","a.txt")?;let repeat=run("oracle.txt","direct.txt","b.txt")?;let first=std::fs::read(work.join("a.txt")).unwrap_or_default();let second=std::fs::read(work.join("b.txt")).unwrap_or_default();let drift=run("oracle.txt","drift.txt","m.txt")?;let missing=run("absent.txt","direct.txt","n.txt")?;let escape=run("oracle.txt","direct.txt","../differential-escape.txt")?;let said=|o:&std::process::Output,text:&str|String::from_utf8_lossy(&o.stdout).contains(text)||String::from_utf8_lossy(&o.stderr).contains(text);let mut flags=0u32;if parity.status.success()&&first.starts_with(b"route=normalized-text\n"){flags|=1}if repeat.status.success()&&!first.is_empty()&&first==second{flags|=2}if!drift.status.success()&&said(&drift,"differential mismatch")&&!work.join("m.txt").exists(){flags|=4}if!missing.status.success()&&!work.join("n.txt").exists(){flags|=8}if!escape.status.success()&&!root.join(".cache/differential-escape.txt").exists(){flags|=16}Ok(flags)})();let _=std::fs::remove_dir_all(&work);observed}
pub fn renew_differential_catalog(root:&Path)->Result<Option<String>,String>{if verify_differential_catalog(root).is_ok(){return Ok(None)}let flags=observe_differential_route(root)?;if flags!=31{return Err(format!("differential route scenarios rejected flags={flags} expected=31"))}cold_renew_gates(root,"state/differential_catalog.spi",&[("settlement","state/differential.spi"),("catalog","state/legacy_catalog.spi"),("contract","src/differential_normalization_contract/model.spi"),("route","src/legacy_operations_entry/legacy_terminal.spi")],verify_differential_catalog)}
pub fn renew_family_contracts(root:&Path)->Result<Option<String>,String>{cold_renew_gates(root,"state/family_contracts.spi",&[("catalog","state/legacy_catalog.spi"),("migration","state/migration.spi"),("runtime","state/runtime_drift.spi"),("adapter","state/adapter_inventory.spi")],verify_family_contracts)}
fn cold_restore_receipt(root:&Path,relative:&str,original:&Option<String>){if let Some(text)=original{let _=eoie_rust_std_fs::atomic_write(&root.join(relative),text.as_bytes());}}
pub fn refresh_cold_proof_census_gate(root:&Path)->Result<(),String>{let rolled=|error:String|format!("ColdProofV4 renewal rolled back: {error}");let differential=renew_differential_catalog(root).map_err(rolled)?;let family=match renew_family_contracts(root){Ok(original)=>original,Err(error)=>{cold_restore_receipt(root,"state/differential_catalog.spi",&differential);return Err(rolled(error))}};if let Err(error)=renew_cold_proof(root){cold_restore_receipt(root,"state/differential_catalog.spi",&differential);cold_restore_receipt(root,"state/family_contracts.spi",&family);return Err(error)}Ok(())}


fn cold_rebuild_u32(source:&str,name:&str)->Result<u32,String>{let needle=format!("inl {name} () : u32 = " );let line=source.lines().find(|line|line.contains(&needle)).ok_or_else(||format!("cold rebuild numeric marker missing: {name}"))?;let value=line.split('=').nth(1).ok_or_else(||format!("cold rebuild numeric marker malformed: {name}"))?.trim().trim_end_matches("u32");value.parse::<u32>().map_err(|_|format!("cold rebuild numeric marker malformed: {name}={value}"))}
fn cold_rebuild_text(source:&str,name:&str)->Result<String,String>{let needle=format!("inl {name} ()");let line=source.lines().find(|line|line.contains(&needle)).ok_or_else(||format!("cold rebuild text marker missing: {name}"))?;let fields=cold_quoted_fields(line);if fields.len()!=1{return Err(format!("cold rebuild text marker malformed: {name}"))}Ok(fields[0].clone())}


pub fn verify_public_cold_rebuild(root:&Path)->Result<i32,String>{let receipt=eoie_rust_std_fs::read_regular_text_limited(&root.join("state/cold_rebuild.spi"),128*1024)?;for marker in["current_cold_rebuild_schema () : cold_rebuild_schema = ColdRebuildV1","current_cold_rebuild_status () : cold_rebuild_status = ColdRebuildClosed"]{if!receipt.contains(marker){return Err(format!("cold rebuild header rejected: {marker}"))}}let owners=cold_rebuild_u32(&receipt,"current_owner_count")?;let width=cold_rebuild_u32(&receipt,"current_shard_width")?;let shards=cold_rebuild_u32(&receipt,"current_shard_count")?;let flags=cold_rebuild_u32(&receipt,"current_gate_flags")?;let max_owner_ms=cold_rebuild_u32(&receipt,"current_max_owner_compile_ms")?;let max_gate_ms=cold_rebuild_u32(&receipt,"current_max_global_gate_ms")?;let shape_error=||format!("cold rebuild shape rejected owners={owners} width={width} shards={shards} flags={flags}");let checked_owners=i32::try_from(owners).map_err(|_|shape_error())?;let checked_width=i32::try_from(width).map_err(|_|shape_error())?;let checked_shards=i32::try_from(shards).map_err(|_|shape_error())?;let expected_shards=eoie_cold_rebuild_expected_shards(checked_owners,checked_width);if expected_shards<1||checked_shards!=expected_shards||flags!=63{return Err(shape_error())}let declared=eoie_source_map::source_rebuild_pairs(root)?.len();if declared!=owners as usize{return Err(format!("cold rebuild owner inventory drift receipt={owners} declared={declared}"))}if max_owner_ms==0||max_owner_ms>15000||max_gate_ms==0||max_gate_ms>30000{return Err(format!("cold rebuild performance budget rejected owner_ms={max_owner_ms} global_ms={max_gate_ms}"))}for(name,expected)in[("contract","src/cold_rebuild_contract/model.spi"),("bindings","src/cold_rebuild_contract/bindings.spi"),("matrix","src/eoie_cold_rebuild_matrix/main.spi"),("toolchain","state/toolchain_identity.spi")]{let needle=format!("inl {name}_gate ()");let line=receipt.lines().find(|line|line.contains(&needle)).ok_or_else(||format!("cold rebuild gate missing: {name}"))?;let fields=cold_quoted_fields(line);if fields.len()!=2||fields[0]!=expected{return Err(format!("cold rebuild gate shape rejected: {name}"))}let path=root.join(eoie_rust_std_fs::safe_relative_path(expected)?);let observed=eoie_proxy_search::inspection_file_sha256(&path)?;if fields[1]!=observed{return Err(format!("cold rebuild gate hash drift name={name} expected={} observed={observed}",fields[1]))}}let current_source=eoie_proxy_search::inspection_tree_sha256(root,"src")?;let source_a=cold_rebuild_text(&receipt,"current_source_a_sha256")?;let source_b=cold_rebuild_text(&receipt,"current_source_b_sha256")?;let product_a=cold_rebuild_text(&receipt,"current_product_a_sha256")?;let product_b=cold_rebuild_text(&receipt,"current_product_b_sha256")?;for(name,value)in[("source-a",&source_a),("source-b",&source_b),("product-a",&product_a),("product-b",&product_b)]{if!cold_rebuild_hash(value){return Err(format!("cold rebuild hash malformed: {name}"))}}if source_a!=current_source||source_b!=current_source{return Err(format!("cold rebuild source drift current={current_source} a={source_a} b={source_b}"))}if product_a!=product_b{return Err(format!("cold rebuild product mismatch a={product_a} b={product_b}"))}let toolchain=eoie_rust_std_fs::read_regular_text_limited(&root.join("state/toolchain_identity.spi"),128*1024)?;for name in["current_rustc_sha256","current_compiler_entry_sha256","current_compiler_facade_sha256"]{let value=cold_rebuild_text(&receipt,name)?;if!cold_rebuild_hash(&value)||!toolchain.contains(&value){return Err(format!("cold rebuild tool identity rejected: {name}"))}}Ok(1)}
pub fn verify_formal_authority(root:&Path)->Result<(),String>{verify_cold_proof(root)?;verify_public_cold_rebuild(root)?;Ok(())}


#[cfg(test)]
#[path = "rebuild_receipt_tests.rs"]
mod rebuild_receipt_tests;

fn family_renewal_0(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/legacy_catalog.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v3));
    let mut v5: Rc<str> = std::rc::Rc::<str>::from((0..36u64).map(|i|{let q=char::from(34u8).to_string();let n=char::from(10u8).to_string();let f=i.to_string();["CapabilityFamily (",&q,"f",&f,&q,", Absorbed, x)",&n,"LegacyDecision (",&q,"f",&f,&q,", PortNow, x)",&n].concat()}).collect::<String>());
    std::fs::write(&*v4,&*v5).unwrap();
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/migration.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("migration fixture"); } LIT.with(|lit| lit.clone()) };
    std::fs::write(&*v8,&*v9).unwrap();
    let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/runtime_drift.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("runtime drift fixture"); } LIT.with(|lit| lit.clone()) };
    std::fs::write(&*v12,&*v13).unwrap();
    let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/adapter_inventory.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v15));
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("adapter fixture"); } LIT.with(|lit| lit.clone()) };
    std::fs::write(&*v16,&*v17).unwrap();
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("catalog"); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl catalog"); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl catalog_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl catalog_gate () = (\"state/legacy_catalog.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl catalog_gate () = (\"state/legacy_catalog.spi\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0000000000000000000000000000000000000000000000000000000000000000"); } LIT.with(|lit| lit.clone()) };
    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\")\n"); } LIT.with(|lit| lit.clone()) };
    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0000000000000000000000000000000000000000000000000000000000000000\")\n"); } LIT.with(|lit| lit.clone()) };
    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl catalog_gate () = (\"state/legacy_catalog.spi\", \"0000000000000000000000000000000000000000000000000000000000000000\")\n"); } LIT.with(|lit| lit.clone()) };
    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("migration"); } LIT.with(|lit| lit.clone()) };
    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl migration"); } LIT.with(|lit| lit.clone()) };
    let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl migration_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl migration_gate () = (\"state/migration.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl migration_gate () = (\"state/migration.spi\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0000000000000000000000000000000000000000000000000000000000000000\")\n"); } LIT.with(|lit| lit.clone()) };
    let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl migration_gate () = (\"state/migration.spi\", \"0000000000000000000000000000000000000000000000000000000000000000\")\n"); } LIT.with(|lit| lit.clone()) };
    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl catalog_gate () = (\"state/legacy_catalog.spi\", \"0000000000000000000000000000000000000000000000000000000000000000\")\ninl migration_gate () = (\"state/migration.spi\", \"0000000000000000000000000000000000000000000000000000000000000000\")\n"); } LIT.with(|lit| lit.clone()) };
    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("runtime"); } LIT.with(|lit| lit.clone()) };
    let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl runtime"); } LIT.with(|lit| lit.clone()) };
    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl runtime_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl runtime_gate () = (\"state/runtime_drift.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl runtime_gate () = (\"state/runtime_drift.spi\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0000000000000000000000000000000000000000000000000000000000000000\")\n"); } LIT.with(|lit| lit.clone()) };
    let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl runtime_gate () = (\"state/runtime_drift.spi\", \"0000000000000000000000000000000000000000000000000000000000000000\")\n"); } LIT.with(|lit| lit.clone()) };
    let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl catalog_gate () = (\"state/legacy_catalog.spi\", \"0000000000000000000000000000000000000000000000000000000000000000\")\ninl migration_gate () = (\"state/migration.spi\", \"0000000000000000000000000000000000000000000000000000000000000000\")\ninl runtime_gate () = (\"state/runtime_drift.spi\", \"0000000000000000000000000000000000000000000000000000000000000000\")\n"); } LIT.with(|lit| lit.clone()) };
    let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("adapter"); } LIT.with(|lit| lit.clone()) };
    let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl adapter"); } LIT.with(|lit| lit.clone()) };
    let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl adapter_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl adapter_gate () = (\"state/adapter_inventory.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl adapter_gate () = (\"state/adapter_inventory.spi\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0000000000000000000000000000000000000000000000000000000000000000\")\n"); } LIT.with(|lit| lit.clone()) };
    let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl adapter_gate () = (\"state/adapter_inventory.spi\", \"0000000000000000000000000000000000000000000000000000000000000000\")\n"); } LIT.with(|lit| lit.clone()) };
    let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl catalog_gate () = (\"state/legacy_catalog.spi\", \"0000000000000000000000000000000000000000000000000000000000000000\")\ninl migration_gate () = (\"state/migration.spi\", \"0000000000000000000000000000000000000000000000000000000000000000\")\ninl runtime_gate () = (\"state/runtime_drift.spi\", \"0000000000000000000000000000000000000000000000000000000000000000\")\ninl adapter_gate () = (\"state/adapter_inventory.spi\", \"0000000000000000000000000000000000000000000000000000000000000000\")\n"); } LIT.with(|lit| lit.clone()) };
    let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl current_family_contract_schema () : family_contract_schema = FamilyContractsV1\ninl current_family_contract_status () : family_contract_status = FamilyContractsClosed\ninl current_family_contract_semantic_flags () : u32 = 31u32\ninl current_family_count () : u32 = 36u32\ninl current_family_decision_count () : u32 = 36u32\ninl current_unsettled_family_count () : u32 = 0u32\n"); } LIT.with(|lit| lit.clone()) };
    let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl current_family_contract_schema () : family_contract_schema = FamilyContractsV1\ninl current_family_contract_status () : family_contract_status = FamilyContractsClosed\ninl current_family_contract_semantic_flags () : u32 = 31u32\ninl current_family_count () : u32 = 36u32\ninl current_family_decision_count () : u32 = 36u32\ninl current_unsettled_family_count () : u32 = 0u32\ninl catalog_gate () = (\"state/legacy_catalog.spi\", \"0000000000000000000000000000000000000000000000000000000000000000\")\ninl migration_gate () = (\"state/migration.spi\", \"0000000000000000000000000000000000000000000000000000000000000000\")\ninl runtime_gate () = (\"state/runtime_drift.spi\", \"0000000000000000000000000000000000000000000000000000000000000000\")\ninl adapter_gate () = (\"state/adapter_inventory.spi\", \"0000000000000000000000000000000000000000000000000000000000000000\")\n"); } LIT.with(|lit| lit.clone()) };
    let mut v56: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/family_contracts.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v58: Rc<str> = Rc::<str>::from(format!("{}{}", v56, v57));
    std::fs::write(&*v58,&*v55).unwrap();
    let mut v59: bool = renew_family_contracts(std::path::Path::new(&*v0)).unwrap().is_some();
    if v59 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v60: bool = verify_family_contracts(std::path::Path::new(&*v0)).is_ok();
    if v60 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v61: bool = renew_family_contracts(std::path::Path::new(&*v0)).unwrap().is_none();
    if v61 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v62: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v63: Rc<str> = Rc::<str>::from(format!("{}{}", v62, v3));
    let mut v64: Rc<str> = std::rc::Rc::<str>::from((0..35u64).map(|i|{let q=char::from(34u8).to_string();let n=char::from(10u8).to_string();let f=i.to_string();["CapabilityFamily (",&q,"f",&f,&q,", Absorbed, x)",&n,"LegacyDecision (",&q,"f",&f,&q,", PortNow, x)",&n].concat()}).collect::<String>());
    std::fs::write(&*v63,&*v64).unwrap();
    std::fs::write(&*v58,&*v55).unwrap();
    let mut v65: Rc<str> = std::rc::Rc::<str>::from(renew_family_contracts(std::path::Path::new(&*v0)).unwrap_err());
    let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("family contract catalog mismatch"); } LIT.with(|lit| lit.clone()) };
    let mut v67: bool = v65.contains(&*v66);
    if v67 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v68: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v58).unwrap());
    let mut v69: bool = v68.clone() == v55.clone();
    if v69 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    0i32
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        family_renewal_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn differential_renewal_1(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie.exe"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v3));
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("not an executable differential route"); } LIT.with(|lit| lit.clone()) };
    std::fs::write(&*v4,&*v5).unwrap();
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/differential_catalog.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl current_differential_catalog_schema () : differential_catalog_schema = DifferentialCatalogV1\n"); } LIT.with(|lit| lit.clone()) };
    std::fs::write(&*v8,&*v9).unwrap();
    let mut v10: Rc<str> = std::rc::Rc::<str>::from(renew_differential_catalog(std::path::Path::new(&*v0)).unwrap_err());
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("differential route"); } LIT.with(|lit| lit.clone()) };
    let mut v12: bool = v10.contains(&*v11);
    if v12 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v13: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v8).unwrap());
    let mut v14: bool = v13.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl current_differential_catalog_schema () : differential_catalog_schema = DifferentialCatalogV1\n"); } LIT.with(|lit| lit.clone()) };
    if v14 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v15: Rc<str> = std::rc::Rc::<str>::from(refresh_cold_proof_census_gate(std::path::Path::new(&*v0)).unwrap_err());
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ColdProofV4 renewal rolled back"); } LIT.with(|lit| lit.clone()) };
    let mut v17: bool = v15.contains(&*v16);
    if v17 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v18: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v8).unwrap());
    let mut v19: bool = v18.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl current_differential_catalog_schema () : differential_catalog_schema = DifferentialCatalogV1\n"); } LIT.with(|lit| lit.clone()) };
    if v19 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    0i32
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        differential_renewal_1(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn selection_2(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(cold_public_binary(std::path::Path::new(&*v0)).unwrap_or(""));
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v2 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v3));
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie.exe"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v5));
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("windows binary"); } LIT.with(|lit| lit.clone()) };
    std::fs::write(&*v6,&*v7).unwrap();
    let mut v8: Rc<str> = std::rc::Rc::<str>::from(cold_public_binary(std::path::Path::new(&*v0)).unwrap_or(""));
    let mut v9: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie.exe"); } LIT.with(|lit| lit.clone()) };
    if v9 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v3));
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unix binary"); } LIT.with(|lit| lit.clone()) };
    std::fs::write(&*v12,&*v13).unwrap();
    let mut v14: Rc<str> = std::rc::Rc::<str>::from(cold_public_binary(std::path::Path::new(&*v0)).unwrap_or(""));
    let mut v15: bool = v14.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v15 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v3));
    let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v5));
    std::fs::remove_file(&*v17).unwrap();
    let mut v18: Rc<str> = std::rc::Rc::<str>::from(cold_public_binary(std::path::Path::new(&*v0)).unwrap_or(""));
    let mut v19: bool = v18.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie"); } LIT.with(|lit| lit.clone()) };
    if v19 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    0i32
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        selection_2(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn binary_gate_3(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie.exe"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v3));
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("native binary payload"); } LIT.with(|lit| lit.clone()) };
    std::fs::write(&*v4,&*v5).unwrap();
    let mut v6: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v4)).unwrap());
    let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/authority_census.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("deliberately incomplete later census fixture"); } LIT.with(|lit| lit.clone()) };
    std::fs::write(&*v9,&*v10).unwrap();
    let mut v11: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v9)).unwrap());
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("census"); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census"); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census_gate () = (\"state/authority_census.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census_gate () = (\"state/authority_census.spi\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\")\n"); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v20));
    let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v21));
    let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/migration.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
    std::fs::write(&*v25,&*v10).unwrap();
    let mut v26: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v25)).unwrap());
    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("migration"); } LIT.with(|lit| lit.clone()) };
    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl migration"); } LIT.with(|lit| lit.clone()) };
    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl migration_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl migration_gate () = (\"state/migration.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl migration_gate () = (\"state/migration.spi\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v32: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v20));
    let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
    let mut v34: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/Cargo.lock"); } LIT.with(|lit| lit.clone()) };
    let mut v36: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v35));
    std::fs::write(&*v36,&*v10).unwrap();
    let mut v37: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v36)).unwrap());
    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("workspace"); } LIT.with(|lit| lit.clone()) };
    let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl workspace"); } LIT.with(|lit| lit.clone()) };
    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl workspace_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl workspace_gate () = (\"src/Cargo.lock"); } LIT.with(|lit| lit.clone()) };
    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl workspace_gate () = (\"src/Cargo.lock\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v43: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v20));
    let mut v44: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v43));
    let mut v45: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/core.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v47: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v46));
    std::fs::write(&*v47,&*v10).unwrap();
    let mut v48: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v47)).unwrap());
    let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state"); } LIT.with(|lit| lit.clone()) };
    let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl state"); } LIT.with(|lit| lit.clone()) };
    let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl state_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl state_gate () = (\"state/core.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl state_gate () = (\"state/core.spi\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v54: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v20));
    let mut v55: Rc<str> = Rc::<str>::from(format!("{}{}", v53, v54));
    let mut v56: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/cold_proof.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v58: Rc<str> = Rc::<str>::from(format!("{}{}", v56, v57));
    let mut v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl current_cold_schema () : cold_receipt_schema = ColdReceiptV4\ninl current_cold_proof_status () : cold_receipt_status = ColdReceiptClosed\ninl current_cold_replay_flags () : u32 = 15u32\n"); } LIT.with(|lit| lit.clone()) };
    let mut v60: Rc<str> = Rc::<str>::from(format!("{}{}", v59, v22));
    let mut v61: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v33));
    let mut v62: Rc<str> = Rc::<str>::from(format!("{}{}", v61, v44));
    let mut v63: Rc<str> = Rc::<str>::from(format!("{}{}", v62, v55));
    let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binary"); } LIT.with(|lit| lit.clone()) };
    let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl binary"); } LIT.with(|lit| lit.clone()) };
    let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl binary_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie"); } LIT.with(|lit| lit.clone()) };
    let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl binary_gate () = (\"eoie"); } LIT.with(|lit| lit.clone()) };
    let mut v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl binary_gate () = (\"eoie\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v70: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v20));
    let mut v71: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v70));
    let mut v72: Rc<str> = Rc::<str>::from(format!("{}{}", v63, v71));
    std::fs::write(&*v58,&*v72).unwrap();
    let mut v73: Rc<str> = std::rc::Rc::<str>::from(verify_cold_proof(std::path::Path::new(&*v0)).unwrap_err());
    let mut v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected=eoie.exe observed=eoie"); } LIT.with(|lit| lit.clone()) };
    let mut v75: bool = v73.contains(&*v74);
    if v75 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v76: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v77: Rc<str> = Rc::<str>::from(format!("{}{}", v76, v8));
    std::fs::write(&*v77,&*v10).unwrap();
    let mut v78: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v77)).unwrap());
    let mut v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census"); } LIT.with(|lit| lit.clone()) };
    let mut v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census_gate () = (\"state/authority_census.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census_gate () = (\"state/authority_census.spi\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v83: Rc<str> = Rc::<str>::from(format!("{}{}", v78, v20));
    let mut v84: Rc<str> = Rc::<str>::from(format!("{}{}", v82, v83));
    let mut v85: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v86: Rc<str> = Rc::<str>::from(format!("{}{}", v85, v24));
    std::fs::write(&*v86,&*v10).unwrap();
    let mut v87: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v86)).unwrap());
    let mut v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl migration"); } LIT.with(|lit| lit.clone()) };
    let mut v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl migration_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl migration_gate () = (\"state/migration.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl migration_gate () = (\"state/migration.spi\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v92: Rc<str> = Rc::<str>::from(format!("{}{}", v87, v20));
    let mut v93: Rc<str> = Rc::<str>::from(format!("{}{}", v91, v92));
    let mut v94: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v95: Rc<str> = Rc::<str>::from(format!("{}{}", v94, v35));
    std::fs::write(&*v95,&*v10).unwrap();
    let mut v96: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v95)).unwrap());
    let mut v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl workspace"); } LIT.with(|lit| lit.clone()) };
    let mut v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl workspace_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl workspace_gate () = (\"src/Cargo.lock"); } LIT.with(|lit| lit.clone()) };
    let mut v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl workspace_gate () = (\"src/Cargo.lock\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v101: Rc<str> = Rc::<str>::from(format!("{}{}", v96, v20));
    let mut v102: Rc<str> = Rc::<str>::from(format!("{}{}", v100, v101));
    let mut v103: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v104: Rc<str> = Rc::<str>::from(format!("{}{}", v103, v46));
    std::fs::write(&*v104,&*v10).unwrap();
    let mut v105: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v104)).unwrap());
    let mut v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl state"); } LIT.with(|lit| lit.clone()) };
    let mut v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl state_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl state_gate () = (\"state/core.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl state_gate () = (\"state/core.spi\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v110: Rc<str> = Rc::<str>::from(format!("{}{}", v105, v20));
    let mut v111: Rc<str> = Rc::<str>::from(format!("{}{}", v109, v110));
    let mut v112: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v113: Rc<str> = Rc::<str>::from(format!("{}{}", v112, v57));
    let mut v114: Rc<str> = Rc::<str>::from(format!("{}{}", v59, v84));
    let mut v115: Rc<str> = Rc::<str>::from(format!("{}{}", v114, v93));
    let mut v116: Rc<str> = Rc::<str>::from(format!("{}{}", v115, v102));
    let mut v117: Rc<str> = Rc::<str>::from(format!("{}{}", v116, v111));
    let mut v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl binary"); } LIT.with(|lit| lit.clone()) };
    let mut v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl binary_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl binary_gate () = (\"eoie.exe"); } LIT.with(|lit| lit.clone()) };
    let mut v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl binary_gate () = (\"eoie.exe\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0000000000000000000000000000000000000000000000000000000000000000"); } LIT.with(|lit| lit.clone()) };
    let mut v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0000000000000000000000000000000000000000000000000000000000000000\")\n"); } LIT.with(|lit| lit.clone()) };
    let mut v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl binary_gate () = (\"eoie.exe\", \"0000000000000000000000000000000000000000000000000000000000000000\")\n"); } LIT.with(|lit| lit.clone()) };
    let mut v125: Rc<str> = Rc::<str>::from(format!("{}{}", v117, v124));
    std::fs::write(&*v113,&*v125).unwrap();
    let mut v126: Rc<str> = std::rc::Rc::<str>::from(verify_cold_proof(std::path::Path::new(&*v0)).unwrap_err());
    let mut v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gate rejected name=binary"); } LIT.with(|lit| lit.clone()) };
    let mut v128: bool = v126.contains(&*v127);
    if v128 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v129: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v130: Rc<str> = Rc::<str>::from(format!("{}{}", v129, v8));
    std::fs::write(&*v130,&*v10).unwrap();
    let mut v131: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v130)).unwrap());
    let mut v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census"); } LIT.with(|lit| lit.clone()) };
    let mut v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census_gate () = (\"state/authority_census.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census_gate () = (\"state/authority_census.spi\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v136: Rc<str> = Rc::<str>::from(format!("{}{}", v131, v20));
    let mut v137: Rc<str> = Rc::<str>::from(format!("{}{}", v135, v136));
    let mut v138: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v139: Rc<str> = Rc::<str>::from(format!("{}{}", v138, v24));
    std::fs::write(&*v139,&*v10).unwrap();
    let mut v140: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v139)).unwrap());
    let mut v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl migration"); } LIT.with(|lit| lit.clone()) };
    let mut v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl migration_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v143: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl migration_gate () = (\"state/migration.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl migration_gate () = (\"state/migration.spi\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v145: Rc<str> = Rc::<str>::from(format!("{}{}", v140, v20));
    let mut v146: Rc<str> = Rc::<str>::from(format!("{}{}", v144, v145));
    let mut v147: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v148: Rc<str> = Rc::<str>::from(format!("{}{}", v147, v35));
    std::fs::write(&*v148,&*v10).unwrap();
    let mut v149: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v148)).unwrap());
    let mut v150: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl workspace"); } LIT.with(|lit| lit.clone()) };
    let mut v151: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl workspace_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl workspace_gate () = (\"src/Cargo.lock"); } LIT.with(|lit| lit.clone()) };
    let mut v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl workspace_gate () = (\"src/Cargo.lock\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v154: Rc<str> = Rc::<str>::from(format!("{}{}", v149, v20));
    let mut v155: Rc<str> = Rc::<str>::from(format!("{}{}", v153, v154));
    let mut v156: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v157: Rc<str> = Rc::<str>::from(format!("{}{}", v156, v46));
    std::fs::write(&*v157,&*v10).unwrap();
    let mut v158: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v157)).unwrap());
    let mut v159: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl state"); } LIT.with(|lit| lit.clone()) };
    let mut v160: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl state_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl state_gate () = (\"state/core.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v162: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl state_gate () = (\"state/core.spi\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v163: Rc<str> = Rc::<str>::from(format!("{}{}", v158, v20));
    let mut v164: Rc<str> = Rc::<str>::from(format!("{}{}", v162, v163));
    let mut v165: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v166: Rc<str> = Rc::<str>::from(format!("{}{}", v165, v57));
    let mut v167: Rc<str> = Rc::<str>::from(format!("{}{}", v59, v137));
    let mut v168: Rc<str> = Rc::<str>::from(format!("{}{}", v167, v146));
    let mut v169: Rc<str> = Rc::<str>::from(format!("{}{}", v168, v155));
    let mut v170: Rc<str> = Rc::<str>::from(format!("{}{}", v169, v164));
    let mut v171: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl binary"); } LIT.with(|lit| lit.clone()) };
    let mut v172: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl binary_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl binary_gate () = (\"eoie.exe"); } LIT.with(|lit| lit.clone()) };
    let mut v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl binary_gate () = (\"eoie.exe\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v175: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v20));
    let mut v176: Rc<str> = Rc::<str>::from(format!("{}{}", v174, v175));
    let mut v177: Rc<str> = Rc::<str>::from(format!("{}{}", v170, v176));
    std::fs::write(&*v166,&*v177).unwrap();
    let mut v178: Rc<str> = std::rc::Rc::<str>::from(verify_cold_proof(std::path::Path::new(&*v0)).unwrap_err());
    let mut v179: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("authority census payload is missing"); } LIT.with(|lit| lit.clone()) };
    let mut v180: bool = v178.contains(&*v179);
    if v180 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    0i32
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        binary_gate_3(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_5(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let mut v2: bool = v1 == 10u64;
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v3.clone()
    } else {
        let mut v4: u64 = eoie_source_topology::source_topology_counts(std::path::Path::new(&*v0)).unwrap()[v1 as usize] as u64;
        let mut v5: u64 = v4.wrapping_add(4294967296u64);
        let mut v6: bool = v1 == 9u64;
        let mut v9: Rc<str> = if v6 {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v7.clone()
        } else {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        };
        let mut v10: Rc<str> = std::rc::Rc::<str>::from(v5.to_string());
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
        let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v9));
        let mut v14: u64 = v1.wrapping_add(1u64);
        let mut v15: Rc<str> = loop_5(v0.clone(), v14);
        let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v15));
        v16.clone()
    }
}
fn loop_6(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let mut v2: bool = v1 == 10u64;
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v3.clone()
    } else {
        let mut v4: bool = v1 == 9u64;
        let mut v7: Rc<str> = if v4 {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v5.clone()
        } else {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        };
        let mut v8: Rc<str> = std::rc::Rc::<str>::from(2147483647u64.to_string());
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
        let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v7));
        let mut v12: u64 = v1.wrapping_add(1u64);
        let mut v13: Rc<str> = loop_6(v0.clone(), v12);
        let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v13));
        v14.clone()
    }
}
fn census_bounds_4(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie.exe"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v3));
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("native binary payload"); } LIT.with(|lit| lit.clone()) };
    std::fs::write(&*v4,&*v5).unwrap();
    let mut v6: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v4)).unwrap());
    let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/authority_census.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("deliberately incomplete later census fixture"); } LIT.with(|lit| lit.clone()) };
    std::fs::write(&*v9,&*v10).unwrap();
    let mut v11: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v9)).unwrap());
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("census"); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census"); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census_gate () = (\"state/authority_census.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census_gate () = (\"state/authority_census.spi\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\")\n"); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v20));
    let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v21));
    let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/migration.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
    std::fs::write(&*v25,&*v10).unwrap();
    let mut v26: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v25)).unwrap());
    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("migration"); } LIT.with(|lit| lit.clone()) };
    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl migration"); } LIT.with(|lit| lit.clone()) };
    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl migration_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl migration_gate () = (\"state/migration.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl migration_gate () = (\"state/migration.spi\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v32: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v20));
    let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
    let mut v34: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/Cargo.lock"); } LIT.with(|lit| lit.clone()) };
    let mut v36: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v35));
    std::fs::write(&*v36,&*v10).unwrap();
    let mut v37: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v36)).unwrap());
    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("workspace"); } LIT.with(|lit| lit.clone()) };
    let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl workspace"); } LIT.with(|lit| lit.clone()) };
    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl workspace_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl workspace_gate () = (\"src/Cargo.lock"); } LIT.with(|lit| lit.clone()) };
    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl workspace_gate () = (\"src/Cargo.lock\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v43: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v20));
    let mut v44: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v43));
    let mut v45: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/core.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v47: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v46));
    std::fs::write(&*v47,&*v10).unwrap();
    let mut v48: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v47)).unwrap());
    let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state"); } LIT.with(|lit| lit.clone()) };
    let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl state"); } LIT.with(|lit| lit.clone()) };
    let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl state_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl state_gate () = (\"state/core.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl state_gate () = (\"state/core.spi\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v54: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v20));
    let mut v55: Rc<str> = Rc::<str>::from(format!("{}{}", v53, v54));
    let mut v56: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/cold_proof.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v58: Rc<str> = Rc::<str>::from(format!("{}{}", v56, v57));
    let mut v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl current_cold_schema () : cold_receipt_schema = ColdReceiptV4\ninl current_cold_proof_status () : cold_receipt_status = ColdReceiptClosed\ninl current_cold_replay_flags () : u32 = 15u32\n"); } LIT.with(|lit| lit.clone()) };
    let mut v60: Rc<str> = Rc::<str>::from(format!("{}{}", v59, v22));
    let mut v61: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v33));
    let mut v62: Rc<str> = Rc::<str>::from(format!("{}{}", v61, v44));
    let mut v63: Rc<str> = Rc::<str>::from(format!("{}{}", v62, v55));
    let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binary"); } LIT.with(|lit| lit.clone()) };
    let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl binary"); } LIT.with(|lit| lit.clone()) };
    let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl binary_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl binary_gate () = (\"eoie.exe"); } LIT.with(|lit| lit.clone()) };
    let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl binary_gate () = (\"eoie.exe\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v69: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v20));
    let mut v70: Rc<str> = Rc::<str>::from(format!("{}{}", v68, v69));
    let mut v71: Rc<str> = Rc::<str>::from(format!("{}{}", v63, v70));
    std::fs::write(&*v58,&*v71).unwrap();
    let mut v72: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v73: Rc<str> = Rc::<str>::from(format!("{}{}", v72, v8));
    let mut v74: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v73)).unwrap());
    let mut v75: u64 = 0u64;
    let mut v76: Rc<str> = loop_5(v0.clone(), v75);
    let mut v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl current () = AuthorityCensus ("); } LIT.with(|lit| lit.clone()) };
    let mut v78: Rc<str> = Rc::<str>::from(format!("{}{}", v77, v76));
    let mut v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")\n"); } LIT.with(|lit| lit.clone()) };
    let mut v80: Rc<str> = Rc::<str>::from(format!("{}{}", v78, v79));
    std::fs::write(&*v73,&*v80).unwrap();
    let mut v81: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v82: Rc<str> = Rc::<str>::from(format!("{}{}", v81, v57));
    let mut v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census"); } LIT.with(|lit| lit.clone()) };
    let mut v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census_gate () = (\"state/authority_census.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census_gate () = (\"state/authority_census.spi\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v87: Rc<str> = Rc::<str>::from(format!("{}{}", v74, v20));
    let mut v88: Rc<str> = Rc::<str>::from(format!("{}{}", v86, v87));
    let mut v89: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v73)).unwrap());
    let mut v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census"); } LIT.with(|lit| lit.clone()) };
    let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census_gate () = (\"state/authority_census.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census_gate () = (\"state/authority_census.spi\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v94: Rc<str> = Rc::<str>::from(format!("{}{}", v89, v20));
    let mut v95: Rc<str> = Rc::<str>::from(format!("{}{}", v93, v94));
    let mut v96: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v82).unwrap());
    let mut v97: Rc<str> = Rc::<str>::from(v96.replace(&*v88, &*v95));
    std::fs::write(&*v82,&*v97).unwrap();
    let mut v98: Rc<str> = std::rc::Rc::<str>::from(verify_cold_proof(std::path::Path::new(&*v0)).unwrap_err());
    let mut v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("census value exceeds signed comparison range"); } LIT.with(|lit| lit.clone()) };
    let mut v100: bool = v98.contains(&*v99);
    if v100 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v101: u64 = 0u64;
    let mut v102: Rc<str> = loop_6(v0.clone(), v101);
    let mut v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("AuthorityCensus ("); } LIT.with(|lit| lit.clone()) };
    let mut v104: Rc<str> = Rc::<str>::from(format!("{}{}", v103, v102));
    let mut v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let mut v106: Rc<str> = Rc::<str>::from(format!("{}{}", v104, v105));
    let mut v107: bool = cold_census_values(&*v106).unwrap() == [i32::MAX as usize;10];
    if v107 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    0i32
}
fn closure4() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        census_bounds_4(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn renewal_7(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie.exe"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v3));
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("native binary payload"); } LIT.with(|lit| lit.clone()) };
    std::fs::write(&*v4,&*v5).unwrap();
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/authority_census.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("deliberately incomplete later census fixture"); } LIT.with(|lit| lit.clone()) };
    std::fs::write(&*v8,&*v9).unwrap();
    let mut v10: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v8)).unwrap());
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("census"); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census"); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census_gate () = (\"state/authority_census.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census_gate () = (\"state/authority_census.spi\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\")\n"); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v19));
    let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v20));
    let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/migration.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v24: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v23));
    std::fs::write(&*v24,&*v9).unwrap();
    let mut v25: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v24)).unwrap());
    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("migration"); } LIT.with(|lit| lit.clone()) };
    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl migration"); } LIT.with(|lit| lit.clone()) };
    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl migration_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl migration_gate () = (\"state/migration.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl migration_gate () = (\"state/migration.spi\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v19));
    let mut v32: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v31));
    let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/Cargo.lock"); } LIT.with(|lit| lit.clone()) };
    let mut v35: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v34));
    std::fs::write(&*v35,&*v9).unwrap();
    let mut v36: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v35)).unwrap());
    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("workspace"); } LIT.with(|lit| lit.clone()) };
    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl workspace"); } LIT.with(|lit| lit.clone()) };
    let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl workspace_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl workspace_gate () = (\"src/Cargo.lock"); } LIT.with(|lit| lit.clone()) };
    let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl workspace_gate () = (\"src/Cargo.lock\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v42: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v19));
    let mut v43: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v42));
    let mut v44: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/core.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v46: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v45));
    std::fs::write(&*v46,&*v9).unwrap();
    let mut v47: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v46)).unwrap());
    let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state"); } LIT.with(|lit| lit.clone()) };
    let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl state"); } LIT.with(|lit| lit.clone()) };
    let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl state_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl state_gate () = (\"state/core.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl state_gate () = (\"state/core.spi\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v53: Rc<str> = Rc::<str>::from(format!("{}{}", v47, v19));
    let mut v54: Rc<str> = Rc::<str>::from(format!("{}{}", v52, v53));
    let mut v55: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/cold_proof.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v57: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v56));
    let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl current_cold_schema () : cold_receipt_schema = ColdReceiptV4\ninl current_cold_proof_status () : cold_receipt_status = ColdReceiptClosed\ninl current_cold_replay_flags () : u32 = 15u32\n"); } LIT.with(|lit| lit.clone()) };
    let mut v59: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v21));
    let mut v60: Rc<str> = Rc::<str>::from(format!("{}{}", v59, v32));
    let mut v61: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v43));
    let mut v62: Rc<str> = Rc::<str>::from(format!("{}{}", v61, v54));
    let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binary"); } LIT.with(|lit| lit.clone()) };
    let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl binary"); } LIT.with(|lit| lit.clone()) };
    let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl binary_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie"); } LIT.with(|lit| lit.clone()) };
    let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl binary_gate () = (\"eoie"); } LIT.with(|lit| lit.clone()) };
    let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl binary_gate () = (\"eoie\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0000000000000000000000000000000000000000000000000000000000000000"); } LIT.with(|lit| lit.clone()) };
    let mut v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0000000000000000000000000000000000000000000000000000000000000000\")\n"); } LIT.with(|lit| lit.clone()) };
    let mut v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl binary_gate () = (\"eoie\", \"0000000000000000000000000000000000000000000000000000000000000000\")\n"); } LIT.with(|lit| lit.clone()) };
    let mut v72: Rc<str> = Rc::<str>::from(format!("{}{}", v62, v71));
    std::fs::write(&*v57,&*v72).unwrap();
    let mut v73: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v74: Rc<str> = Rc::<str>::from(format!("{}{}", v73, v34));
    let mut v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("renewed lock payload"); } LIT.with(|lit| lit.clone()) };
    std::fs::write(&*v74,&*v75).unwrap();
    let mut v76: Rc<str> = std::rc::Rc::<str>::from(renew_cold_proof_text(std::path::Path::new(&*v0),&std::fs::read_to_string(std::path::Path::new(&*v0).join("state/cold_proof.spi")).unwrap()).unwrap());
    let mut v77: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v4)).unwrap());
    let mut v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl binary"); } LIT.with(|lit| lit.clone()) };
    let mut v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl binary_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl binary_gate () = (\"eoie.exe"); } LIT.with(|lit| lit.clone()) };
    let mut v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl binary_gate () = (\"eoie.exe\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v82: Rc<str> = Rc::<str>::from(format!("{}{}", v77, v19));
    let mut v83: Rc<str> = Rc::<str>::from(format!("{}{}", v81, v82));
    let mut v84: bool = v76.contains(&*v83);
    if v84 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v85: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v74)).unwrap());
    let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl workspace"); } LIT.with(|lit| lit.clone()) };
    let mut v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl workspace_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl workspace_gate () = (\"src/Cargo.lock"); } LIT.with(|lit| lit.clone()) };
    let mut v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl workspace_gate () = (\"src/Cargo.lock\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v90: Rc<str> = Rc::<str>::from(format!("{}{}", v85, v19));
    let mut v91: Rc<str> = Rc::<str>::from(format!("{}{}", v89, v90));
    let mut v92: bool = v76.contains(&*v91);
    if v92 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v93: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v94: Rc<str> = Rc::<str>::from(format!("{}{}", v93, v56));
    let mut v95: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v94).unwrap());
    let mut v96: Rc<str> = std::rc::Rc::<str>::from(refresh_cold_proof_census_gate(std::path::Path::new(&*v0)).unwrap_err());
    let mut v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ColdProofV4 renewal rolled back"); } LIT.with(|lit| lit.clone()) };
    let mut v98: bool = v96.contains(&*v97);
    if v98 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v99: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v94).unwrap());
    let mut v100: bool = v99.clone() == v95.clone();
    if v100 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v101: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v102: Rc<str> = Rc::<str>::from(format!("{}{}", v101, v66));
    let mut v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unix binary"); } LIT.with(|lit| lit.clone()) };
    std::fs::write(&*v102,&*v103).unwrap();
    let mut v104: Rc<str> = std::rc::Rc::<str>::from(refresh_cold_proof_census_gate(std::path::Path::new(&*v0)).unwrap_err());
    let mut v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exactly one root binary"); } LIT.with(|lit| lit.clone()) };
    let mut v106: bool = v104.contains(&*v105);
    if v106 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v107: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v94).unwrap());
    let mut v108: bool = v107.clone() == v95.clone();
    if v108 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    0i32
}
fn closure5() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        renewal_7(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn cold_proof_equal_binding_8(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 == v1;
    if v2 {
        1i32
    } else {
        0i32
    }
}
fn cold_rebuild_expected_shards_binding_9(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 1i32;
    if v2 {
        -1i32
    } else {
        let mut v3: bool = v1 == 1i32;
        if v3 {
            v0
        } else {
            let mut v4: bool = v1 == 2i32;
            if v4 {
                let mut v5: i32 = v0.wrapping_sub(1i32);
                let mut v6: i32 = v5.wrapping_div(2i32);
                let mut v7: i32 = v6.wrapping_add(1i32);
                v7
            } else {
                let mut v8: bool = v1 == 4i32;
                if v8 {
                    let mut v9: i32 = v0.wrapping_sub(1i32);
                    let mut v10: i32 = v9.wrapping_div(4i32);
                    let mut v11: i32 = v10.wrapping_add(1i32);
                    v11
                } else {
                    -1i32
                }
            }
        }
    }
}
fn loop_11(mut v0: Rc<str>, mut v1: u64, mut v2: u64) -> bool {
    loop {
        let mut v3: bool = v2 == v1;
        if v3 {
            return true;
        } else {
            let mut v4: u64 = v0.as_bytes()[v2 as usize] as u64;
            let mut v5: bool = v4 < 48u64;
            let mut v7: bool = if v5 {
                false
            } else {
                let mut v6: bool = v4 <= 57u64;
                v6
            };
            let mut v15: bool = if v7 {
                true
            } else {
                let mut v8: bool = v4 < 65u64;
                let mut v10: bool = if v8 {
                    false
                } else {
                    let mut v9: bool = v4 <= 70u64;
                    v9
                };
                if v10 {
                    true
                } else {
                    let mut v11: bool = v4 < 97u64;
                    if v11 {
                        false
                    } else {
                        let mut v12: bool = v4 <= 102u64;
                        v12
                    }
                }
            };
            if v15 {
                let mut v16: u64 = v2.wrapping_add(1u64);
                (v0, v1, v2) = (v0.clone(), v1, v16);
                continue;
            } else {
                return false;
            }
        }
    }
}
fn check_10(mut v0: Rc<str>) -> bool {
    let mut v1: u64 = (v0.clone().len() as u64);
    let mut v2: bool = v1 < 64u64;
    let mut v4: bool = if v2 {
        false
    } else {
        let mut v3: bool = v1 <= 64u64;
        v3
    };
    if v4 {
        let mut v5: u64 = 0u64;
        loop_11(v0.clone(), v1, v5)
    } else {
        false
    }
}
fn closure6() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        check_10(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure7() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        cold_proof_equal_binding_8(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn cold_proof_binary_binding_12(mut v0: i32) -> i32 {
    let mut v1: bool = v0 == 1i32;
    if v1 {
        1i32
    } else {
        let mut v2: bool = v0 == 2i32;
        if v2 {
            2i32
        } else {
            0i32
        }
    }
}
fn closure8() -> Rc<dyn Fn(i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32) -> i32> = Rc::new(move |mut v0: i32| -> i32 {
        cold_proof_binary_binding_12(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure9() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        cold_rebuild_expected_shards_binding_9(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_test_family_renewal(v0: &str) -> i32 {
    closure0()(Rc::<str>::from(v0))
}
pub fn eoie_test_differential_renewal(v0: &str) -> i32 {
    closure1()(Rc::<str>::from(v0))
}
pub fn eoie_test_cold_selection(v0: &str) -> i32 {
    closure2()(Rc::<str>::from(v0))
}
pub fn eoie_test_cold_binary_gate(v0: &str) -> i32 {
    closure3()(Rc::<str>::from(v0))
}
pub fn eoie_test_cold_census_bounds(v0: &str) -> i32 {
    closure4()(Rc::<str>::from(v0))
}
pub fn eoie_test_cold_renewal(v0: &str) -> i32 {
    closure5()(Rc::<str>::from(v0))
}
pub fn cold_rebuild_hash(v0: &str) -> bool {
    closure6()(Rc::<str>::from(v0))
}
pub fn eoie_cold_proof_equal_binding(v0: i32, v1: i32) -> i32 {
    closure7()(v0, v1)
}
pub fn eoie_cold_proof_binary_binding(v0: i32) -> i32 {
    closure8()(v0)
}
pub fn eoie_cold_rebuild_expected_shards(v0: i32, v1: i32) -> i32 {
    closure9()(v0, v1)
}
