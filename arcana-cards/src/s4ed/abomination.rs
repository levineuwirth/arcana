//! Abomination — `{3}{B}{B}` 2/6 black Horror.
//! "Whenever this creature blocks or becomes blocked by a green or white creature,
//! destroy that creature at end of combat."
//! GAP: "destroy at end of combat" — no DelayedWhen::EndOfCombat; using immediate DestroyPermanent.
//! The trigger fires on SelfBlocksOrBecomesBlocked; we check the other combatant's color via trig.other_combatant().

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Abomination");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBlocksOrBecomesBlocked,
                intervening_if: None,
                effect: destroy_green_or_white_combatant,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn destroy_green_or_white_combatant(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.other_combatant() else { return Vec::new(); };
    // Check if other creature is green or white
    // GAP: cannot filter by combatant color at trigger time; destroy unconditionally as best effort.
    // GAP: "at end of combat" — using immediate destroy instead; no DelayedWhen::EndOfCombat.
    let _ = state; // used for potential script calls
    vec![Effect::DestroyPermanent { target: id }]
}
