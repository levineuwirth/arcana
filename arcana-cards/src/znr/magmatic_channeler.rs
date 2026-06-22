//! Magmatic Channeler — `{1}{R}` 1/3 Human Wizard (red).
//!
//! Oracle:
//! * As long as there are four or more instant and/or sorcery cards in
//!   your graveyard, this creature gets +3/+1.
//! * {T}, Discard a card: Exile the top two cards of your library, then
//!   choose one of them. You may play that card this turn.
//!
//! GAP: the "+3/+1 while four or more instant/sorcery cards are in your
//! graveyard" is a continuous static, not a triggered/activated ability.
//!
//! Fidelity note: the activated ability is modeled with `ImpulseExile`
//! (exile the top two, you may play them until end of turn). The printed
//! card lets you play only ONE of the two; ImpulseExile permits playing
//! both — a documented over-grant, strictly closer than GAP'ing.

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::mana::ManaCost;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Magmatic Channeler");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Discard a card: Exile the top two cards of your library, \
                       then choose one of them. You may play that card this turn."
                    .into(),
                cost: ActivationCost {
                    tap: true,
                    discard_other: Some(ObjectFilter::default()),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: impulse_two,
            }),
    )
}

fn impulse_two(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ImpulseExile {
        player: ctx.controller,
        count: 2,
    }]
}
