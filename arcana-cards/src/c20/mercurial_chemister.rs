//! Mercurial Chemister — `{3}{U}{R}` 2/3 Human Wizard.
//!
//! Oracle:
//! * `{U}, {T}: Draw two cards.`
//! * `{R}, {T}, Discard a card: This creature deals damage to target creature
//!   equal to the discarded card's mana value.`
//!
//! The first activation is fully wired (mana + tap, draw two). The second's
//! cost is wired (mana + tap + discard-a-card), and it targets a creature, but
//! the damage AMOUNT is GAP'd: "equal to the discarded card's mana value" — the
//! mana value of the card discarded as a cost is not exposed to the resolver,
//! so no damage is dealt.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mercurial Chemister");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U}, {T}: Draw two cards.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R}, {T}, Discard a card: This creature deals damage to \
                       target creature equal to the discarded card's mana value."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}").expect("valid cost"),
                    tap: true,
                    discard_other: Some(ObjectFilter::default()),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: damage_equal_to_discard,
            }),
    )
}

fn draw_two(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: ctx.controller, count: 2 }]
}

fn damage_equal_to_discard(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: damage "equal to the discarded card's mana value" — the mana value
    // of the card discarded as a cost is not exposed to the resolver.
    Vec::new()
}
