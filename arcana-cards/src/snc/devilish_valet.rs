//! Devilish Valet — `{2}{R}` 1/3 Creature — Devil Warrior. Red.
//!
//! Oracle text:
//! * Trample, haste
//! * Alliance — Whenever another creature you control enters, double this
//!   creature's power until end of turn.
//!
//! Decomposition:
//! * Keyword line → `keywords: vec![KeywordAbility::Trample,
//!   KeywordAbility::Haste]` (Alliance is an ability word, not a keyword).
//! * Alliance trigger → one `TriggeredAbilityDef` (ZoneChange: a creature
//!   you control entering). "Double this creature's power until end of turn"
//!   = pump this creature by its CURRENT power (so total = 2×) via
//!   `script::power_of` + `Effect::Pump`. (Following the catalog convention
//!   for "another creature you control enters" — the plain
//!   you-control-creature ZoneChange filter; ZoneChange carries no
//!   TriggerSelf self-exclusion field.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Devilish Valet");
    let devil = reg.interner_mut().intern("Devil");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(devil);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: double_power,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn double_power(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Doubling = add the creature's current power to itself this turn.
    let current = script::power_of(state, trig.source);
    vec![Effect::Pump {
        target: trig.source,
        power: current,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
