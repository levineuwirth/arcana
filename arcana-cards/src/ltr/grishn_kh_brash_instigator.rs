//! Grishnákh, Brash Instigator — `{2}{R}` 1/1 Legendary Creature —
//! Goblin Soldier. "When Grishnákh enters, amass Orcs 2. When you do,
//! until end of turn, gain control of target nonlegendary creature an
//! opponent controls with power less than or equal to the amassed Army's
//! power. Untap that creature. It gains haste until end of turn."
//!
//! The ETB trigger amasses Orcs 2 (`Effect::Amass`) and then performs the
//! reflexive Threaten suite on the target — gain control until end of
//! turn, untap, grant haste. PARTIAL: the "power <= the amassed Army's
//! power" restriction on the target is dynamic (depends on the Army's
//! current power) and is not expressible in the static target filter, so
//! the target is restricted only to a nonlegendary creature an opponent
//! controls.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Grishnákh, Brash Instigator");
    let goblin = reg.interner_mut().intern("Goblin");
    let soldier = reg.interner_mut().intern("Soldier");
    // Interned at register so the effect fn can look them up at resolve.
    let _army = reg.interner_mut().intern("Army");
    let _orc = reg.interner_mut().intern("Orc");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: amass_and_seize,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::Opponent)
                            .without_supertypes(
                                SupertypeSet::new()
                                    .with(SupertypeSet::LEGENDARY),
                            ),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn amass_and_seize(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let army_subtype = reg.interner().lookup("Army").unwrap_or_default();
    let race_subtype = reg.interner().lookup("Orc").unwrap_or_default();
    let mut effects = vec![Effect::Amass {
        controller: trig.controller,
        count: 2,
        army_subtype,
        race_subtype,
    }];
    // "When you do, ... gain control of target ... creature." PARTIAL:
    // the "power <= amassed Army's power" gate is not expressible in the
    // target filter.
    if let Some(TargetChoice::Object(id)) = trig.targets.targets.first() {
        effects.push(Effect::ChangeControlEot {
            target: *id,
            new_controller: trig.controller,
        });
        effects.push(Effect::Untap { target: *id });
        effects.push(Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        });
    }
    effects
}
