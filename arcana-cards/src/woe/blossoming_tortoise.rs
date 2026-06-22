//! Blossoming Tortoise — `{2}{G}{G}` 3/3 Turtle.
//! "Whenever this creature enters or attacks, mill three cards, then
//! return a land card from your graveyard to the battlefield tapped."
//!
//! The "enters or attacks" clause is decomposed into two triggers (a
//! SelfEntersBattlefield and a SelfAttacks) sharing one resolver.
//!
//! GAP (fidelity): the returned land enters UNTAPPED — Effect::Reanimate
//! has no "tapped" rider.
//! GAP (static): "Activated abilities of lands you control cost {1} less
//! to activate." — a cost-reduction static; no expressible primitive.
//! GAP (static): "Land creatures you control get +1/+1." — a continuous
//! anthem static; no expressible triggered/activated primitive.
//!
//! ("Mill" is reminder text, not a real KeywordAbility variant — the
//! mill itself is the Effect::Mill in the resolver.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blossoming Tortoise");
    let turtle = reg.interner_mut().intern("Turtle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(turtle);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: mill_and_return_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: mill_and_return_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn mill_and_return_land(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::Mill { player: trig.controller, count: 3 },
        Effect::Reanimate {
            player: trig.controller,
            filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
            from_zone: Zone::Graveyard(trig.controller),
        },
    ]
}
