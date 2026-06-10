//! Hans Eriksson — `{2}{R}{G}` 1/4 Legendary Human Scout.
//! "Whenever Hans Eriksson attacks, reveal the top card of your library.
//! If it's a creature card, put it onto the battlefield tapped and attacking
//! defending player or a planeswalker they control. Otherwise, put that card
//! into your hand. When you put a creature card onto the battlefield this way,
//! it fights Hans Eriksson."
//!
//! Wired: the trigger fn peeks the top card of the library at resolution.
//! If it's a creature card it's put onto the battlefield tapped and attacking
//! (Effect::PutOntoBattlefieldTappedAttacking); otherwise it goes to hand
//! (Effect::ReturnToHand). The reveal-then-route is mandatory on the card, so
//! the deterministic peek is faithful.
//! GAP: the reflexive "it fights Hans Eriksson" trigger is omitted — the put
//! re-ids the card per CR 400.7, so the new battlefield id isn't visible to
//! card code for a fight effect.

use arcana_core::effects::Effect;
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
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Reveal the top card at resolution: creature → battlefield tapped and
    // attacking; otherwise → hand. GAP: the "it fights Hans Eriksson"
    // reflexive trigger is omitted (see module doc).
    let you = trig.controller;
    if you >= state.num_players() {
        return Vec::new();
    }
    let Some(&top) = state.player(you).library_top_to_bottom.first() else {
        return Vec::new();
    };
    let is_creature = state.objects.get(top).is_some_and(|o| o.is_creature());
    if is_creature {
        vec![Effect::PutOntoBattlefieldTappedAttacking {
            target: top,
            controller: you,
        }]
    } else {
        vec![Effect::ReturnToHand { target: top }]
    }
}
