//! Leery Fogbeast — `{2}{G}` 4/2 green Creature — Beast. "Whenever
//! this creature becomes blocked, prevent all combat damage that would
//! be dealt this turn."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Leery Fogbeast");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesBlocked,
                intervening_if: None,
                effect: prevent_all_combat_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn prevent_all_combat_damage(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Best-effort "prevent all combat damage this turn": board-wide
    // source-filtered prevention from any permanent. PreventDamageFrom
    // takes a single TargetFilter, so we cover damage to players here.
    // GAP: PreventDamageFrom has no combat_only flag (this also
    // prevents noncombat damage from permanents), and a single
    // TargetFilter can't simultaneously cover both players and
    // creatures — damage dealt to creatures is not prevented.
    vec![Effect::PreventDamageFrom {
        source_filter: ObjectFilter::permanent(),
        target_filter: TargetFilter::Player,
        amount: None,
        duration: ReplacementDuration::EndOfTurn,
    }]
}
