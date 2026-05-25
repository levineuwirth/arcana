//! Mindstab Thrull — `{1}{B}{B}` 2/2 black Thrull.
//! "Whenever this creature attacks and isn't blocked, you may sacrifice it. If you do,
//! defending player discards three cards."

use arcana_core::effects::{Effect, DiscardChoice};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mindstab Thrull");
    let thrull = reg.interner_mut().intern("Thrull");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(thrull);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacksUnblocked,
                intervening_if: None,
                effect: on_unblocked,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_unblocked(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "defending player discards three cards" — use defending_player accessor.
    let Some(p) = trig.defending_player() else { return Vec::new(); };
    vec![Effect::Discard {
        player: p,
        count: 3,
        choice: DiscardChoice::ControllerChooses,
    }]
}
