#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_patterns, unreachable_code, while_true)]
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

fn method0(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/legacy_catalog.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v2, &*v3, &*v3].concat());
    let mut v5: Rc<str> = std::rc::Rc::<str>::from((0..36u64).map(|i|{let q=char::from(34u8).to_string();let n=char::from(10u8).to_string();let f=i.to_string();["CapabilityFamily (",&q,"f",&f,&q,", Absorbed, x)",&n,"LegacyDecision (",&q,"f",&f,&q,", PortNow, x)",&n].concat()}).collect::<String>());
    std::fs::write(&*v4,&*v5).unwrap();
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/migration.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v6, &*v3, &*v3].concat());
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("migration fixture"); } LIT.with(|lit| lit.clone()) };
    std::fs::write(&*v7,&*v8).unwrap();
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/runtime_drift.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v9, &*v3, &*v3].concat());
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("runtime drift fixture"); } LIT.with(|lit| lit.clone()) };
    std::fs::write(&*v10,&*v11).unwrap();
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/adapter_inventory.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v12, &*v3, &*v3].concat());
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("adapter fixture"); } LIT.with(|lit| lit.clone()) };
    std::fs::write(&*v13,&*v14).unwrap();
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("catalog"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v16, &*v17, &*v2, &*v18].concat());
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0000000000000000000000000000000000000000000000000000000000000000"); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\")\n"); } LIT.with(|lit| lit.clone()) };
    let mut v22: Rc<str> = std::rc::Rc::<str>::from([&*v20, &*v21, &*v3, &*v3, &*v3].concat());
    let mut v23: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v22, &*v3, &*v3, &*v3].concat());
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("migration"); } LIT.with(|lit| lit.clone()) };
    let mut v25: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v24, &*v17, &*v6, &*v18].concat());
    let mut v26: Rc<str> = std::rc::Rc::<str>::from([&*v20, &*v21, &*v3, &*v3, &*v3].concat());
    let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v25, &*v26, &*v3, &*v3, &*v3].concat());
    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("runtime"); } LIT.with(|lit| lit.clone()) };
    let mut v29: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v28, &*v17, &*v9, &*v18].concat());
    let mut v30: Rc<str> = std::rc::Rc::<str>::from([&*v20, &*v21, &*v3, &*v3, &*v3].concat());
    let mut v31: Rc<str> = std::rc::Rc::<str>::from([&*v29, &*v30, &*v3, &*v3, &*v3].concat());
    let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("adapter"); } LIT.with(|lit| lit.clone()) };
    let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v32, &*v17, &*v12, &*v18].concat());
    let mut v34: Rc<str> = std::rc::Rc::<str>::from([&*v20, &*v21, &*v3, &*v3, &*v3].concat());
    let mut v35: Rc<str> = std::rc::Rc::<str>::from([&*v33, &*v34, &*v3, &*v3, &*v3].concat());
    let mut v36: Rc<str> = std::rc::Rc::<str>::from([&*v23, &*v27, &*v31, &*v35, &*v3].concat());
    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl current_family_contract_schema () : family_contract_schema = FamilyContractsV1\ninl current_family_contract_status () : family_contract_status = FamilyContractsClosed\ninl current_family_contract_semantic_flags () : u32 = 31u32\ninl current_family_count () : u32 = 36u32\ninl current_family_decision_count () : u32 = 36u32\ninl current_unsettled_family_count () : u32 = 0u32\n"); } LIT.with(|lit| lit.clone()) };
    let mut v38: Rc<str> = std::rc::Rc::<str>::from([&*v37, &*v36, &*v3, &*v3, &*v3].concat());
    let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/family_contracts.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v40: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v39, &*v3, &*v3].concat());
    std::fs::write(&*v40,&*v38).unwrap();
    let mut v41: bool = renew_family_contracts(std::path::Path::new(&*v0)).unwrap().is_some();
    if v41 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v42: bool = verify_family_contracts(std::path::Path::new(&*v0)).is_ok();
    if v42 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v43: bool = renew_family_contracts(std::path::Path::new(&*v0)).unwrap().is_none();
    if v43 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v44: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v2, &*v3, &*v3].concat());
    let mut v45: Rc<str> = std::rc::Rc::<str>::from((0..35u64).map(|i|{let q=char::from(34u8).to_string();let n=char::from(10u8).to_string();let f=i.to_string();["CapabilityFamily (",&q,"f",&f,&q,", Absorbed, x)",&n,"LegacyDecision (",&q,"f",&f,&q,", PortNow, x)",&n].concat()}).collect::<String>());
    std::fs::write(&*v44,&*v45).unwrap();
    std::fs::write(&*v40,&*v38).unwrap();
    let mut v46: Rc<str> = std::rc::Rc::<str>::from(renew_family_contracts(std::path::Path::new(&*v0)).unwrap_err());
    let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("family contract catalog mismatch"); } LIT.with(|lit| lit.clone()) };
    let mut v48: bool = v46.contains(&*v47);
    if v48 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v49: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v40).unwrap());
    let mut v50: bool = v49 == v38 ;
    if v50 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    0i32
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        method0(v0.clone())
    }); }
    CLOSURE.with(|closure| closure.clone())
}
fn method1(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie.exe"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v2, &*v3, &*v3].concat());
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("not an executable differential route"); } LIT.with(|lit| lit.clone()) };
    std::fs::write(&*v4,&*v5).unwrap();
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/differential_catalog.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v6, &*v3, &*v3].concat());
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl current_differential_catalog_schema () : differential_catalog_schema = DifferentialCatalogV1\n"); } LIT.with(|lit| lit.clone()) };
    std::fs::write(&*v7,&*v8).unwrap();
    let mut v9: Rc<str> = std::rc::Rc::<str>::from(renew_differential_catalog(std::path::Path::new(&*v0)).unwrap_err());
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("differential route"); } LIT.with(|lit| lit.clone()) };
    let mut v11: bool = v9.contains(&*v10);
    if v11 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v12: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v7).unwrap());
    let mut v13: bool = v12 == v8 ;
    if v13 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v14: Rc<str> = std::rc::Rc::<str>::from(refresh_cold_proof_census_gate(std::path::Path::new(&*v0)).unwrap_err());
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ColdProofV4 renewal rolled back"); } LIT.with(|lit| lit.clone()) };
    let mut v16: bool = v14.contains(&*v15);
    if v16 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v17: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v7).unwrap());
    let mut v18: bool = v17 == v8 ;
    if v18 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    0i32
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        method1(v0.clone())
    }); }
    CLOSURE.with(|closure| closure.clone())
}
fn method2(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(cold_public_binary(std::path::Path::new(&*v0)).unwrap_or(""));
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v1 == v2 ;
    if v3 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie.exe"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v4, &*v5, &*v2, &*v2].concat());
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("windows binary"); } LIT.with(|lit| lit.clone()) };
    std::fs::write(&*v6,&*v7).unwrap();
    let mut v8: Rc<str> = std::rc::Rc::<str>::from(cold_public_binary(std::path::Path::new(&*v0)).unwrap_or(""));
    let mut v9: bool = v8 == v5 ;
    if v9 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v4, &*v10, &*v2, &*v2].concat());
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unix binary"); } LIT.with(|lit| lit.clone()) };
    std::fs::write(&*v11,&*v12).unwrap();
    let mut v13: Rc<str> = std::rc::Rc::<str>::from(cold_public_binary(std::path::Path::new(&*v0)).unwrap_or(""));
    let mut v14: bool = v13 == v2 ;
    if v14 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v15: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v4, &*v5, &*v2, &*v2].concat());
    std::fs::remove_file(&*v15).unwrap();
    let mut v16: Rc<str> = std::rc::Rc::<str>::from(cold_public_binary(std::path::Path::new(&*v0)).unwrap_or(""));
    let mut v17: bool = v16 == v10 ;
    if v17 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    0i32
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        method2(v0.clone())
    }); }
    CLOSURE.with(|closure| closure.clone())
}
fn method3(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie.exe"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v2, &*v3, &*v3].concat());
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("native binary payload"); } LIT.with(|lit| lit.clone()) };
    std::fs::write(&*v4,&*v5).unwrap();
    let mut v6: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v4)).unwrap());
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/authority_census.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v7, &*v3, &*v3].concat());
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("deliberately incomplete later census fixture"); } LIT.with(|lit| lit.clone()) };
    std::fs::write(&*v8,&*v9).unwrap();
    let mut v10: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v8)).unwrap());
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("census"); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v12, &*v13, &*v7, &*v14].concat());
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\")\n"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v16, &*v3, &*v3, &*v3].concat());
    let mut v18: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v17, &*v3, &*v3, &*v3].concat());
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/migration.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v19, &*v3, &*v3].concat());
    std::fs::write(&*v20,&*v9).unwrap();
    let mut v21: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v20)).unwrap());
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("migration"); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v22, &*v13, &*v19, &*v14].concat());
    let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v16, &*v3, &*v3, &*v3].concat());
    let mut v25: Rc<str> = std::rc::Rc::<str>::from([&*v23, &*v24, &*v3, &*v3, &*v3].concat());
    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/Cargo.lock"); } LIT.with(|lit| lit.clone()) };
    let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v26, &*v3, &*v3].concat());
    std::fs::write(&*v27,&*v9).unwrap();
    let mut v28: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v27)).unwrap());
    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("workspace"); } LIT.with(|lit| lit.clone()) };
    let mut v30: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v29, &*v13, &*v26, &*v14].concat());
    let mut v31: Rc<str> = std::rc::Rc::<str>::from([&*v28, &*v16, &*v3, &*v3, &*v3].concat());
    let mut v32: Rc<str> = std::rc::Rc::<str>::from([&*v30, &*v31, &*v3, &*v3, &*v3].concat());
    let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/core.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v34: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v33, &*v3, &*v3].concat());
    std::fs::write(&*v34,&*v9).unwrap();
    let mut v35: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v34)).unwrap());
    let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state"); } LIT.with(|lit| lit.clone()) };
    let mut v37: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v36, &*v13, &*v33, &*v14].concat());
    let mut v38: Rc<str> = std::rc::Rc::<str>::from([&*v35, &*v16, &*v3, &*v3, &*v3].concat());
    let mut v39: Rc<str> = std::rc::Rc::<str>::from([&*v37, &*v38, &*v3, &*v3, &*v3].concat());
    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/cold_proof.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v41: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v40, &*v3, &*v3].concat());
    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl current_cold_schema () : cold_receipt_schema = ColdReceiptV4\ninl current_cold_proof_status () : cold_receipt_status = ColdReceiptClosed\ninl current_cold_replay_flags () : u32 = 15u32\n"); } LIT.with(|lit| lit.clone()) };
    let mut v43: Rc<str> = std::rc::Rc::<str>::from([&*v42, &*v18, &*v25, &*v32, &*v39].concat());
    let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binary"); } LIT.with(|lit| lit.clone()) };
    let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie"); } LIT.with(|lit| lit.clone()) };
    let mut v46: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v44, &*v13, &*v45, &*v14].concat());
    let mut v47: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v16, &*v3, &*v3, &*v3].concat());
    let mut v48: Rc<str> = std::rc::Rc::<str>::from([&*v46, &*v47, &*v3, &*v3, &*v3].concat());
    let mut v49: Rc<str> = std::rc::Rc::<str>::from([&*v43, &*v48, &*v3, &*v3, &*v3].concat());
    std::fs::write(&*v41,&*v49).unwrap();
    let mut v50: Rc<str> = std::rc::Rc::<str>::from(verify_cold_proof(std::path::Path::new(&*v0)).unwrap_err());
    let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected=eoie.exe observed=eoie"); } LIT.with(|lit| lit.clone()) };
    let mut v52: bool = v50.contains(&*v51);
    if v52 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v53: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v7, &*v3, &*v3].concat());
    std::fs::write(&*v53,&*v9).unwrap();
    let mut v54: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v53)).unwrap());
    let mut v55: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v12, &*v13, &*v7, &*v14].concat());
    let mut v56: Rc<str> = std::rc::Rc::<str>::from([&*v54, &*v16, &*v3, &*v3, &*v3].concat());
    let mut v57: Rc<str> = std::rc::Rc::<str>::from([&*v55, &*v56, &*v3, &*v3, &*v3].concat());
    let mut v58: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v19, &*v3, &*v3].concat());
    std::fs::write(&*v58,&*v9).unwrap();
    let mut v59: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v58)).unwrap());
    let mut v60: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v22, &*v13, &*v19, &*v14].concat());
    let mut v61: Rc<str> = std::rc::Rc::<str>::from([&*v59, &*v16, &*v3, &*v3, &*v3].concat());
    let mut v62: Rc<str> = std::rc::Rc::<str>::from([&*v60, &*v61, &*v3, &*v3, &*v3].concat());
    let mut v63: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v26, &*v3, &*v3].concat());
    std::fs::write(&*v63,&*v9).unwrap();
    let mut v64: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v63)).unwrap());
    let mut v65: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v29, &*v13, &*v26, &*v14].concat());
    let mut v66: Rc<str> = std::rc::Rc::<str>::from([&*v64, &*v16, &*v3, &*v3, &*v3].concat());
    let mut v67: Rc<str> = std::rc::Rc::<str>::from([&*v65, &*v66, &*v3, &*v3, &*v3].concat());
    let mut v68: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v33, &*v3, &*v3].concat());
    std::fs::write(&*v68,&*v9).unwrap();
    let mut v69: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v68)).unwrap());
    let mut v70: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v36, &*v13, &*v33, &*v14].concat());
    let mut v71: Rc<str> = std::rc::Rc::<str>::from([&*v69, &*v16, &*v3, &*v3, &*v3].concat());
    let mut v72: Rc<str> = std::rc::Rc::<str>::from([&*v70, &*v71, &*v3, &*v3, &*v3].concat());
    let mut v73: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v40, &*v3, &*v3].concat());
    let mut v74: Rc<str> = std::rc::Rc::<str>::from([&*v42, &*v57, &*v62, &*v67, &*v72].concat());
    let mut v75: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v44, &*v13, &*v2, &*v14].concat());
    let mut v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0000000000000000000000000000000000000000000000000000000000000000"); } LIT.with(|lit| lit.clone()) };
    let mut v77: Rc<str> = std::rc::Rc::<str>::from([&*v76, &*v16, &*v3, &*v3, &*v3].concat());
    let mut v78: Rc<str> = std::rc::Rc::<str>::from([&*v75, &*v77, &*v3, &*v3, &*v3].concat());
    let mut v79: Rc<str> = std::rc::Rc::<str>::from([&*v74, &*v78, &*v3, &*v3, &*v3].concat());
    std::fs::write(&*v73,&*v79).unwrap();
    let mut v80: Rc<str> = std::rc::Rc::<str>::from(verify_cold_proof(std::path::Path::new(&*v0)).unwrap_err());
    let mut v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gate rejected name=binary"); } LIT.with(|lit| lit.clone()) };
    let mut v82: bool = v80.contains(&*v81);
    if v82 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v83: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v7, &*v3, &*v3].concat());
    std::fs::write(&*v83,&*v9).unwrap();
    let mut v84: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v83)).unwrap());
    let mut v85: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v12, &*v13, &*v7, &*v14].concat());
    let mut v86: Rc<str> = std::rc::Rc::<str>::from([&*v84, &*v16, &*v3, &*v3, &*v3].concat());
    let mut v87: Rc<str> = std::rc::Rc::<str>::from([&*v85, &*v86, &*v3, &*v3, &*v3].concat());
    let mut v88: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v19, &*v3, &*v3].concat());
    std::fs::write(&*v88,&*v9).unwrap();
    let mut v89: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v88)).unwrap());
    let mut v90: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v22, &*v13, &*v19, &*v14].concat());
    let mut v91: Rc<str> = std::rc::Rc::<str>::from([&*v89, &*v16, &*v3, &*v3, &*v3].concat());
    let mut v92: Rc<str> = std::rc::Rc::<str>::from([&*v90, &*v91, &*v3, &*v3, &*v3].concat());
    let mut v93: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v26, &*v3, &*v3].concat());
    std::fs::write(&*v93,&*v9).unwrap();
    let mut v94: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v93)).unwrap());
    let mut v95: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v29, &*v13, &*v26, &*v14].concat());
    let mut v96: Rc<str> = std::rc::Rc::<str>::from([&*v94, &*v16, &*v3, &*v3, &*v3].concat());
    let mut v97: Rc<str> = std::rc::Rc::<str>::from([&*v95, &*v96, &*v3, &*v3, &*v3].concat());
    let mut v98: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v33, &*v3, &*v3].concat());
    std::fs::write(&*v98,&*v9).unwrap();
    let mut v99: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v98)).unwrap());
    let mut v100: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v36, &*v13, &*v33, &*v14].concat());
    let mut v101: Rc<str> = std::rc::Rc::<str>::from([&*v99, &*v16, &*v3, &*v3, &*v3].concat());
    let mut v102: Rc<str> = std::rc::Rc::<str>::from([&*v100, &*v101, &*v3, &*v3, &*v3].concat());
    let mut v103: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v40, &*v3, &*v3].concat());
    let mut v104: Rc<str> = std::rc::Rc::<str>::from([&*v42, &*v87, &*v92, &*v97, &*v102].concat());
    let mut v105: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v44, &*v13, &*v2, &*v14].concat());
    let mut v106: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v16, &*v3, &*v3, &*v3].concat());
    let mut v107: Rc<str> = std::rc::Rc::<str>::from([&*v105, &*v106, &*v3, &*v3, &*v3].concat());
    let mut v108: Rc<str> = std::rc::Rc::<str>::from([&*v104, &*v107, &*v3, &*v3, &*v3].concat());
    std::fs::write(&*v103,&*v108).unwrap();
    let mut v109: Rc<str> = std::rc::Rc::<str>::from(verify_cold_proof(std::path::Path::new(&*v0)).unwrap_err());
    let mut v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("authority census payload is missing"); } LIT.with(|lit| lit.clone()) };
    let mut v111: bool = v109.contains(&*v110);
    if v111 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    0i32
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        method3(v0.clone())
    }); }
    CLOSURE.with(|closure| closure.clone())
}
fn method5(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let mut v2: bool = v1 == 10u64;
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v3.clone()
    } else {
        let mut v4: u64 = eoie_source_topology::source_topology_counts(std::path::Path::new(&*v0)).unwrap()[v1 as usize] as u64;
        let mut v5: u64 = v4 + 4294967296u64;
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
        let mut v12: u64 = v1 + 1u64;
        let mut v13: Rc<str> = method5(v0.clone(), v12);
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v15: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v11, &*v9, &*v13, &*v14].concat());
        v15.clone()
    }
}
fn method6(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
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
        let mut v10: u64 = v1 + 1u64;
        let mut v11: Rc<str> = method6(v0.clone(), v10);
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v9, &*v7, &*v11, &*v12].concat());
        v13.clone()
    }
}
fn method4(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie.exe"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v2, &*v3, &*v3].concat());
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("native binary payload"); } LIT.with(|lit| lit.clone()) };
    std::fs::write(&*v4,&*v5).unwrap();
    let mut v6: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v4)).unwrap());
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/authority_census.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v7, &*v3, &*v3].concat());
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("deliberately incomplete later census fixture"); } LIT.with(|lit| lit.clone()) };
    std::fs::write(&*v8,&*v9).unwrap();
    let mut v10: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v8)).unwrap());
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("census"); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v12, &*v13, &*v7, &*v14].concat());
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\")\n"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v16, &*v3, &*v3, &*v3].concat());
    let mut v18: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v17, &*v3, &*v3, &*v3].concat());
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/migration.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v19, &*v3, &*v3].concat());
    std::fs::write(&*v20,&*v9).unwrap();
    let mut v21: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v20)).unwrap());
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("migration"); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v22, &*v13, &*v19, &*v14].concat());
    let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v16, &*v3, &*v3, &*v3].concat());
    let mut v25: Rc<str> = std::rc::Rc::<str>::from([&*v23, &*v24, &*v3, &*v3, &*v3].concat());
    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/Cargo.lock"); } LIT.with(|lit| lit.clone()) };
    let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v26, &*v3, &*v3].concat());
    std::fs::write(&*v27,&*v9).unwrap();
    let mut v28: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v27)).unwrap());
    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("workspace"); } LIT.with(|lit| lit.clone()) };
    let mut v30: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v29, &*v13, &*v26, &*v14].concat());
    let mut v31: Rc<str> = std::rc::Rc::<str>::from([&*v28, &*v16, &*v3, &*v3, &*v3].concat());
    let mut v32: Rc<str> = std::rc::Rc::<str>::from([&*v30, &*v31, &*v3, &*v3, &*v3].concat());
    let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/core.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v34: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v33, &*v3, &*v3].concat());
    std::fs::write(&*v34,&*v9).unwrap();
    let mut v35: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v34)).unwrap());
    let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state"); } LIT.with(|lit| lit.clone()) };
    let mut v37: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v36, &*v13, &*v33, &*v14].concat());
    let mut v38: Rc<str> = std::rc::Rc::<str>::from([&*v35, &*v16, &*v3, &*v3, &*v3].concat());
    let mut v39: Rc<str> = std::rc::Rc::<str>::from([&*v37, &*v38, &*v3, &*v3, &*v3].concat());
    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/cold_proof.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v41: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v40, &*v3, &*v3].concat());
    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl current_cold_schema () : cold_receipt_schema = ColdReceiptV4\ninl current_cold_proof_status () : cold_receipt_status = ColdReceiptClosed\ninl current_cold_replay_flags () : u32 = 15u32\n"); } LIT.with(|lit| lit.clone()) };
    let mut v43: Rc<str> = std::rc::Rc::<str>::from([&*v42, &*v18, &*v25, &*v32, &*v39].concat());
    let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binary"); } LIT.with(|lit| lit.clone()) };
    let mut v45: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v44, &*v13, &*v2, &*v14].concat());
    let mut v46: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v16, &*v3, &*v3, &*v3].concat());
    let mut v47: Rc<str> = std::rc::Rc::<str>::from([&*v45, &*v46, &*v3, &*v3, &*v3].concat());
    let mut v48: Rc<str> = std::rc::Rc::<str>::from([&*v43, &*v47, &*v3, &*v3, &*v3].concat());
    std::fs::write(&*v41,&*v48).unwrap();
    let mut v49: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v7, &*v3, &*v3].concat());
    let mut v50: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v49)).unwrap());
    let mut v51: u64 = 0u64;
    let mut v52: Rc<str> = method5(v0.clone(), v51);
    let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl current () = AuthorityCensus ("); } LIT.with(|lit| lit.clone()) };
    let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")\n"); } LIT.with(|lit| lit.clone()) };
    let mut v55: Rc<str> = std::rc::Rc::<str>::from([&*v53, &*v52, &*v54, &*v3, &*v3].concat());
    std::fs::write(&*v49,&*v55).unwrap();
    let mut v56: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v40, &*v3, &*v3].concat());
    let mut v57: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v56).unwrap());
    let mut v58: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v12, &*v13, &*v7, &*v14].concat());
    let mut v59: Rc<str> = std::rc::Rc::<str>::from([&*v50, &*v16, &*v3, &*v3, &*v3].concat());
    let mut v60: Rc<str> = std::rc::Rc::<str>::from([&*v58, &*v59, &*v3, &*v3, &*v3].concat());
    let mut v61: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v49)).unwrap());
    let mut v62: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v12, &*v13, &*v7, &*v14].concat());
    let mut v63: Rc<str> = std::rc::Rc::<str>::from([&*v61, &*v16, &*v3, &*v3, &*v3].concat());
    let mut v64: Rc<str> = std::rc::Rc::<str>::from([&*v62, &*v63, &*v3, &*v3, &*v3].concat());
    let mut v65: Rc<str> = std::rc::Rc::<str>::from(v57.replace(&*v60, &v64));
    std::fs::write(&*v56,&*v65).unwrap();
    let mut v66: Rc<str> = std::rc::Rc::<str>::from(verify_cold_proof(std::path::Path::new(&*v0)).unwrap_err());
    let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("census value exceeds signed comparison range"); } LIT.with(|lit| lit.clone()) };
    let mut v68: bool = v66.contains(&*v67);
    if v68 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v69: u64 = 0u64;
    let mut v70: Rc<str> = method6(v0.clone(), v69);
    let mut v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("AuthorityCensus ("); } LIT.with(|lit| lit.clone()) };
    let mut v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let mut v73: Rc<str> = std::rc::Rc::<str>::from([&*v71, &*v70, &*v72, &*v3, &*v3].concat());
    let mut v74: bool = cold_census_values(&*v73).unwrap() == [i32::MAX as usize;10];
    if v74 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    0i32
}
fn closure4() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        method4(v0.clone())
    }); }
    CLOSURE.with(|closure| closure.clone())
}
fn method7(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie.exe"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v2, &*v3, &*v3].concat());
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("native binary payload"); } LIT.with(|lit| lit.clone()) };
    std::fs::write(&*v4,&*v5).unwrap();
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/authority_census.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v6, &*v3, &*v3].concat());
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("deliberately incomplete later census fixture"); } LIT.with(|lit| lit.clone()) };
    std::fs::write(&*v7,&*v8).unwrap();
    let mut v9: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v7)).unwrap());
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("census"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_gate () = (\""); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v11, &*v12, &*v6, &*v13].concat());
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\")\n"); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v15, &*v3, &*v3, &*v3].concat());
    let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v16, &*v3, &*v3, &*v3].concat());
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/migration.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v18, &*v3, &*v3].concat());
    std::fs::write(&*v19,&*v8).unwrap();
    let mut v20: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v19)).unwrap());
    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("migration"); } LIT.with(|lit| lit.clone()) };
    let mut v22: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v21, &*v12, &*v18, &*v13].concat());
    let mut v23: Rc<str> = std::rc::Rc::<str>::from([&*v20, &*v15, &*v3, &*v3, &*v3].concat());
    let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v22, &*v23, &*v3, &*v3, &*v3].concat());
    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/Cargo.lock"); } LIT.with(|lit| lit.clone()) };
    let mut v26: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v25, &*v3, &*v3].concat());
    std::fs::write(&*v26,&*v8).unwrap();
    let mut v27: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v26)).unwrap());
    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("workspace"); } LIT.with(|lit| lit.clone()) };
    let mut v29: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v28, &*v12, &*v25, &*v13].concat());
    let mut v30: Rc<str> = std::rc::Rc::<str>::from([&*v27, &*v15, &*v3, &*v3, &*v3].concat());
    let mut v31: Rc<str> = std::rc::Rc::<str>::from([&*v29, &*v30, &*v3, &*v3, &*v3].concat());
    let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/core.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v32, &*v3, &*v3].concat());
    std::fs::write(&*v33,&*v8).unwrap();
    let mut v34: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v33)).unwrap());
    let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state"); } LIT.with(|lit| lit.clone()) };
    let mut v36: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v35, &*v12, &*v32, &*v13].concat());
    let mut v37: Rc<str> = std::rc::Rc::<str>::from([&*v34, &*v15, &*v3, &*v3, &*v3].concat());
    let mut v38: Rc<str> = std::rc::Rc::<str>::from([&*v36, &*v37, &*v3, &*v3, &*v3].concat());
    let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/cold_proof.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v40: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v39, &*v3, &*v3].concat());
    let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl current_cold_schema () : cold_receipt_schema = ColdReceiptV4\ninl current_cold_proof_status () : cold_receipt_status = ColdReceiptClosed\ninl current_cold_replay_flags () : u32 = 15u32\n"); } LIT.with(|lit| lit.clone()) };
    let mut v42: Rc<str> = std::rc::Rc::<str>::from([&*v41, &*v17, &*v24, &*v31, &*v38].concat());
    let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binary"); } LIT.with(|lit| lit.clone()) };
    let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie"); } LIT.with(|lit| lit.clone()) };
    let mut v45: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v43, &*v12, &*v44, &*v13].concat());
    let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0000000000000000000000000000000000000000000000000000000000000000"); } LIT.with(|lit| lit.clone()) };
    let mut v47: Rc<str> = std::rc::Rc::<str>::from([&*v46, &*v15, &*v3, &*v3, &*v3].concat());
    let mut v48: Rc<str> = std::rc::Rc::<str>::from([&*v45, &*v47, &*v3, &*v3, &*v3].concat());
    let mut v49: Rc<str> = std::rc::Rc::<str>::from([&*v42, &*v48, &*v3, &*v3, &*v3].concat());
    std::fs::write(&*v40,&*v49).unwrap();
    let mut v50: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v25, &*v3, &*v3].concat());
    let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("renewed lock payload"); } LIT.with(|lit| lit.clone()) };
    std::fs::write(&*v50,&*v51).unwrap();
    let mut v52: Rc<str> = std::rc::Rc::<str>::from(renew_cold_proof_text(std::path::Path::new(&*v0),&std::fs::read_to_string(std::path::Path::new(&*v0).join("state/cold_proof.spi")).unwrap()).unwrap());
    let mut v53: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v4)).unwrap());
    let mut v54: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v43, &*v12, &*v2, &*v13].concat());
    let mut v55: Rc<str> = std::rc::Rc::<str>::from([&*v53, &*v15, &*v3, &*v3, &*v3].concat());
    let mut v56: Rc<str> = std::rc::Rc::<str>::from([&*v54, &*v55, &*v3, &*v3, &*v3].concat());
    let mut v57: bool = v52.contains(&*v56);
    if v57 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v58: Rc<str> = std::rc::Rc::<str>::from(eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v50)).unwrap());
    let mut v59: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v28, &*v12, &*v25, &*v13].concat());
    let mut v60: Rc<str> = std::rc::Rc::<str>::from([&*v58, &*v15, &*v3, &*v3, &*v3].concat());
    let mut v61: Rc<str> = std::rc::Rc::<str>::from([&*v59, &*v60, &*v3, &*v3, &*v3].concat());
    let mut v62: bool = v52.contains(&*v61);
    if v62 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v63: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v39, &*v3, &*v3].concat());
    let mut v64: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v63).unwrap());
    let mut v65: Rc<str> = std::rc::Rc::<str>::from(refresh_cold_proof_census_gate(std::path::Path::new(&*v0)).unwrap_err());
    let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ColdProofV4 renewal rolled back"); } LIT.with(|lit| lit.clone()) };
    let mut v67: bool = v65.contains(&*v66);
    if v67 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v68: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v63).unwrap());
    let mut v69: bool = v68 == v64 ;
    if v69 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v70: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v44, &*v3, &*v3].concat());
    let mut v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unix binary"); } LIT.with(|lit| lit.clone()) };
    std::fs::write(&*v70,&*v71).unwrap();
    let mut v72: Rc<str> = std::rc::Rc::<str>::from(refresh_cold_proof_census_gate(std::path::Path::new(&*v0)).unwrap_err());
    let mut v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exactly one root binary"); } LIT.with(|lit| lit.clone()) };
    let mut v74: bool = v72.contains(&*v73);
    if v74 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    let mut v75: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v63).unwrap());
    let mut v76: bool = v75 == v64 ;
    if v76 { () } else { std::panic::panic_any("Spiral test assertion failed") };
    0i32
}
fn closure5() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        method7(v0.clone())
    }); }
    CLOSURE.with(|closure| closure.clone())
}
fn method8(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 == v1;
    if v2 {
        1i32
    } else {
        0i32
    }
}
fn method9(mut v0: i32, mut v1: i32) -> i32 {
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
                let mut v5: i32 = v0 - 1i32;
                let mut v6: i32 = v5 / 2i32;
                let mut v7: i32 = v6 + 1i32;
                v7
            } else {
                let mut v8: bool = v1 == 4i32;
                if v8 {
                    let mut v9: i32 = v0 - 1i32;
                    let mut v10: i32 = v9 / 4i32;
                    let mut v11: i32 = v10 + 1i32;
                    v11
                } else {
                    -1i32
                }
            }
        }
    }
}
fn method11(mut v0: Rc<str>, mut v1: u64, mut v2: u64) -> bool {
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
                let mut v16: u64 = v2 + 1u64;
                (v0, v1, v2) = (v0.clone(), v1, v16);
                continue;
            } else {
                return false;
            }
        }
    }
}
fn method10(mut v0: Rc<str>) -> bool {
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
        method11(v0.clone(), v1, v5)
    } else {
        false
    }
}
fn closure6() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        method10(v0.clone())
    }); }
    CLOSURE.with(|closure| closure.clone())
}
fn closure7() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        method8(v0, v1)
    }); }
    CLOSURE.with(|closure| closure.clone())
}
fn method12(mut v0: i32) -> i32 {
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
        method12(v0)
    }); }
    CLOSURE.with(|closure| closure.clone())
}
fn closure9() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        method9(v0, v1)
    }); }
    CLOSURE.with(|closure| closure.clone())
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
