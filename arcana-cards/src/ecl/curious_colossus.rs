//! Curious Colossus — `{5}{W}{W}` 7/7 white creature. "When this creature
//! enters, each creature target opponent controls loses all abilities, becomes a
//! Coward in addition to its other types, and has base power and toughness 1/1."
//!
//! "Loses all abilities" is `Effect::LoseAllAbilities`, the 1/1 base
//! is `Effect::SetBasePT`, and "becomes a Coward in addition to its
//! other types" is a targeted subtype-add continuous effect
//! (`ContinuousEffect::add_subtypes`), all `Duration::Permanent` (the
//! oracle text has no duration).

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Curious Colossus");
    let giant = reg.interner_mut().intern("Giant");
    let warrior = reg.interner_mut().intern("Warrior");
    let _coward = reg.interner_mut().intern("Coward"); // looked up in etb_mass_debuff
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_mass_debuff,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_player()],
            }),
    )
}

fn etb_mass_debuff(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    use arcana_core::targets::TargetChoice;
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(opp) = target else {
        return Vec::new();
    };
    // "each creature target opponent controls" — from the targeted
    // opponent's perspective that is ControllerConstraint::You.
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        *opp,
    );
    let mut subs = SubtypeSet::default();
    if let Some(coward) = reg.interner().lookup("Coward") {
        subs.0.insert(coward);
    }
    let source = trig.source;
    ids.into_iter()
        .flat_map(|id| {
            [
                Effect::LoseAllAbilities {
                    target: id,
                    duration: Duration::Permanent,
                },
                // "…becomes a Coward in addition to its other types"
                Effect::InstallContinuousEffect {
                    effect: ContinuousEffect::add_subtypes(
                        source,
                        id,
                        subs.clone(),
                        Duration::Permanent,
                    ),
                },
                Effect::SetBasePT {
                    target: id,
                    power: 1,
                    toughness: 1,
                    duration: Duration::Permanent,
                },
            ]
        })
        .collect()
}
