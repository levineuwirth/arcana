//! A-Dokuchi Silencer — `{1}{B}` 2/1 Human Ninja.
//! Ninjutsu {1}{B}. "Whenever Dokuchi Silencer deals combat damage to a
//! player, you may discard a card. When you do, destroy target creature
//! or planeswalker that player controls."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::targets::TargetFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Dokuchi Silencer");
    let human = reg.interner_mut().intern("Human");
    let ninja = reg.interner_mut().intern("Ninja");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(ninja);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Ninjutsu — no KeywordAbility::Ninjutsu / alternate-cast cost shape available.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "Whenever ~ deals combat damage to a player, you may discard a
            // card. When you do, destroy target creature or planeswalker that
            // player controls."
            // The "destroy target creature/planeswalker that player controls"
            // is gated on the optional discard and references the damaged
            // player; the discard-then sub-trigger and player-restricted
            // target are not expressible together. We model the optional
            // discard as a best-effort partial.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: arcana_core::targets::ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: combat_damage_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn combat_damage_discard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "When you do, destroy target creature or planeswalker that player
    // controls" — the reflexive when-you-discard sub-trigger restricting the
    // destroy target to the damaged player's permanents is not expressible.
    vec![Effect::Discard {
        player: trig.controller,
        count: 1,
        choice: DiscardChoice::ControllerChooses,
    }]
}
