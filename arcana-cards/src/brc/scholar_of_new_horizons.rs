//! Scholar of New Horizons — `{1}{W}` 1/1 Human Scout.
//! Enters with a +1/+1 counter on it. (enters-with replacement — GAP)
//! `{T}, Remove a counter from a permanent you control: Search your library
//! for a Plains card and reveal it. ... put it into your hand. Then shuffle.`

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scholar of New Horizons");
    let human = reg.interner_mut().intern("Human");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(scout);

    // GAP: "This creature enters with a +1/+1 counter on it" is an as-enters
    // replacement, not a triggered ability — not expressible here.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}, Remove a counter from a permanent you control: Search your library for a Plains card and reveal it. ... put it into your hand. Then shuffle.".into(),
            // GAP: "Remove a counter from a permanent you control" is a chosen-
            // other-permanent counter-removal cost; only remove_self_counter is
            // expressible, so only the {T} portion of the cost is modeled.
            cost: ActivationCost {
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: tutor_plains,
        }),
    )
}

fn tutor_plains(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    // GAP: the conditional "if an opponent controls more lands than you, you may
    // put that card onto the battlefield tapped, otherwise into hand" branch is
    // not expressible; model the default — search for a Plains card to hand.
    vec![Effect::TutorToHand {
        player: ctx.controller,
        filter: script::subtype_filter(reg, "Plains"),
        reveal: true,
    }]
}
