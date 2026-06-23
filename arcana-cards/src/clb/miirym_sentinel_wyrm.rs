//! Miirym, Sentinel Wyrm — `{3}{G}{U}{R}` 6/6 Legendary Creature —
//! Dragon Spirit.
//!
//! * Flying, ward {2} (keywords).
//! * "Whenever another nontoken Dragon you control enters, create a
//!   token that's a copy of it, except the token isn't legendary." —
//!   a ZoneChange (enters-the-battlefield) trigger on nontoken Dragons
//!   you control; resolves to `Effect::CopyPermanent` of the entering
//!   creature. (The "except the token isn't legendary" rider is a
//!   documented CopyPermanent fidelity gap — the copy retains the
//!   original's supertypes.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Miirym, Sentinel Wyrm");
    let dragon = reg.interner_mut().intern("Dragon");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    subtypes.0.insert(spirit);

    // "another nontoken Dragon you control enters" — exclude tokens and
    // restrict to your control. (The trigger is on OTHER Dragons; the token
    // it creates is a token, so the nontoken filter prevents re-triggering.)
    let dragon_filter = script::subtype_filter(reg, "Dragon")
        .controlled_by(ControllerConstraint::You)
        .nontoken();

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{U}{R}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: dragon_filter,
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: copy_entering_dragon,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn copy_entering_dragon(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.entering_object() else {
        return Vec::new();
    };
    vec![Effect::CopyPermanent { target: id }]
}
