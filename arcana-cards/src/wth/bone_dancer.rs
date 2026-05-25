//! Bone Dancer — `{1}{B}{B}` 2/2 black Zombie. "Whenever this creature
//! attacks and isn't blocked, you may put the top creature card of defending
//! player's graveyard onto the battlefield under your control. If you do,
//! this creature assigns no combat damage this turn."
//!
//! GAP: SelfAttacksUnblocked triggers but the effect (put top creature card
//! of defending player's graveyard onto battlefield) has no direct catalog
//! match. Using Reanimate as closest proxy; "this assigns no combat damage"
//! not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bone Dancer");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
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
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "put top creature card of defending player's graveyard onto
    // battlefield" — Reanimate requires filter from a specific player's
    // graveyard; defending player is from trig.defending_player().
    // GAP: "assigns no combat damage" not expressible.
    let Some(p) = trig.defending_player() else { return Vec::new(); };
    vec![Effect::Reanimate {
        player: trig.controller,
        filter: ObjectFilter::creature(),
        from_zone: Zone::Graveyard(p),
    }]
}
