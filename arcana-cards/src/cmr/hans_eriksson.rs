//! Hans Eriksson — `{2}{R}{G}` 1/4 Legendary Human Scout.
//! "Whenever Hans Eriksson attacks, reveal the top card of your library.
//! If it's a creature card, put it onto the battlefield tapped and attacking
//! defending player or a planeswalker they control. Otherwise, put that card
//! into your hand. When you put a creature card onto the battlefield this way,
//! it fights Hans Eriksson."
//!
//! GAP: The conditional reveal (creature → battlefield tapped+attacking,
//! non-creature → hand) combined with a secondary fight trigger is not fully
//! expressible. Effect::DigTopN handles the "look at top card, take it or
//! not" shape but cannot conditionally deploy a creature to the battlefield
//! tapped and attacking, nor trigger a fight on ETB. Emitting DigTopN
//! (creature filter, hand destination) as a best-effort approximation;
//! the attacking-and-fighting rider is omitted.

use arcana_core::effects::{DigRest, Effect};
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
    let name = reg.interner_mut().intern("Hans Eriksson");
    let human = reg.interner_mut().intern("Human");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(scout);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attacks_reveal_top,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attacks_reveal_top(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: cannot conditionally put a creature onto the battlefield tapped
    // and attacking, nor fire a fight sub-trigger on that ETB.
    // Best-effort: dig the top card, take it if it's a creature, else bottom.
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 1,
        filter: Some(ObjectFilter {
            types: Some(TypeLine::CREATURE.into()),
            ..ObjectFilter::default()
        }),
        rest: DigRest::BottomRandom,
    }]
}
