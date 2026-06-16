//! Rakdos the Defiler — `{2}{B}{B}{R}{R}` 7/6 Legendary Demon with Flying
//! and Trample.
//! Whenever Rakdos attacks, sacrifice half the non-Demon permanents you
//! control, rounded up. Whenever Rakdos deals combat damage to a player,
//! that player sacrifices half the non-Demon permanents they control,
//! rounded up.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rakdos the Defiler");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}{R}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_sacrifice_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::default(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: combat_damage_sacrifice_them,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attack_sacrifice_self(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "non-Demon" restriction — ObjectFilter has no without_subtype
    // refinement; counts/sacrifices all your permanents.
    let filter = ObjectFilter::permanent().controlled_by(ControllerConstraint::You);
    let total = script::count_matching(state, &filter, trig.controller);
    let half = (total + 1) / 2;
    if half == 0 {
        return Vec::new();
    }
    vec![Effect::Sacrifice {
        player: trig.controller,
        filter,
        count: half,
    }]
}

fn combat_damage_sacrifice_them(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(player) = trig.damaged_player() else {
        return Vec::new();
    };
    // GAP: "non-Demon" restriction — no without_subtype refinement; counts
    // and sacrifices all of that player's permanents.
    let filter = ObjectFilter::permanent().controlled_by(ControllerConstraint::You);
    let total = script::count_matching(state, &filter, player);
    let half = (total + 1) / 2;
    if half == 0 {
        return Vec::new();
    }
    vec![Effect::Sacrifice {
        player,
        filter,
        count: half,
    }]
}
