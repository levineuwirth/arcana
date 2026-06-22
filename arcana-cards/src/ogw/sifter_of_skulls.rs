//! Sifter of Skulls — `{3}{B}` 4/3 Eldrazi (colorless / Devoid).
//! Devoid (this card has no color).
//! Whenever another nontoken creature you control dies, create a 1/1 colorless
//! Eldrazi Scion creature token. It has "Sacrifice this token: Add {C}."
//!
//! Devoid makes the card colorless (handled by `colors: ColorSet::colorless()`).
//! The dies-trigger mints a 1/1 colorless Eldrazi Scion. The "another"
//! self-exclusion is a minor fidelity gap (ObjectFilter has no self-id
//! predicate), and the token's intrinsic "Sacrifice this token: Add {C}"
//! activated ability is not wired (token minted plainly) — both GAP'd.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sifter of Skulls");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let _scion = reg.interner_mut().intern("Scion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP (fidelity): "another" self-exclusion not expressible (no self-id predicate on ObjectFilter).
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You)
                    .nontoken(),
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard(0),
            },
            intervening_if: None,
            effect: make_scion,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn make_scion(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let eldrazi = reg.interner().lookup("Eldrazi").unwrap_or_default();
    let scion = reg.interner().lookup("Scion").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(scion);
    // GAP: token's intrinsic "Sacrifice this token: Add {C}" ability isn't wired.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: scion,
            colors: ColorSet::colorless(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
