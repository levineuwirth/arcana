//! Uro, Titan of Nature's Wrath — `{1}{G}{U}` 6/6 Legendary Creature —
//! Elder Giant.
//! When Uro enters, sacrifice it unless it escaped.
//! Whenever Uro enters or attacks, you gain 3 life and draw a card, then
//!   you may put a land card from your hand onto the battlefield.
//! Escape—{G}{G}{U}{U}, Exile five other cards from your graveyard.
//!
//! GAP: Escape is not an expressible KeywordAbility, and the escape cast
//! (cast-from-graveyard alternative cost) is not a creature activated
//! ability — it is dropped.
//! GAP: "sacrifice it unless it escaped" cannot be gated on the escaped
//! state, so that ETB trigger's effect is empty (firing the sacrifice
//! unconditionally would be a materially wrong card).

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
    let name = reg.interner_mut().intern("Uro, Titan of Nature's Wrath");
    let elder = reg.interner_mut().intern("Elder");
    let giant = reg.interner_mut().intern("Giant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elder);
    subtypes.0.insert(giant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        // GAP: Escape is not an expressible KeywordAbility.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: sacrifice_unless_escaped,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: gain_draw_ramp,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: gain_draw_ramp,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn sacrifice_unless_escaped(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "sacrifice it unless it escaped" — no way to read the escaped
    // state, so neither branch is emitted.
    Vec::new()
}

fn gain_draw_ramp(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Sequence(vec![
        Effect::GainLife { player: trig.controller, amount: 3 },
        Effect::DrawCards { player: trig.controller, count: 1 },
        Effect::PutFromHandOntoBattlefield {
            player: trig.controller,
            filter: ObjectFilter {
                types: Some(TypeLine::LAND.into()),
                ..ObjectFilter::default()
            },
            tapped: false,
        },
    ])]
}
