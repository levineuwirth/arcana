//! Rapacious One — `{5}{R}` 5/4 red Creature — Eldrazi Drone.
//! Trample.
//! Whenever this creature deals combat damage to a player, create that many
//! 0/1 colorless Eldrazi Spawn creature tokens. They have "Sacrifice this
//! token: Add {C}."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rapacious One");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let drone = reg.interner_mut().intern("Drone");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(drone);
    let self_filter = ObjectFilter { name: Some(name), ..ObjectFilter::default() };
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: self_filter,
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: combat_damage_make_spawn,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn combat_damage_make_spawn(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let n = trig.damage_amount().unwrap_or(0);
    let spawn = reg.interner().lookup("Eldrazi Spawn").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    if let Some(eldrazi) = reg.interner().lookup("Eldrazi") {
        subtypes.0.insert(eldrazi);
    }
    if let Some(s) = reg.interner().lookup("Spawn") {
        subtypes.0.insert(s);
    }
    // GAP: the token's intrinsic activated mana ability "Sacrifice this
    // token: Add {C}" cannot be expressed — TokenDefinition.abilities holds
    // only triggered abilities, not activated ones. The dynamic count of
    // 0/1 colorless Eldrazi Spawn tokens (= combat damage dealt) is faithful.
    let token = TokenDefinition {
        name: spawn,
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    (0..n)
        .map(|_| Effect::CreateToken {
            controller: trig.controller,
            token: token.clone(),
        })
        .collect()
}
