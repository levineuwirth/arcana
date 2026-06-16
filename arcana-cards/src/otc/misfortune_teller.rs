//! Misfortune Teller — `{3}{B}` 3/1 Human Warlock with Deathtouch.
//! "Whenever this creature enters or deals combat damage to a player, exile
//!  target card from a graveyard. If it was a creature card, create a 2/2
//!  black Rogue creature token. If it was a land card, create a Treasure
//!  token. Otherwise, you gain 3 life."
//!
//! The trigger fires on two events (enters / deals combat damage to a
//! player), wired as two TriggeredAbilityDefs. Each exiles the targeted
//! graveyard card. The type-conditional payoff (creature → Rogue token /
//! land → Treasure / else gain 3 life) branches on the exiled card's type
//! after exile, which is not expressible with the documented primitives —
//! GAP'd; only the exile is performed.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Misfortune Teller");
    let human = reg.interner_mut().intern("Human");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warlock);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };
    let target = || TargetRequirement {
        filter: TargetFilter::Card {
            zone: Zone::Graveyard(0),
            filter: ObjectFilter::default(),
        },
        count: TargetCount::Exactly(1),
        controller: None,
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: exile_target_graveyard_card,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![target()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::default(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: exile_target_graveyard_card,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![target()],
            }),
    )
}

fn exile_target_graveyard_card(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: type-conditional payoff (creature → 2/2 Rogue / land → Treasure /
    // else gain 3 life) branches on the exiled card's type post-exile, not
    // expressible with the documented primitives. Only the exile is wired.
    vec![Effect::ExileFromGraveyard { target: *id }]
}
