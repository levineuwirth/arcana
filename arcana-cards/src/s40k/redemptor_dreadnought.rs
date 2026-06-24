//! Redemptor Dreadnought — `{5}` 4/4 Artifact Creature — Astartes Dreadnought.
//! Fallen Warrior — As an additional cost to cast this spell, you may exile a
//! creature card from your graveyard. Trample. Plasma Incinerator — Whenever
//! this creature attacks, if a card is exiled with it, it gets +X/+X until end
//! of turn, where X is the power of the exiled card.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Redemptor Dreadnought");
    let astartes = reg.interner_mut().intern("Astartes");
    let dreadnought = reg.interner_mut().intern("Dreadnought");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(astartes);
    subtypes.0.insert(dreadnought);

    // GAP: Fallen Warrior — "as an additional cost to cast this spell, you may
    // exile a creature card from your graveyard" is an optional ADDITIONAL CAST
    // cost that IMPRINTS the exiled card (read later as "exiled with it"). The
    // payment is an exile-from-graveyard with imprint — not a sacrifice/discard
    // — and there is no additional-cost / imprint surface, so it is not
    // expressible.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // Plasma Incinerator trigger condition (attacks) is expressible, but
            // its payload depends on the card exiled by Fallen Warrior, which is
            // not modeled — so the effect body is a GAP.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: plasma_incinerator,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn plasma_incinerator(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "+X/+X where X is the power of the exiled card" — there is no engine
    // hook to read the card exiled-with-it by the (unmodeled) Fallen Warrior
    // additional cost, so the conditional pump cannot be computed.
    Vec::new()
}
