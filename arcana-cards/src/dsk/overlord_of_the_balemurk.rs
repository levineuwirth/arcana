//! Overlord of the Balemurk — `{3}{B}{B}` 5/5 Enchantment Creature —
//! Avatar Horror. (Impending alt-cost keyword not in supported
//! surface — GAP'd.)
//! "Whenever this permanent enters or attacks, mill four cards, then
//! you may return a non-Avatar creature card or a planeswalker card
//! from your graveyard to your hand."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Overlord of the Balemurk");
    let avatar = reg.interner_mut().intern("Avatar");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    // GAP: "Impending 5—{1}{B}" — Impending alternate-cost keyword not supported.
    // Two triggers (enters / attacks) share the mill-then-return body.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: mill_then_return,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![return_target()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: mill_then_return,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![return_target()],
            }),
    )
}

fn return_target() -> TargetRequirement {
    // GAP: "non-Avatar" exclusion not applied — no subtype-exclusion predicate in the supported surface;
    //      filter is "creature or planeswalker card in graveyard". "up to one" models the optional "you may".
    TargetRequirement {
        filter: TargetFilter::Card {
            zone: Zone::Graveyard(0),
            filter: ObjectFilter::new()
                .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::PLANESWALKER)),
        },
        count: TargetCount::UpTo(1),
        controller: None,
    }
}

fn mill_then_return(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = vec![Effect::Mill {
        player: trig.controller,
        count: 4,
    }];
    if let Some(TargetChoice::Object(id)) = trig.targets.targets.first() {
        effects.push(Effect::ReturnFromGraveyardToHand { target: *id });
    }
    effects
}
