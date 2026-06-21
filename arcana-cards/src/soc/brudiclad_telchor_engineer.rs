//! Brudiclad, Telchor Engineer — `{4}{U}{R}` 4/4 Legendary Artifact Creature
//! — Phyrexian Artificer.
//! Creature tokens you control have haste.
//! At the beginning of combat on your turn, create a 2/1 blue Phyrexian Myr
//! artifact creature token. Then you may choose a token you control. If you do,
//! each other token you control becomes a copy of that token.
//!
//! The "creature tokens you control have haste" line is a static continuous
//! ability with no triggered/activated form → GAP. The combat trigger creates
//! the 2/1 blue Phyrexian Myr artifact creature token; the "each other token
//! becomes a copy of a chosen token" rider has no make-an-existing-object-
//! become-a-copy primitive (CopyPermanent only mints a token), so it is GAPped.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brudiclad, Telchor Engineer");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let artificer = reg.interner_mut().intern("Artificer");
    let _myr = reg.interner_mut().intern("Myr");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(artificer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    // GAP: "Creature tokens you control have haste" — static continuous
    // ability, not a triggered/activated ability.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::PhaseBegins {
                phase: Phase::Combat,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: make_myr,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn make_myr(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let myr = reg.interner().lookup("Myr").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(myr);
    let phyrexian = reg.interner().lookup("Phyrexian").unwrap_or_default();
    subtypes.0.insert(phyrexian);
    // GAP: "Then you may choose a token you control. If you do, each other token
    // you control becomes a copy of that token." — no become-a-copy primitive.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: myr,
            colors: ColorSet::blue(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
