//! Skanos, Green Dragon Vassal — `{4}{G}{G}` 6/6 Legendary Dragon Ranger with
//! Vigilance. "Whenever Skanos attacks, untap another target attacking creature. It
//! gets +X/+0 until end of turn, where X is Skanos's power." Target restriction to
//! "another attacking creature" is approximated by target creature (the attacking /
//! another-than-self restriction is not expressible in the listed TargetFilter set).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skanos, Green Dragon Vassal");
    let dragon = reg.interner_mut().intern("Dragon");
    let ranger = reg.interner_mut().intern("Ranger");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    subtypes.0.insert(ranger);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: untap_and_pump,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            // GAP: "another" / "attacking" restriction — approximated as target creature.
            target_requirements: vec![TargetRequirement::target_creature()],
        }),
    )
}

fn untap_and_pump(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let x = script::power_of(state, trig.source).max(0);
    vec![
        Effect::Untap { target: *id },
        Effect::Pump {
            target: *id,
            power: x,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
    ]
}
