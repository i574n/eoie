use command_spec_domain::{eoie_command_descriptor, eoie_command_proxy_usage};

#[test]
fn every_advertised_proxy_has_specific_usage() {
    let descriptor = eoie_command_descriptor("4");
    let catalog = descriptor.2.strip_prefix("proxy: ").expect("proxy descriptor");
    let names = catalog.trim_end_matches(" ...").split('|').collect::<Vec<_>>();
    assert!(names.len() > 50, "unexpectedly incomplete proxy catalog");
    for name in names {
        let usage = eoie_command_proxy_usage(name);
        let mut words = usage.split_whitespace();
        assert_eq!(words.next(), Some("proxy"), "missing usage for {name}");
        assert_eq!(words.next(), Some(name), "misaligned usage for {name}: {usage}");
    }
}

#[test]
fn source_inspection_help_describes_read_and_optional_write_forms() {
    assert_eq!(&*eoie_command_proxy_usage("source-stats"), "proxy source-stats <root>");
    assert_eq!(&*eoie_command_proxy_usage("source-topology"), "proxy source-topology <root> [state/authority_census.spi]");
    assert_eq!(&*eoie_command_proxy_usage("source-map"), "proxy source-map <root>");
    assert_eq!(&*eoie_command_proxy_usage("source-diff"), "proxy source-diff <left-root> <right-root>");
}

#[test]
fn unknown_capabilities_do_not_get_fabricated_usage() {
    for name in ["", "source-stat", "SOURCE-STATS", "source-stats extra", "not-a-capability"] {
        assert!(eoie_command_proxy_usage(name).is_empty(), "unexpected usage for {name}");
    }
}
