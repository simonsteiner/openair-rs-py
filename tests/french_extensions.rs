//! French OpenAir extensions (FFVL, FFVP, ZSM, NOTAM, ...).
//!
//! These `AC` tokens are documented in
//! <https://github.com/BPascal-91/eAirspacesFormats/tree/master/openair>.
//! They are not OpenAir v2 classes, so they must survive as
//! `Class::Unknown` with the original token.

use indoc::indoc;
use openair::*;

fn parse_one(data: &str) -> Airspace {
    let mut spaces = parse(data.as_bytes())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(spaces.len(), 1);
    spaces.pop().unwrap()
}

#[test]
fn extended_class_tokens_are_preserved() {
    for token in [
        "NOTAM",
        "NOTAM ref",
        "ZSM",
        "FFVL",
        "FFVP",
        "SIV",
        "RAS",
        "ADIZ",
        "AMA",
        "PART",
        "FIR",
        "UIR",
        "OCA",
        "POLITICAL",
        "NO-FIR",
        "OTHER",
    ] {
        let class = Class::parse(token).unwrap();
        assert_eq!(class, Class::Unknown(token.into()));
        assert_eq!(class.as_str(), token);
    }
}

#[test]
fn ffvl_mundolsheim() {
    let space = parse_one(indoc! {"
        * en: (c) FFVL 14/02/2007 - Activité de vol libre de Mundolsheim.
        AC FFVL
        AN FFVL-Prot Vol libre Mundolsheim (PARAGLIDER) (LFFFVLMundolsheim)
        AH 500ft AGL
        AL GND
        V X=48:38:00.00 N 007:42:34.00 E
        DC 1.0
    "});
    assert_eq!(space.class, Class::Unknown("FFVL".into()));
    assert_eq!(
        space.name.as_deref(),
        Some("FFVL-Prot Vol libre Mundolsheim (PARAGLIDER) (LFFFVLMundolsheim)")
    );
    assert_eq!(space.upper_bound, Altitude::FeetAgl(500));
    assert_eq!(space.lower_bound, Altitude::Gnd);
    let Geometry::Circle {
        centerpoint,
        radius,
    } = space.geom
    else {
        panic!("Expected circle geometry");
    };
    assert!((centerpoint.lat - 48.633_333_333).abs() < 1e-8);
    assert!((centerpoint.lng - 7.709_444_444).abs() < 1e-8);
    assert!((radius - 1.0).abs() < 1e-6);
}

#[test]
fn zsm_with_comment_metadata() {
    let space = parse_one(indoc! {r#"
        AC ZSM
        AY PROTECT
        AN PROTECT 2827 Gypaete barbu - Zone Tampon 300m/sol (BIRD)
        *AUID GUId=LFZSMDSTAC2827 UId=28 Id=LFZSMDSTAC2827
        *AAlt ["SFC/985FT AGL", "0m/2795m"]
        *ATimes {"1": ["UTC(01/01->31/08)", "ANY(00:00->23:59)"]}
        AH 985FT AGL
        AL SFC
        DP 45:20:35 N 007:01:35 E
        DP 45:20:53 N 007:01:45 E
        DP 45:20:59 N 007:01:51 E
        DP 45:20:35 N 007:01:35 E
    "#});
    assert_eq!(space.class, Class::Unknown("ZSM".into()));
    assert_eq!(space.type_, Some(AirspaceType::Unknown("PROTECT".into())));
    assert_eq!(space.upper_bound, Altitude::FeetAgl(985));
    assert_eq!(space.lower_bound, Altitude::Gnd);
    let Geometry::Polygon { segments } = space.geom else {
        panic!("Expected polygon geometry");
    };
    assert_eq!(segments.len(), 4);
}

#[test]
fn ffvp_with_arc_and_frequency() {
    let space = parse_one(indoc! {"
        AC FFVP
        AY FFVP-Prot
        AN FFVP-Prot RMZ ECHO 2 App(122.550 puis 122.500) (GLIDER)
        AF 122.550 puis 122.500
        AH 4000FT AMSL
        AL 3300.0FT AMSL
        DP 45:38:16 N 000:02:40 E
        DP 45:48:38 N 000:08:00 E
        V X=45:49:26 N 000:01:58 W
        V D=+
        DB 45:48:38 N 000:08:00 E, 45:45:41 N 000:06:29 E
        DP 45:38:16 N 000:02:40 E
    "});
    assert_eq!(space.class, Class::Unknown("FFVP".into()));
    assert_eq!(space.frequency.as_deref(), Some("122.550 puis 122.500"));
    assert_eq!(space.upper_bound, Altitude::FeetAmsl(4000));
    assert_eq!(space.lower_bound, Altitude::FeetAmsl(3300));
    let Geometry::Polygon { segments } = space.geom else {
        panic!("Expected polygon geometry");
    };
    assert!(segments.iter().any(|s| matches!(s, PolygonSegment::Arc(_))));
}
