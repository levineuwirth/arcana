//! Lord Xander, the Collector — `{4}{U}{B}{R}` 6/6 Legendary Vampire Demon Noble.
//! When Lord Xander enters, target opponent discards half the cards in their
//! hand, rounded down.
//! Whenever Lord Xander attacks, defending player mills half their library,
//! rounded down.
//! When Lord Xander dies, target opponent sacrifices half the nonland
//! permanents they control of their choice, rounded down.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lord Xander, the Collector");
    let vampire = reg.interner_mut().intern("Vampire");
    let demon = reg.interner_mut().intern("Demon");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(demon);
    subtypes.0.insert(noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_discard_half,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_player()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attacks_mill_half,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_sacrifice_half,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_player()],
            }),
    )
}

fn etb_discard_half(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let n = script::hand_size(state, *p) / 2;
    vec![Effect::Discard {
        player: *p,
        count: n,
        choice: DiscardChoice::ControllerChooses,
    }]
}

fn attacks_mill_half(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(p) = trig.defending_player() else {
        return Vec::new();
    };
    let n = script::library_size(state, p) / 2;
    vec![Effect::Mill { player: p, count: n }]
}

fn dies_sacrifice_half(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    // From `p`'s own perspective, count the nonland permanents they control.
    let filter = ObjectFilter::permanent()
        .without_types(TypeLine::LAND.into())
        .controlled_by(ControllerConstraint::You);
    let n = script::count_matching(state, &filter, *p) / 2;
    vec![Effect::Sacrifice {
        player: *p,
        filter,
        count: n,
    }]
}
