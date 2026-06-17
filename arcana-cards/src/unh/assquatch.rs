//! Assquatch — `{4}{R}` 3½/3½ Donkey (Un-set; the half P/T is recorded as
//! the nearest integer, PtValue having no fractional variant).
//! "Each other Donkey gets +1½/+1½." (static anthem — GAP)
//! "Whenever another Donkey enters, untap target creature and gain control
//! of it until end of turn. That creature gains haste until end of turn."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Assquatch");
    let donkey = reg.interner_mut().intern("Donkey");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(donkey);

    // Trigger filter: another Donkey entering (the source excludes itself
    // because it is already on the battlefield by the time another enters).
    let donkey_filter = script::subtype_filter(reg, "Donkey");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // GAP: actual P/T is 3½/3½; PtValue has no half-point variant, so
        // the nearest integer is recorded.
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: static anthem "Each other Donkey gets +1½/+1½" — no static
    // ability hook in this card class (and no half-point pump).

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: donkey_filter,
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: untap_and_steal,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Creature,
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn untap_and_steal(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![
        Effect::Untap { target: *id },
        Effect::ChangeControlEot {
            target: *id,
            new_controller: trig.controller,
        },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        },
    ]
}
