//! Evie Frye — `{1}{U}` 2/1 Legendary Human Assassin.
//!
//! * Partner with Jacob Frye (When this creature enters, target player
//!   may put Jacob into their hand from their library, then shuffle.)
//!   — Partner-with tutor with a "target player may" + named-card
//!   search; not faithfully expressible (no may-tutor for a chosen
//!   player). GAP'd.
//! * {1}, {T}: Draw a card, then discard a card. When you discard a
//!   creature card this way, target creature you control can't be
//!   blocked this turn.
//!
//! The activated ability's draw+discard is modeled. The reflexive
//! "when you discard a creature card this way, target creature can't be
//! blocked" rider has no inline expression, so it is GAP'd.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Evie Frye");
    let human = reg.interner_mut().intern("Human");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(assassin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: "Partner with Jacob Frye" ETB tutor (target player may search
    // for a named card) not faithfully expressible.

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}, {T}: Draw a card, then discard a card.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: loot,
        }),
    )
}

fn loot(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: reflexive "when you discard a creature card this way, target
    // creature you control can't be blocked this turn" rider not
    // expressible inline.
    vec![Effect::Sequence(vec![
        Effect::DrawCards {
            player: ctx.controller,
            count: 1,
        },
        Effect::Discard {
            player: ctx.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ])]
}
