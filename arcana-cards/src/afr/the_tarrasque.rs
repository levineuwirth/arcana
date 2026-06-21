//! The Tarrasque — `{6}{G}{G}{G}` 10/10 Legendary Dinosaur.
//! The Tarrasque has haste and ward {10} as long as it was cast. (GAP:
//! conditional "as long as it was cast" static — not expressible.)
//! Whenever The Tarrasque attacks, it fights target creature defending
//! player controls.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Tarrasque");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);

    // GAP: "has haste and ward {10} as long as it was cast" — a cast-gated
    // conditional static; not expressible. (Scryfall "Fight" tag is not a
    // KeywordAbility variant.)
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(10)),
        toughness: Some(PtValue::Fixed(10)),
        ..Default::default()
    };

    // "defending player controls" restriction not expressible; targets
    // any creature (partial).
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: attack_fight,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::target_creature()],
        }),
    )
}

fn attack_fight(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::Fight {
        a: trig.source,
        b: *id,
    }]
}
